use codec::{Encode, Decode};
use super::{frame, RuntimeT};

use pallet_deip_assets::pallet_assets;

type RealRuntime = node_template_runtime::Runtime;

#[derive(Clone, Debug, Eq, PartialEq, Decode, Encode)]
pub struct WrappedCall<T>(pub T);

impl frame::deip_proposal::DeipProposal for RuntimeT {
    type ProposalBatch = pallet_deip_proposal::proposal::ProposalBatch<RealRuntime>;
    type InputProposalBatch = pallet_deip_proposal::proposal::InputProposalBatch<RealRuntime>;
    type ProposalId = pallet_deip_proposal::proposal::ProposalId;
    type Call = node_template_runtime::Call;
    type BatchItem = pallet_deip_proposal::proposal::ProposalBatchItemOf<RealRuntime>;
    type ProposalState = pallet_deip_proposal::proposal::ProposalState;
    type WrappedBatch = Vec<pallet_deip_proposal::proposal::BatchItem<
        node_template_runtime::AccountId, Self::WrappedCall>>;
    type WrappedInputBatch = Vec<pallet_deip_proposal::proposal::BatchItem<
        node_template_runtime::deip_account::DeipAccountId<
            node_template_runtime::AccountId>, Self::WrappedCall>>;
    type WrappedCall = WrappedCall<Self::Call>;

    fn wrap_batch<T: From<Self::WrappedBatch>>(batch: &Self::ProposalBatch) -> T {
        batch.iter().map(|x| {
            pallet_deip_proposal::proposal::BatchItem {
                account: x.account.clone(),
                call: Self::wrap_call(&x.call)
            }
        }).collect::<Self::WrappedBatch>().into()
    }

    fn wrap_input_batch(batch: &Self::InputProposalBatch) -> Self::WrappedInputBatch {
        batch.iter().map(|x| {
            pallet_deip_proposal::proposal::BatchItem {
                account: x.account.clone(),
                call: Self::wrap_call(&x.call)
            }
        }).collect()
    }
    
    fn wrap_call(call: &Self::Call) -> Self::WrappedCall {
        WrappedCall(call.clone())
    }
}

impl frame::deip::Deip for RuntimeT {
    type DomainId = pallet_deip::DomainId;
    type ProjectId = pallet_deip::ProjectId;
    type Project = pallet_deip::Project<Self::Hash, Self::AccountId>;
    type ReviewId = pallet_deip::ReviewId;
    type Review = pallet_deip::Review<Self::Hash, Self::AccountId>;
    type NdaId = pallet_deip::NdaId;
    type NdaAccessRequestId = pallet_deip::NdaAccessRequestId;
    type ProjectContentId = pallet_deip::ProjectContentId;
    type InvestmentId = pallet_deip::InvestmentId;
    type FundingModel = pallet_deip::FundingModelOf<RealRuntime>;
    type ContractAgreementId = pallet_deip::ContractAgreementId;
    type ContractAgreementTerms = pallet_deip::ContractAgreementTermsOf<RealRuntime>;
}

impl frame::deip_org::DeipOrg for RuntimeT {
    type Org = pallet_deip_org::org::OrgOf<RealRuntime>;
}

type AssetId = <RealRuntime as pallet_assets::Config>::AssetId;
type Balance = <RealRuntime as pallet_assets::Config>::Balance;

impl frame::deip_assets::DeipAssets for RuntimeT {
    type AssetId = AssetId;
    type Balance = Balance;
}
