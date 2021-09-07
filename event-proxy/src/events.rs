use std::fmt::Debug;

use substrate_subxt::{RawEvent, Event, system::System};
use codec::Decode;
use serde::{Serialize, ser::{Serializer}};

use sp_runtime::generic::Block;
use sp_runtime::traits::{Block as _Block, Header as _Header};

use super::frame::{
    deip_proposal::{self, DeipProposal},
    deip::{self, Deip},
    deip_org::{self, DeipOrg},
    deip_assets::{self, DeipAssets},
};

mod mapping;

#[derive(Serialize, Debug, Copy, Clone)]
pub struct BlockMetadata<T: System> {
    pub number: T::BlockNumber,
    pub hash: T::Hash,
    pub parent_hash: T::Hash,
}
impl<T: System> BlockMetadata<T> {
    pub fn new(block: &Block<T::Header, T::Extrinsic>) -> Self { Self {
        number: block.header().number().to_owned(),
        hash: block.header().hash(),
        parent_hash: block.header().parent_hash().to_owned(),
    }}
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum TypedEvent<D, I> where Self: From<D> + From<I>
{
    Domain(D),
    Infrastructure(I),
}

pub type SpecializedEvent<T> = TypedEvent<DomainEvent<T>, InfrastructureEvent<T>>;

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum InfrastructureEventData<BlockCreated> {
    BlockCreated(BlockCreated),
}
#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum InfrastructureEventMeta {
    BlockCreated { domain_events: u32 },
}

pub type InfrastructureEvent<T> = BaseEvent<InfrastructureEventData<BlockMetadata<T>>, InfrastructureEventMeta>;

impl<T: System> InfrastructureEvent<T> {
    pub fn block_created(block: &Block<T::Header, T::Extrinsic>, domain_events: u32) -> Self { Self {
        name: "block_created".to_string(),
        data: InfrastructureEventData::BlockCreated(BlockMetadata::new(block)),
        meta: InfrastructureEventMeta::BlockCreated { domain_events }
    } }
}

#[derive(Serialize, Debug)]
pub struct BaseEvent<Data, Meta> {
    name: String,
    data: Data,
    meta: Meta,
}

#[derive(Serialize, Debug)]
pub struct DomainEventMeta<Block> {
    index: u32,
    block: Block,
}

pub type DomainEvent<T> = BaseEvent<DomainEventData<T>, DomainEventMeta<BlockMetadata<T>>>;

impl<T> From<DomainEvent<T>> for SpecializedEvent<T>
    where T: Deip + DeipProposal + DeipOrg + DeipAssets
{
    fn from(source: DomainEvent<T>) -> Self { Self::Domain(source) }
}

impl<T> From<InfrastructureEvent<T>> for SpecializedEvent<T>
    where T: Deip + DeipProposal + DeipOrg + DeipAssets
{
    fn from(source: InfrastructureEvent<T>) -> Self { Self::Infrastructure(source) }
}

impl<T: DeipProposal + Deip + DeipOrg + DeipAssets> Serialize for DomainEventData<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        match self {
            // =============== DeipProposal:
            ProposalProposed(e) => e.serialize(serializer),
            ProposalApproved(e) => e.serialize(serializer),
            ProposalRevokedApproval(e) => e.serialize(serializer),
            ProposalResolved(e) => e.serialize(serializer),
            ProposalExpired(e) => e.serialize(serializer),
            // =============== Deip:
            ProjectCreated(e) => e.serialize(serializer),
            ProjectRemoved(e) => e.serialize(serializer),
            ProjectUpdated(e) => e.serialize(serializer),
            ProjectContentCreated(e) => e.serialize(serializer),
            NdaCreated(e) => e.serialize(serializer),
            NdaAccessRequestCreated(e) => e.serialize(serializer),
            NdaAccessRequestFulfilled(e) => e.serialize(serializer),
            NdaAccessRequestRejected(e) => e.serialize(serializer),
            DomainAdded(e) => e.serialize(serializer),
            ReviewCreated(e) => e.serialize(serializer),
            ReviewUpvoted(e) => e.serialize(serializer),
            SimpleCrowdfundingCreated(e) => e.serialize(serializer),
            SimpleCrowdfundingActivated(e) => e.serialize(serializer),
            SimpleCrowdfundingFinished(e) => e.serialize(serializer),
            SimpleCrowdfundingExpired(e) => e.serialize(serializer),
            Invested(e) => e.serialize(serializer),
            ContractAgreementCreated(e) => e.serialize(serializer),
            ContractAgreementAccepted(e) => e.serialize(serializer),
            // =============== DeipOrg:
            OrgCreate(e) => e.serialize(serializer),
            OrgTransferOwnership(e) => e.serialize(serializer),
            // =============== DeipAssets:
            AssetClassCreated(e) => e.serialize(serializer),
            AssetIssued(e) => e.serialize(serializer),
            AssetTransferred(e) => e.serialize(serializer),
            AssetBurned(e) => e.serialize(serializer),
            AssetTeamChanged(e) => e.serialize(serializer),
            AssetOwnerChanged(e) => e.serialize(serializer),
            AssetForceTransferred(e) => e.serialize(serializer),
            AssetAccountFrozen(e) => e.serialize(serializer),
            AssetAccountThawed(e) => e.serialize(serializer),
            AssetFrozen(e) => e.serialize(serializer),
            AssetThawed(e) => e.serialize(serializer),
            AssetClassDestroyed(e) => e.serialize(serializer),
            AssetClassForceCreated(e) => e.serialize(serializer),
            AssetMaxZombiesChanged(e) => e.serialize(serializer),
            AssetMetadataSet(e) => e.serialize(serializer),
        }
    }
}

pub use DomainEventData::*;

#[derive(Debug)]
pub enum DomainEventData<T: DeipProposal + Deip + DeipOrg + DeipAssets> {
    // DeipProposal:
    ProposalProposed(deip_proposal::ProposedEvent<T>),
    ProposalApproved(deip_proposal::ApprovedEvent<T>),
    ProposalRevokedApproval(deip_proposal::RevokedApprovalEvent<T>),
    ProposalResolved(deip_proposal::ResolvedEvent<T>),
    ProposalExpired(deip_proposal::ExpiredEvent<T>),
    // Deip:
    ProjectCreated(deip::ProjectCreatedEvent<T>),
    ProjectRemoved(deip::ProjectRemovedEvent<T>),
    ProjectUpdated(deip::ProjectUpdatedEvent<T>),
    ProjectContentCreated(deip::ProjectContentCreatedEvent<T>),
    NdaCreated(deip::NdaCreatedEvent<T>),
    NdaAccessRequestCreated(deip::NdaAccessRequestCreatedEvent<T>),
    NdaAccessRequestFulfilled(deip::NdaAccessRequestFulfilledEvent<T>),
    NdaAccessRequestRejected(deip::NdaAccessRequestRejectedEvent<T>),
    DomainAdded(deip::DomainAddedEvent<T>),
    ReviewCreated(deip::ReviewCreatedEvent<T>),
    ReviewUpvoted(deip::ReviewUpvotedEvent<T>),
    SimpleCrowdfundingCreated(deip::SimpleCrowdfundingCreatedEvent<T>),
    SimpleCrowdfundingActivated(deip::SimpleCrowdfundingActivatedEvent<T>),
    SimpleCrowdfundingFinished(deip::SimpleCrowdfundingFinishedEvent<T>),
    SimpleCrowdfundingExpired(deip::SimpleCrowdfundingExpiredEvent<T>),
    Invested(deip::InvestedEvent<T>),
    ContractAgreementCreated(deip::ContractAgreementCreatedEvent<T>),
    ContractAgreementAccepted(deip::ContractAgreementAcceptedEvent<T>),
    // DeipOrg:
    OrgCreate(deip_org::OrgCreateEvent<T>),
    OrgTransferOwnership(deip_org::OrgTransferOwnershipEvent<T>),
    // DeipAssets:
    AssetClassCreated(deip_assets::CreatedEvent<T>),
    AssetIssued(deip_assets::IssuedEvent<T>),
    AssetTransferred(deip_assets::TransferredEvent<T>),
    AssetBurned(deip_assets::BurnedEvent<T>),
    AssetTeamChanged(deip_assets::TeamChangedEvent<T>),
    AssetOwnerChanged(deip_assets::OwnerChangedEvent<T>),
    AssetForceTransferred(deip_assets::ForceTransferredEvent<T>),
    AssetAccountFrozen(deip_assets::FrozenEvent<T>),
    AssetAccountThawed(deip_assets::ThawedEvent<T>),
    AssetFrozen(deip_assets::AssetFrozenEvent<T>),
    AssetThawed(deip_assets::AssetThawedEvent<T>),
    AssetClassDestroyed(deip_assets::DestroyedEvent<T>),
    AssetClassForceCreated(deip_assets::ForceCreatedEvent<T>),
    AssetMaxZombiesChanged(deip_assets::MaxZombiesChangedEvent<T>),
    AssetMetadataSet(deip_assets::MetadataSetEvent<T>),
}

pub fn known_domain_events<T: DeipProposal + Deip + DeipOrg + DeipAssets + Debug>(
    raw: &(u32, RawEvent),
    block: &Block<<T as System>::Header, <T as System>::Extrinsic>
)
    -> Result<Option<SpecializedEvent<T>>, codec::Error> 
{
    let (index, raw) = raw;
    let meta = DomainEventMeta {
        index: *index,
        block: BlockMetadata::new(block),
    };
    let event = match (raw.module.as_str(), raw.variant.as_str()) {
        // =========== DeipProposal:
        (
            deip_proposal::ProposedEvent::<T>::MODULE,
            deip_proposal::ProposedEvent::<T>::EVENT
        ) => DomainEvent {
            name: "proposal_proposed".to_string(),
            data: decode_event_data(raw).map(ProposalProposed)?,
            meta,
        },
        (
            deip_proposal::ApprovedEvent::<T>::MODULE,
            deip_proposal::ApprovedEvent::<T>::EVENT
        ) => DomainEvent {
            name: "proposal_approved".to_string(),
            data: decode_event_data(raw).map(ProposalApproved)?,
            meta,
        },
        (
            deip_proposal::RevokedApprovalEvent::<T>::MODULE,
            deip_proposal::RevokedApprovalEvent::<T>::EVENT
        ) => DomainEvent {
            name: "proposal_revokedApproval".to_string(),
            data: decode_event_data(raw).map(ProposalRevokedApproval)?,
            meta,
        },
        (
            deip_proposal::ResolvedEvent::<T>::MODULE,
            deip_proposal::ResolvedEvent::<T>::EVENT
        ) => DomainEvent {
            name: "proposal_resolved".to_string(),
            data: decode_event_data(raw).map(ProposalResolved)?,
            meta,
        },
        (
            deip_proposal::ExpiredEvent::<T>::MODULE,
            deip_proposal::ExpiredEvent::<T>::EVENT
        ) => DomainEvent {
            name: "proposal_expired".to_string(),
            data: decode_event_data(raw).map(ProposalExpired)?,
            meta,
        },
        // =========== Deip:
        (
            deip::ProjectCreatedEvent::<T>::MODULE,
            deip::ProjectCreatedEvent::<T>::EVENT
        ) => DomainEvent {
            name: "project_created".to_string(),
            data: decode_event_data(raw).map(ProjectCreated)?,
            meta,
        },   
        (                               
            deip::ProjectRemovedEvent::<T>::MODULE,
            deip::ProjectRemovedEvent::<T>::EVENT
        ) => DomainEvent {
            name: "project_removed".to_string(),
            data: decode_event_data(raw).map(ProjectRemoved)?,
            meta,
        },
        (                               
            deip::ProjectUpdatedEvent::<T>::MODULE,
            deip::ProjectUpdatedEvent::<T>::EVENT
        ) => DomainEvent {
            name: "project_updated".to_string(),
            data: decode_event_data(raw).map(ProjectUpdated)?,
            meta,
        },
        (                               
            deip::ProjectContentCreatedEvent::<T>::MODULE,
            deip::ProjectContentCreatedEvent::<T>::EVENT
        ) => DomainEvent {
            name: "project_contentCreated".to_string(),
            data: decode_event_data(raw).map(ProjectContentCreated)?,
            meta,
        },
        (                               
            deip::NdaCreatedEvent::<T>::MODULE,
            deip::NdaCreatedEvent::<T>::EVENT
        ) => DomainEvent {
            name: "project_ndaCreated".to_string(),
            data: decode_event_data(raw).map(NdaCreated)?,
            meta,
        },
        (                               
            deip::NdaAccessRequestCreatedEvent::<T>::MODULE,
            deip::NdaAccessRequestCreatedEvent::<T>::EVENT
        ) => DomainEvent {
            name: "project_ndaAccessRequestCreated".to_string(),
            data: decode_event_data(raw).map(NdaAccessRequestCreated)?,
            meta,
        },
        (                               
            deip::NdaAccessRequestFulfilledEvent::<T>::MODULE,
            deip::NdaAccessRequestFulfilledEvent::<T>::EVENT
        ) => DomainEvent {
            name: "project_ndaAccessRequestFulfilled".to_string(),
            data: decode_event_data(raw).map(NdaAccessRequestFulfilled)?,
            meta,
        },
        (                               
            deip::NdaAccessRequestRejectedEvent::<T>::MODULE,
            deip::NdaAccessRequestRejectedEvent::<T>::EVENT
        ) => DomainEvent {
            name: "project_ndaAccessRequestRejected".to_string(),
            data: decode_event_data(raw).map(NdaAccessRequestRejected)?,
            meta,
        },
        (                               
            deip::DomainAddedEvent::<T>::MODULE,
            deip::DomainAddedEvent::<T>::EVENT
        ) => DomainEvent {
            name: "project_domainAdded".to_string(),
            data: decode_event_data(raw).map(DomainAdded)?,
            meta,
        },
        (                               
            deip::ReviewCreatedEvent::<T>::MODULE,
            deip::ReviewCreatedEvent::<T>::EVENT
        ) => DomainEvent {
            name: "project_reviewCreated".to_string(),
            data: decode_event_data(raw).map(ReviewCreated)?,
            meta,
        },
        (                               
            deip::ReviewUpvotedEvent::<T>::MODULE,
            deip::ReviewUpvotedEvent::<T>::EVENT
        ) => DomainEvent {
            name: "project_reviewUpvoted".to_string(),
            data: decode_event_data(raw).map(ReviewUpvoted)?,
            meta,
        },
        (                               
            deip::SimpleCrowdfundingCreatedEvent::<T>::MODULE,
            deip::SimpleCrowdfundingCreatedEvent::<T>::EVENT
        ) => DomainEvent {
            name: "project_tokenSaleCreated".to_string(),
            data: decode_event_data(raw).map(SimpleCrowdfundingCreated)?,
            meta,
        },
        (                               
            deip::SimpleCrowdfundingActivatedEvent::<T>::MODULE,
            deip::SimpleCrowdfundingActivatedEvent::<T>::EVENT
        ) => DomainEvent {
            name: "project_tokenSaleActivated".to_string(),
            data: decode_event_data(raw).map(SimpleCrowdfundingActivated)?,
            meta,
        },
        (                               
            deip::SimpleCrowdfundingFinishedEvent::<T>::MODULE,
            deip::SimpleCrowdfundingFinishedEvent::<T>::EVENT
        ) => DomainEvent {
            name: "project_tokenSaleFinished".to_string(),
            data: decode_event_data(raw).map(SimpleCrowdfundingFinished)?,
            meta,
        },
        (                               
            deip::SimpleCrowdfundingExpiredEvent::<T>::MODULE,
            deip::SimpleCrowdfundingExpiredEvent::<T>::EVENT
        ) => DomainEvent {
            name: "project_tokenSaleExpired".to_string(),
            data: decode_event_data(raw).map(SimpleCrowdfundingExpired)?,
            meta,
        },
        (                               
            deip::InvestedEvent::<T>::MODULE,
            deip::InvestedEvent::<T>::EVENT
        ) => DomainEvent {
            name: "project_tokenSaleContributed".to_string(),
            data: decode_event_data(raw).map(Invested)?,
            meta,
        },
        (
            deip::ContractAgreementCreatedEvent::<T>::MODULE,
            deip::ContractAgreementCreatedEvent::<T>::EVENT
        ) => DomainEvent {
            name: "deip_contractAgreementCreated".to_string(),
            data: decode_event_data(raw).map(ContractAgreementCreated)?,
            meta,
        },
        (
            deip::ContractAgreementAcceptedEvent::<T>::MODULE,
            deip::ContractAgreementAcceptedEvent::<T>::EVENT
        ) => DomainEvent {
            name: "deip_contractAgreementAccepted".to_string(),
            data: decode_event_data(raw).map(ContractAgreementAccepted)?,
            meta,
        },
        // =========== DeipOrg:
        (                               
            deip_org::OrgCreateEvent::<T>::MODULE,
            deip_org::OrgCreateEvent::<T>::EVENT
        ) => DomainEvent {
            name: "dao_create".to_string(),
            data: decode_event_data(raw).map(OrgCreate)?,
            meta,
        },
        (                               
            deip_org::OrgTransferOwnershipEvent::<T>::MODULE,
            deip_org::OrgTransferOwnershipEvent::<T>::EVENT
        ) => DomainEvent {
            name: "dao_transferOwnership".to_string(),
            data: decode_event_data(raw).map(OrgTransferOwnership)?,
            meta,
        },
        // =========== DeipAssets:
        (                               
            deip_assets::CreatedEvent::<T>::MODULE,
            deip_assets::CreatedEvent::<T>::EVENT
        ) => DomainEvent {
            name: "asset_class_created".to_string(),
            data: decode_event_data(raw).map(AssetClassCreated)?,
            meta,
        },
        (                               
            deip_assets::IssuedEvent::<T>::MODULE,
            deip_assets::IssuedEvent::<T>::EVENT
        ) => DomainEvent {
            name: "asset_issued".to_string(),
            data: decode_event_data(raw).map(AssetIssued)?,
            meta,
        },
        (                               
            deip_assets::TransferredEvent::<T>::MODULE,
            deip_assets::TransferredEvent::<T>::EVENT
        ) => DomainEvent {
            name: "asset_transferred".to_string(),
            data: decode_event_data(raw).map(AssetTransferred)?,
            meta,
        },
        (                               
            deip_assets::BurnedEvent::<T>::MODULE,
            deip_assets::BurnedEvent::<T>::EVENT
        ) => DomainEvent {
            name: "asset_burned".to_string(),
            data: decode_event_data(raw).map(AssetBurned)?,
            meta,
        },
        (                               
            deip_assets::TeamChangedEvent::<T>::MODULE,
            deip_assets::TeamChangedEvent::<T>::EVENT
        ) => DomainEvent {
            name: "asset_team_changed".to_string(),
            data: decode_event_data(raw).map(AssetTeamChanged)?,
            meta,
        },
        (                               
            deip_assets::OwnerChangedEvent::<T>::MODULE,
            deip_assets::OwnerChangedEvent::<T>::EVENT
        ) => DomainEvent {
            name: "asset_owner_changed".to_string(),
            data: decode_event_data(raw).map(AssetOwnerChanged)?,
            meta,
        },
        (                               
            deip_assets::ForceTransferredEvent::<T>::MODULE,
            deip_assets::ForceTransferredEvent::<T>::EVENT
        ) => DomainEvent {
            name: "asset_force_transferred".to_string(),
            data: decode_event_data(raw).map(AssetForceTransferred)?,
            meta,
        },
        (                               
            deip_assets::FrozenEvent::<T>::MODULE,
            deip_assets::FrozenEvent::<T>::EVENT
        ) => DomainEvent {
            name: "asset_account_frozen".to_string(),
            data: decode_event_data(raw).map(AssetAccountFrozen)?,
            meta,
        },
        (                               
            deip_assets::ThawedEvent::<T>::MODULE,
            deip_assets::ThawedEvent::<T>::EVENT
        ) => DomainEvent {
            name: "asset_account_thawed".to_string(),
            data: decode_event_data(raw).map(AssetAccountThawed)?,
            meta,
        },
        (                               
            deip_assets::AssetFrozenEvent::<T>::MODULE,
            deip_assets::AssetFrozenEvent::<T>::EVENT
        ) => DomainEvent {
            name: "asset_frozen".to_string(),
            data: decode_event_data(raw).map(AssetFrozen)?,
            meta,
        },
        (                               
            deip_assets::AssetThawedEvent::<T>::MODULE,
            deip_assets::AssetThawedEvent::<T>::EVENT
        ) => DomainEvent {
            name: "asset_thawed".to_string(),
            data: decode_event_data(raw).map(AssetThawed)?,
            meta,
        },
        (                               
            deip_assets::DestroyedEvent::<T>::MODULE,
            deip_assets::DestroyedEvent::<T>::EVENT
        ) => DomainEvent {
            name: "asset_class_destroyed".to_string(),
            data: decode_event_data(raw).map(AssetClassDestroyed)?,
            meta,
        },
        (                               
            deip_assets::ForceCreatedEvent::<T>::MODULE,
            deip_assets::ForceCreatedEvent::<T>::EVENT
        ) => DomainEvent {
            name: "asset_class_force_created".to_string(),
            data: decode_event_data(raw).map(AssetClassForceCreated)?,
            meta,
        },
        (                               
            deip_assets::MaxZombiesChangedEvent::<T>::MODULE,
            deip_assets::MaxZombiesChangedEvent::<T>::EVENT
        ) => DomainEvent {
            name: "asset_max_zombies_changed".to_string(),
            data: decode_event_data(raw).map(AssetMaxZombiesChanged)?,
            meta,
        },
        (                               
            deip_assets::MetadataSetEvent::<T>::MODULE,
            deip_assets::MetadataSetEvent::<T>::EVENT
        ) => DomainEvent {
            name: "asset_metadata_set".to_string(),
            data: decode_event_data(raw).map(AssetMetadataSet)?,
            meta,
        },
        _ => return Ok(None),
    };
    Ok(Some(event.into()))
}

fn decode_event_data<T: Decode>(e: &RawEvent) -> Result<T, codec::Error> {
    T::decode(&mut &e.data[..])
}
