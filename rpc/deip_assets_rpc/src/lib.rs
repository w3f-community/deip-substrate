use jsonrpc_core::{Error as RpcError, ErrorCode as RpcErrorCode, Result as RpcResult};
use jsonrpc_derive::rpc;

use std::sync::Arc;
use std::vec::Vec;

use codec::Codec;

use sp_runtime::{generic::BlockId, traits::Block as BlockT};

use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;

#[rpc]
pub trait DeipAssetsRpc<BlockHash, AccountId> {}

pub struct DeipAssetsRpcObj<C, Block> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<Block>,
}

impl<C, Block> DeipAssetsRpcObj<C, Block> {
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _marker: Default::default(),
        }
    }
}
