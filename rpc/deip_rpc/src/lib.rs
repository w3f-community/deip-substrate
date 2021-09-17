use codec::Codec;
use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
pub use pallet_deip::api::DeipApi as DeipStorageRuntimeApi;
use pallet_deip::*;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use std::sync::Arc;

use common_rpc::{
    get_list_by_index, to_rpc_error, Error, FutureResult, HashOf, ListResult, StorageMap,
};

use frame_support::{Blake2_128Concat, Identity, Twox64Concat};

mod types;

#[rpc]
pub trait DeipStorageApi<BlockHash, AccountId, Moment, AssetId, AssetBalance, Hash> {
    #[rpc(name = "deip_getProjectList")]
    fn get_project_list(
        &self,
        at: Option<BlockHash>,
        count: u32,
        start_id: Option<ProjectId>,
    ) -> FutureResult<Vec<ListResult<ProjectId, Project<H256, AccountId>>>>;

    #[rpc(name = "deipStorage_getProject")]
    fn get_project(
        &self,
        at: Option<BlockHash>,
        project_id: ProjectId,
    ) -> Result<Project<H256, AccountId>>;

    #[rpc(name = "deip_getProjectListByTeam")]
    fn get_project_list_by_team(
        &self,
        at: Option<BlockHash>,
        team_id: AccountId,
        count: u32,
        start_id: Option<ProjectId>,
    ) -> FutureResult<Vec<ListResult<ProjectId, Project<H256, AccountId>>>>;

    #[rpc(name = "deip_getProjectContentList")]
    fn get_project_content_list(
        &self,
        at: Option<BlockHash>,
        count: u32,
        start_id: Option<ProjectContentId>,
    ) -> FutureResult<Vec<ListResult<ProjectContentId, ProjectContent<Hash, AccountId>>>>;

    #[rpc(name = "deip_getProjectContentListByProject")]
    fn get_project_content_list_by_project(
        &self,
        at: Option<BlockHash>,
        project_id: ProjectId,
        count: u32,
        start_id: Option<ProjectContentId>,
    ) -> FutureResult<Vec<ListResult<ProjectContentId, ProjectContent<Hash, AccountId>>>>;

    #[rpc(name = "deip_getProjectContent")]
    fn get_project_content(
        &self,
        at: Option<BlockHash>,
        id: ProjectContentId,
    ) -> Result<ProjectContent<Hash, AccountId>>;

    #[rpc(name = "deip_getDomainList")]
    fn get_domains(
        &self,
        at: Option<BlockHash>,
        count: u32,
        start_id: Option<DomainId>,
    ) -> FutureResult<Vec<ListResult<DomainId, Domain>>>;

    #[rpc(name = "deip_getDomain")]
    fn get_domain(&self, at: Option<BlockHash>, domain_id: DomainId) -> Result<Domain>;

    #[rpc(name = "deipStorage_getNdaList")]
    fn get_nda_list(&self, at: Option<BlockHash>) -> Result<Vec<Nda<H256, AccountId, u64>>>;
    #[rpc(name = "deipStorage_getNda")]
    fn get_nda(&self, at: Option<BlockHash>, nda_id: NdaId) -> Result<Nda<H256, AccountId, u64>>;
    #[rpc(name = "deipStorage_getReviews")]
    fn get_reviews(&self, at: Option<BlockHash>) -> Result<Vec<Review<H256, AccountId>>>;
    #[rpc(name = "deipStorage_getReview")]
    fn get_review(
        &self,
        at: Option<BlockHash>,
        review_id: ReviewId,
    ) -> Result<Review<H256, AccountId>>;

    #[rpc(name = "deip_getInvestmentOpportunity")]
    fn get_investment_opportunity(
        &self,
        at: Option<BlockHash>,
        id: InvestmentId,
    ) -> Result<Option<SimpleCrowdfunding<Moment, AssetId, AssetBalance>>>;

    #[rpc(name = "deip_getInvestmentOpportunityList")]
    fn get_investment_opportunity_list(
        &self,
        at: Option<BlockHash>,
        count: u32,
        start_id: Option<InvestmentId>,
    ) -> FutureResult<
        Vec<ListResult<InvestmentId, SimpleCrowdfunding<Moment, AssetId, AssetBalance>>>,
    >;

    #[rpc(name = "deip_getContractAgreement")]
    fn get_contract_agreement(
        &self,
        at: Option<BlockHash>,
        id: ContractAgreementId,
    ) -> Result<
        Option<contract::Agreement<AccountId, Hash, Moment, DeipAsset<AssetId, AssetBalance>>>,
    >;

    #[rpc(name = "deip_getContractAgreementList")]
    fn get_contract_agreement_list(
        &self,
        at: Option<BlockHash>,
        count: u32,
        start_id: Option<ContractAgreementId>,
    ) -> FutureResult<
        Vec<
            ListResult<
                ContractAgreementId,
                contract::Agreement<AccountId, Hash, Moment, DeipAsset<AssetId, AssetBalance>>,
            >,
        >,
    >;

    #[rpc(name = "deip_getContractAgreementListByType")]
    fn get_contract_agreement_list_by_type(
        &self,
        at: Option<BlockHash>,
        key: ContractAgreementIndexTerms,
        count: u32,
        start_id: Option<ContractAgreementId>,
    ) -> FutureResult<
        Vec<
            ListResult<
                ContractAgreementId,
                contract::Agreement<AccountId, Hash, Moment, DeipAsset<AssetId, AssetBalance>>,
            >,
        >,
    >;
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

impl<C, State, Block, AccountId, Moment, AssetId, AssetBalance, Hash>
    DeipStorageApi<HashOf<Block>, AccountId, Moment, AssetId, AssetBalance, Hash>
    for DeipStorage<C, State, Block>
where
    Block: BlockT,
    C: Send + Sync + 'static,
    C: ProvideRuntimeApi<Block>,
    C: HeaderBackend<Block>,
    C::Api: DeipStorageRuntimeApi<Block, AccountId, Moment, AssetId, AssetBalance, Hash>,
    State: sc_rpc_api::state::StateApi<HashOf<Block>>,
    AccountId: 'static + Codec + Send,
    Moment: 'static + Codec + Send,
    AssetId: 'static + Codec + Send,
    AssetBalance: 'static + Codec + Send,
    Hash: 'static + Codec + Send,
{
    fn get_project_list(
        &self,
        at: Option<HashOf<Block>>,
        count: u32,
        start_id: Option<ProjectId>,
    ) -> FutureResult<Vec<ListResult<ProjectId, Project<H256, AccountId>>>> {
        StorageMap::<Identity>::get_list(
            &self.state,
            at,
            b"Deip",
            b"ProjectMap",
            count,
            start_id.map(types::ProjectKeyValue::new),
        )
    }

    fn get_project(
        &self,
        at: Option<<Block as BlockT>::Hash>,
        project_id: ProjectId,
    ) -> Result<Project<H256, AccountId>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

        let runtime_api_result = api.get_project(&at, &project_id);
        runtime_api_result
            .map_err(|e| to_rpc_error(Error::ProjectApiGetFailed, Some(format!("{:?}", e))))
    }

    fn get_project_list_by_team(
        &self,
        at: Option<HashOf<Block>>,
        key: AccountId,
        count: u32,
        start_id: Option<ProjectId>,
    ) -> FutureResult<Vec<ListResult<ProjectId, Project<H256, AccountId>>>> {
        get_list_by_index::<Blake2_128Concat, Identity, _, _, _, _>(
            &self.state,
            at,
            b"Deip",
            b"ProjectIdByTeamId",
            b"ProjectMap",
            count,
            &key,
            start_id.map(types::ProjectKeyValue::new),
        )
    }

    fn get_domains(
        &self,
        at: Option<HashOf<Block>>,
        count: u32,
        start_id: Option<DomainId>,
    ) -> FutureResult<Vec<ListResult<DomainId, Domain>>> {
        StorageMap::<Blake2_128Concat>::get_list(
            &self.state,
            at,
            b"Deip",
            b"Domains",
            count,
            start_id.map(types::DomainKeyValue::new),
        )
    }

    fn get_domain(
        &self,
        at: Option<<Block as BlockT>::Hash>,
        domain_id: DomainId,
    ) -> Result<Domain> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

        let runtime_api_result = api.get_domain(&at, &domain_id);
        runtime_api_result
            .map_err(|e| to_rpc_error(Error::DomainApiGetFailed, Some(format!("{:?}", e))))
    }

    fn get_project_content_list(
        &self,
        at: Option<HashOf<Block>>,
        count: u32,
        start_id: Option<ProjectContentId>,
    ) -> FutureResult<Vec<ListResult<ProjectContentId, ProjectContent<Hash, AccountId>>>> {
        StorageMap::<Identity>::get_list(
            &self.state,
            at,
            b"Deip",
            b"ProjectContentMap",
            count,
            start_id.map(types::ProjectContentKeyValue::new),
        )
    }

    fn get_project_content_list_by_project(
        &self,
        at: Option<HashOf<Block>>,
        key: ProjectId,
        count: u32,
        start_id: Option<ProjectContentId>,
    ) -> FutureResult<Vec<ListResult<ProjectContentId, ProjectContent<Hash, AccountId>>>> {
        get_list_by_index::<Identity, Identity, _, _, _, _>(
            &self.state,
            at,
            b"Deip",
            b"ContentIdByProjectId",
            b"ProjectContentMap",
            count,
            &key,
            start_id.map(types::ProjectContentKeyValue::new),
        )
    }

    fn get_project_content(
        &self,
        at: Option<HashOf<Block>>,
        id: ProjectContentId,
    ) -> Result<ProjectContent<Hash, AccountId>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

        let runtime_api_result = api.get_project_content(&at, &id);
        runtime_api_result
            .map_err(|e| to_rpc_error(Error::ProjectContentApiGetFailed, Some(format!("{:?}", e))))
    }

    fn get_nda_list(
        &self,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<Vec<Nda<H256, AccountId, u64>>> {
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
    fn get_nda(
        &self,
        at: Option<<Block as BlockT>::Hash>,
        nda_id: NdaId,
    ) -> Result<Nda<H256, AccountId, u64>> {
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

    fn get_reviews(
        &self,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<Vec<Review<H256, AccountId>>> {
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
    fn get_review(
        &self,
        at: Option<<Block as BlockT>::Hash>,
        review_id: ReviewId,
    ) -> Result<Review<H256, AccountId>> {
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

    fn get_investment_opportunity(
        &self,
        at: Option<HashOf<Block>>,
        id: InvestmentId,
    ) -> Result<Option<SimpleCrowdfunding<Moment, AssetId, AssetBalance>>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

        let runtime_api_result = api.get_investment_opportunity(&at, &id);
        runtime_api_result.map_err(|e| {
            to_rpc_error(
                Error::InvestmentOpportunityApiGetFailed,
                Some(format!("{:?}", e)),
            )
        })
    }

    fn get_investment_opportunity_list(
        &self,
        at: Option<HashOf<Block>>,
        count: u32,
        start_id: Option<InvestmentId>,
    ) -> FutureResult<
        Vec<ListResult<InvestmentId, SimpleCrowdfunding<Moment, AssetId, AssetBalance>>>,
    > {
        StorageMap::<Identity>::get_list(
            &self.state,
            at,
            b"Deip",
            b"SimpleCrowdfundingMap",
            count,
            start_id.map(types::InvestmentOpportunityKeyValue::new),
        )
    }

    fn get_contract_agreement(
        &self,
        at: Option<HashOf<Block>>,
        id: ContractAgreementId,
    ) -> Result<
        Option<contract::Agreement<AccountId, Hash, Moment, DeipAsset<AssetId, AssetBalance>>>,
    > {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

        let runtime_api_result = api.get_contract_agreement(&at, &id);
        runtime_api_result
            .map_err(|e| to_rpc_error(Error::AgreementApiGetFailed, Some(format!("{:?}", e))))
    }

    fn get_contract_agreement_list(
        &self,
        at: Option<HashOf<Block>>,
        count: u32,
        start_id: Option<ContractAgreementId>,
    ) -> FutureResult<
        Vec<
            ListResult<
                ContractAgreementId,
                contract::Agreement<AccountId, Hash, Moment, DeipAsset<AssetId, AssetBalance>>,
            >,
        >,
    > {
        StorageMap::<Blake2_128Concat>::get_list(
            &self.state,
            at,
            b"Deip",
            b"ContractAgreementMap",
            count,
            start_id.map(types::AgreementKeyValue::new),
        )
    }

    fn get_contract_agreement_list_by_type(
        &self,
        at: Option<HashOf<Block>>,
        key: ContractAgreementIndexTerms,
        count: u32,
        start_id: Option<ContractAgreementId>,
    ) -> FutureResult<
        Vec<
            ListResult<
                ContractAgreementId,
                contract::Agreement<AccountId, Hash, Moment, DeipAsset<AssetId, AssetBalance>>,
            >,
        >,
    > {
        get_list_by_index::<Twox64Concat, Blake2_128Concat, _, _, _, _>(
            &self.state,
            at,
            b"Deip",
            b"ContractAgreementIdByType",
            b"ContractAgreementMap",
            count,
            &key,
            start_id.map(types::AgreementKeyValue::new),
        )
    }
}
