#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_mut_passed)]

use super::*;
use codec::Codec;
use sp_std::vec::Vec;

// Here we declare the runtime API. It is implemented it the `impl` block in
// runtime amalgamator file (the `runtime/src/lib.rs`)
sp_api::decl_runtime_apis! {
    pub trait DeipApi<AccountId, Moment, AssetId, AssetBalance, Hash>
        where
            AccountId: Codec,
            Moment: Codec,
            AssetId: Codec,
            AssetBalance: Codec,
            Hash: Codec,
    {
        fn get_project(project_id: &ProjectId) -> Project<H256, AccountId>;
        fn get_domains() -> Vec<Domain>;
        fn get_domain(domain_id: &DomainId) -> Domain;
        fn get_project_content_list(content_ids: &Option<Vec<ProjectContentId>>) -> Vec<ProjectContent<H256, AccountId>>;
        fn get_project_content(project_id: &ProjectId, project_content_id: &ProjectContentId) -> ProjectContent<H256, AccountId>;
        fn get_nda_list() -> Vec<Nda<H256, AccountId, u64>>;
        fn get_nda(nda_id: &NdaId) -> Nda<H256, AccountId, u64>;
        fn get_reviews() -> Vec<Review<H256, AccountId>>;
        fn get_review(review_id: &ReviewId) -> Review<H256, AccountId>;
        fn get_investment_opportunity(id: &InvestmentId) -> Option<SimpleCrowdfunding<Moment, AssetId, AssetBalance>>;
        fn get_contract_agreement(id: &ContractAgreementId) -> Option<contract::Agreement<AccountId, Hash, Moment, DeipAsset<AssetId, AssetBalance>>>;
    }
}
