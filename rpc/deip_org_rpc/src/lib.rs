use jsonrpc_core::Result as RpcResult;
use jsonrpc_derive::rpc;

use std::sync::Arc;
use std::vec::Vec;

use codec::Codec;

use sp_runtime::{generic::BlockId, traits::Block as BlockT};

use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;

pub use pallet_deip_org::api::DeipOrgRuntimeApi;
use pallet_deip_org::api::{GetMultiResult, GetResult};
use pallet_deip_org::org::{Org, OrgName};

use frame_support::Blake2_128Concat;

use common_rpc::{FutureResult, HashOf, ListResult, StorageMap};

mod types;

#[rpc]
pub trait DeipOrgRpcApi<BlockHash, AccountId> {
    #[rpc(name = "deipDao_get")]
    fn get(&self, at: Option<BlockHash>, name: OrgName) -> RpcResult<GetResult<AccountId>>;

    #[rpc(name = "deipDao_getMulti")]
    fn get_multi(
        &self,
        at: Option<BlockHash>,
        names: Vec<OrgName>,
    ) -> RpcResult<GetMultiResult<AccountId>>;

    #[rpc(name = "deipDao_getList")]
    fn list(
        &self,
        at: Option<BlockHash>,
        count: u32,
        start_id: Option<OrgName>,
    ) -> FutureResult<Vec<ListResult<OrgName, Org<AccountId, OrgName>>>>;
}

pub struct DeipOrgRpcApiObj<C, State, Block> {
    client: Arc<C>,
    state: State,
    _marker: std::marker::PhantomData<Block>,
}

impl<C, State, Block> DeipOrgRpcApiObj<C, State, Block> {
    pub fn new(client: Arc<C>, state: State) -> Self {
        Self {
            client,
            state,
            _marker: Default::default(),
        }
    }
}

impl<C, State, Block, AccountId> DeipOrgRpcApi<HashOf<Block>, AccountId>
    for DeipOrgRpcApiObj<C, State, Block>
where
    Block: BlockT,
    C: Send + Sync + 'static,
    C: ProvideRuntimeApi<Block>,
    C: HeaderBackend<Block>,
    C::Api: DeipOrgRuntimeApi<Block, AccountId>,
    State: sc_rpc_api::state::StateApi<HashOf<Block>>,
    AccountId: 'static + Codec + std::marker::Send,
{
    fn get(
        &self,
        at: Option<<Block as BlockT>::Hash>,
        name: OrgName,
    ) -> RpcResult<GetResult<AccountId>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

        let runtime_api_result = api.get(&at, name);
        runtime_api_result.map_err(|e| {
            common_rpc::to_rpc_error(common_rpc::Error::DaoApiGetFailed, Some(format!("{:?}", e)))
        })
    }

    fn get_multi(
        &self,
        at: Option<<Block as BlockT>::Hash>,
        names: Vec<OrgName>,
    ) -> RpcResult<GetMultiResult<AccountId>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

        let runtime_api_result = api.get_multi(&at, names);
        runtime_api_result.map_err(|e| {
            common_rpc::to_rpc_error(
                common_rpc::Error::DaoApiGetMultiFailed,
                Some(format!("{:?}", e)),
            )
        })
    }

    fn list(
        &self,
        at: Option<HashOf<Block>>,
        count: u32,
        start_id: Option<OrgName>,
    ) -> FutureResult<Vec<ListResult<OrgName, Org<AccountId, OrgName>>>> {
        StorageMap::<Blake2_128Concat>::get_list(
            &self.state,
            b"DeipOrg",
            b"OrgRepository",
            at,
            count,
            start_id.map(types::DaoKeyValue::new),
        )
    }
}
