use jsonrpc_core::{
    futures::future::{self, Future},
    futures::{stream, Stream},
};

use sp_core::{hashing::twox_128_into, storage::StorageKey};

use frame_support::{ReversibleStorageHasher, StorageHasher};

use codec::{Decode, Encode};

use std::convert::TryFrom;

pub mod error;
pub use error::*;

pub type HashOf<T> = <T as sp_runtime::traits::Block>::Hash;
pub type FutureResult<T> = Box<dyn Future<Item = T, Error = RpcError> + Send>;

pub fn prefix(pallet: &[u8], storage: &[u8]) -> Vec<u8> {
    let mut prefix = Vec::new();
    prefix.resize(32, 0u8);

    twox_128_into(pallet, <&mut [u8; 16]>::try_from(&mut prefix[..16]).unwrap());
    twox_128_into(storage, <&mut [u8; 16]>::try_from(&mut prefix[16..]).unwrap());

    prefix
}

pub fn chain_key_hash_map<Key: Encode, Hasher: StorageHasher>(
    prefix: &[u8],
    key: &Key,
) -> StorageKey {
    StorageKey(
        prefix
            .iter()
            .chain(key.using_encoded(Hasher::hash).as_ref())
            .map(|b| *b)
            .collect(),
    )
}

pub fn key_hash_map<Key: Encode, Hasher: StorageHasher>(
    pallet: &[u8],
    map: &[u8],
    key: &Key,
) -> StorageKey {
    chain_key_hash_map::<_, Hasher>(prefix(pallet, map).as_ref(), key)
}

pub fn chain_key_hash_double_map<KeyFirst, KeySecond, HasherFirst, HasherSecond>(
    prefix: &[u8],
    key_first: &KeyFirst,
    key_second: &KeySecond,
) -> StorageKey
where
    KeyFirst: Encode,
    KeySecond: Encode,
    HasherFirst: StorageHasher,
    HasherSecond: StorageHasher,
{
    StorageKey(
        prefix
            .iter()
            .chain(key_first.using_encoded(HasherFirst::hash).as_ref())
            .chain(key_second.using_encoded(HasherSecond::hash).as_ref())
            .map(|b| *b)
            .collect(),
    )
}

pub fn key_hash_double_map<KeyFirst, KeySecond, HasherFirst, HasherSecond>(
    pallet: &[u8],
    map: &[u8],
    key_first: &KeyFirst,
    key_second: &KeySecond,
) -> StorageKey
where
    KeyFirst: Encode,
    KeySecond: Encode,
    HasherFirst: StorageHasher,
    HasherSecond: StorageHasher,
{
    chain_key_hash_double_map::<_, _, HasherFirst, HasherSecond>(
        prefix(pallet, map).as_ref(),
        key_first,
        key_second,
    )
}

fn get_value<R, State, Hash>(
    state: &State,
    key: StorageKey,
    at: Option<Hash>,
) -> FutureResult<Option<R>>
where
    R: 'static + Decode + GetError + Send,
    State: sc_rpc_api::state::StateApi<Hash>,
    Hash: Copy,
{
    Box::new(
        state
            .storage(key, at)
            .map_err(|e| to_rpc_error(Error::ScRpcApiError, Some(format!("{:?}", e))))
            .and_then(|d| match d {
                None => future::ok(None),
                Some(data) => match R::decode(&mut &data.0[..]) {
                    Err(_) => {
                        future::err(to_rpc_error(R::get_error(), Some(format!("{:?}", data))))
                    }
                    Ok(decoded) => future::ok(Some(decoded)),
                },
            }),
    )
}

pub struct StorageMap<Hasher>(std::marker::PhantomData<Hasher>);

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListResult<Key, Value> {
    key: Key,
    value: Value,
}

impl<Hasher: StorageHasher + ReversibleStorageHasher> StorageMap<Hasher> {
    pub fn get_value<R, State, Key, BlockHash>(
        state: &State,
        pallet: &[u8],
        map: &[u8],
        key: &Key,
        at: Option<BlockHash>,
    ) -> FutureResult<Option<R>>
    where
        R: 'static + Decode + GetError + Send,
        State: sc_rpc_api::state::StateApi<BlockHash>,
        Key: Encode,
        BlockHash: Copy,
    {
        get_value(state, key_hash_map::<_, Hasher>(pallet, map, key), at)
    }

    pub fn get_list<Value, State, BlockHash, Key>(
        state: &State,
        pallet: &[u8],
        map: &[u8],
        at: Option<BlockHash>,
        count: u32,
        start_id: Option<Key>,
    ) -> FutureResult<Vec<ListResult<Key, Value>>>
    where
        Value: 'static + Decode + GetError + Send,
        State: sc_rpc_api::state::StateApi<BlockHash>,
        BlockHash: Copy,
        Key: 'static + Encode + Decode + GetError + Send,
    {
        let prefix = prefix(pallet, map);
        let start_key = start_id.map(|id| chain_key_hash_map::<_, Hasher>(&prefix, &id));

        let keys = match state
            .storage_keys_paged(Some(StorageKey(prefix)), count, start_key, at)
            .wait()
        {
            Ok(k) => k,
            Err(e) => {
                return Box::new(future::err(to_rpc_error(
                    Error::ScRpcApiError,
                    Some(format!("{:?}", e)),
                )))
            }
        };
        if keys.is_empty() {
            return Box::new(future::ok(vec![]));
        }

        let key_futures: Vec<_> = keys
            .into_iter()
            .map(|k| {
                state
                    .storage(k.clone(), at)
                    .map(|v| (k, v))
                    .map_err(|e| to_rpc_error(Error::ScRpcApiError, Some(format!("{:?}", e))))
            })
            .collect();

        let result = Vec::with_capacity(key_futures.len());
        Box::new(
            stream::futures_ordered(key_futures.into_iter()).fold(result, |mut result, kv| {
                let (key, value) = kv;
                let data = match value {
                    None => return future::err(to_rpc_error(Error::NoneForReturnedKey, None)),
                    Some(d) => d,
                };

                let no_prefix = Hasher::reverse(&key.0[32..]);
                let key = match Key::decode(&mut &no_prefix[..]) {
                    Err(_) => {
                        return future::err(to_rpc_error(
                            Key::get_error(),
                            Some(format!("{:?}", &key.0)),
                        ))
                    }
                    Ok(k) => k,
                };

                match Value::decode(&mut &data.0[..]) {
                    Err(_) => future::err(to_rpc_error(
                        Value::get_error(),
                        Some(format!("{:?}", data)),
                    )),
                    Ok(value) => {
                        result.push(ListResult { key, value });
                        future::ok(result)
                    }
                }
            }),
        )
    }
}

pub struct StorageDoubleMap<HasherFirst, HasherSecond>(
    std::marker::PhantomData<(HasherFirst, HasherSecond)>,
);

impl<HasherFirst: StorageHasher, HasherSecond: StorageHasher>
    StorageDoubleMap<HasherFirst, HasherSecond>
{
    pub fn get_value<R, State, KeyFirst, KeySecond, BlockHash>(
        state: &State,
        pallet: &[u8],
        map: &[u8],
        key_first: &KeyFirst,
        key_second: &KeySecond,
        at: Option<BlockHash>,
    ) -> FutureResult<Option<R>>
    where
        R: 'static + Decode + GetError + Send,
        State: sc_rpc_api::state::StateApi<BlockHash>,
        KeyFirst: Encode,
        KeySecond: Encode,
        BlockHash: Copy,
    {
        get_value(
            state,
            key_hash_double_map::<_, _, HasherFirst, HasherSecond>(
                pallet, map, key_first, key_second,
            ),
            at,
        )
    }
}
