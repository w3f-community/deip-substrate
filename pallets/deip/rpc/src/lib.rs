//! RPC interface for the transaction payment module.

use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use std::sync::Arc;
use deip_runtime_api::{DeipApi as DeipStorageRuntimeApi, ProjectId, H256, Project};
use codec::{Codec};


#[rpc]
pub trait DeipStorageApi<BlockHash, AccountId> {
	#[rpc(name = "deipStorage_getProjects")]
	fn get_projects(&self, at: Option<BlockHash>) -> Result<Vec<Project<H256, AccountId>>>;
	#[rpc(name = "deipStorage_getProject")]
	fn get_project(&self, at: Option<BlockHash>, project_id: ProjectId) -> Result<Project<H256, AccountId>>;
}

/// A struct that implements the `DeipStorage`.
pub struct DeipStorage<C, M> {
	// If you have more generics, no need to DeipStorage<C, M, N, P, ...>
	// just use a tuple like DeipStorage<C, (M, N, P, ...)>
	client: Arc<C>,
	_marker: std::marker::PhantomData<M>,
}

impl<C, M> DeipStorage<C, M> {
	/// Create new `DeipStorage` instance with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self {
			client,
			_marker: Default::default(),
		}
	}
}

/// Error type of this RPC api.
// pub enum Error {
// 	/// The transaction was not decodable.
// 	DecodeError,
// 	/// The call to runtime failed.
// 	RuntimeError,
// }
//
// impl From<Error> for i64 {
// 	fn from(e: Error) -> i64 {
// 		match e {
// 			Error::RuntimeError => 1,
// 			Error::DecodeError => 2,
// 		}
// 	}
// }

impl<C, Block, AccountId> DeipStorageApi<<Block as BlockT>::Hash, AccountId> for DeipStorage<C, Block>
where
	Block: BlockT,
	C: Send + Sync + 'static,
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block>,
	C::Api: DeipStorageRuntimeApi<Block, AccountId>,
	AccountId: Codec,
{
	fn get_projects(&self, at: Option<<Block as BlockT>::Hash>) -> Result<Vec<Project<H256, AccountId>>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let runtime_api_result = api.get_projects(&at)
			.map(|projects| {
				projects.iter()
					.map(|(project_id, ..)| api.get_project(&at, project_id).unwrap())
					.collect()		
			});
		
		runtime_api_result.map_err(|e| RpcError {
			code: ErrorCode::ServerError(9876), // No real reason for this value
			message: "Something wrong".into(),
			data: Some(format!("{:?}", e).into()),
		})
	}
	fn get_project(&self, at: Option<<Block as BlockT>::Hash>, project_id: ProjectId) -> Result<Project<H256, AccountId>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let runtime_api_result = api.get_project(&at, &project_id);
		
		runtime_api_result.map_err(|e| RpcError {
			code: ErrorCode::ServerError(9876), // No real reason for this value
			message: "Something wrong".into(),
			data: Some(format!("{:?}", e).into()),
		})
	}
}
