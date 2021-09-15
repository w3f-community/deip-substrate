//! RPC interface for the transaction payment module.

use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use std::sync::Arc;
pub use pallet_deip::api::DeipApi as DeipStorageRuntimeApi;
use pallet_deip::*;
use codec::{Codec};

use common_rpc::{HashOf, FutureResult, ListResult, StorageMap};

use frame_support::Blake2_128Concat;

mod types;

#[rpc]
pub trait DeipStorageApi<BlockHash, AccountId> {
    #[rpc(name = "deipStorage_getProjects")]
    fn get_projects(&self, at: Option<BlockHash>) -> Result<Vec<Project<H256, AccountId>>>;
    #[rpc(name = "deipStorage_getProject")]
    fn get_project(&self, at: Option<BlockHash>, project_id: ProjectId) -> Result<Project<H256, AccountId>>;
    #[rpc(name = "deipStorage_getProjectContentList")]
    fn get_project_content_list(&self, at: Option<BlockHash>, content_ids: Option<Vec<ProjectContentId>>) -> Result<Vec<ProjectContent<H256, AccountId>>>;
    #[rpc(name = "deipStorage_getProjectContent")]
    fn get_project_content(&self, at: Option<BlockHash>, project_id: ProjectId, project_content_id: ProjectContentId) -> Result<ProjectContent<H256, AccountId>>;

    #[rpc(name = "deip_getDomainList")]
    fn get_domains(&self,
        at: Option<BlockHash>,
        count: u32,
        start_id: Option<DomainId>,
    ) -> FutureResult<Vec<ListResult<types::DomainId, types::Domain>>>;

    #[rpc(name = "deip_getDomain")]
    fn get_domain(&self, at: Option<BlockHash>, domain_id: DomainId) -> Result<Domain>;

    #[rpc(name = "deipStorage_getNdaList")]
    fn get_nda_list(&self, at: Option<BlockHash>) -> Result<Vec<Nda<H256, AccountId, u64>>>;
    #[rpc(name = "deipStorage_getNda")]
    fn get_nda(&self, at: Option<BlockHash>, nda_id: NdaId) -> Result<Nda<H256, AccountId, u64>>;
    #[rpc(name = "deipStorage_getReviews")]
    fn get_reviews(&self, at: Option<BlockHash>) -> Result<Vec<Review<H256, AccountId>>>;
    #[rpc(name = "deipStorage_getReview")]
    fn get_review(&self, at: Option<BlockHash>, review_id: ReviewId) -> Result<Review<H256, AccountId>>;
}

/// A struct that implements the `DeipStorage`.
pub struct DeipStorage<C, State, M> {
    // If you have more generics, no need to DeipStorage<C, M, N, P, ...>
    // just use a tuple like DeipStorage<C, (M, N, P, ...)>
    client: Arc<C>,
    state: State,
    _marker: std::marker::PhantomData<M>,
}

impl<C, State, M> DeipStorage<C, State, M> {
    /// Create new `DeipStorage` instance with the given reference to the client.
    pub fn new(client: Arc<C>, state: State) -> Self {
        Self {
            client,
            state,
            _marker: Default::default(),
        }
    }
}

impl<C, State, Block, AccountId> DeipStorageApi<HashOf<Block>, AccountId> for DeipStorage<C, State, Block>
where
    Block: BlockT,
    C: Send + Sync + 'static,
    C: ProvideRuntimeApi<Block>,
    C: HeaderBackend<Block>,
    C::Api: DeipStorageRuntimeApi<Block, AccountId>,
    State: sc_rpc_api::state::StateApi<HashOf<Block>>,
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

    fn get_domains(&self,
        at: Option<HashOf<Block>>,
        count: u32,
        start_id: Option<DomainId>,
    ) -> FutureResult<Vec<ListResult<types::DomainId, types::Domain>>> {
        StorageMap::<Blake2_128Concat>::get_list(&self.state, b"Deip", b"Domains", at, count, start_id.map(|id| types::DomainId{ id }))
    }

    fn get_domain(&self, at: Option<<Block as BlockT>::Hash>, domain_id: DomainId) -> Result<Domain> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
            // If the block hash is not supplied assume the best block.
            self.client.info().best_hash));

        let runtime_api_result = api.get_domain(&at, &domain_id);
        
        
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876), // No real reason for this value
            message: "Something wrong".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }
    
    fn get_project_content_list(&self, at: Option<<Block as BlockT>::Hash>, content_ids: Option<Vec<ProjectContentId>>) -> Result<Vec<ProjectContent<H256, AccountId>>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
            // If the block hash is not supplied assume the best block.
            self.client.info().best_hash));

        let runtime_api_result = api.get_project_content_list(&at, &content_ids);
        
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876), // No real reason for this value
            message: "Something wrong".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }
    fn get_project_content(&self, at: Option<<Block as BlockT>::Hash>, project_id: ProjectId, project_content_id: ProjectContentId) -> Result<ProjectContent<H256, AccountId>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
            // If the block hash is not supplied assume the best block.
            self.client.info().best_hash));

        let runtime_api_result = api.get_project_content(&at, &project_id, &project_content_id);
        
        
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876), // No real reason for this value
            message: "Something wrong".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }

    fn get_nda_list(&self, at: Option<<Block as BlockT>::Hash>) -> Result<Vec<Nda<H256, AccountId, u64>>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
            // If the block hash is not supplied assume the best block.
            self.client.info().best_hash));

        let runtime_api_result = api.get_nda_list(&at);
        
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876), // No real reason for this value
            message: "Something wrong".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }
    fn get_nda(&self, at: Option<<Block as BlockT>::Hash>, nda_id: NdaId) -> Result<Nda<H256, AccountId, u64>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
            // If the block hash is not supplied assume the best block.
            self.client.info().best_hash));

        let runtime_api_result = api.get_nda(&at, &nda_id);
        
        
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876), // No real reason for this value
            message: "Something wrong".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }

    fn get_reviews(&self, at: Option<<Block as BlockT>::Hash>) -> Result<Vec<Review<H256, AccountId>>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
            // If the block hash is not supplied assume the best block.
            self.client.info().best_hash));

        let runtime_api_result = api.get_reviews(&at);
        
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876), // No real reason for this value
            message: "Something wrong".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }
    fn get_review(&self, at: Option<<Block as BlockT>::Hash>, review_id: ReviewId) -> Result<Review<H256, AccountId>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
            // If the block hash is not supplied assume the best block.
            self.client.info().best_hash));

        let runtime_api_result = api.get_review(&at, &review_id);
        
        
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876), // No real reason for this value
            message: "Something wrong".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }

}
