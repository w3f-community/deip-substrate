use std::fmt::Debug;

use substrate_subxt::{RawEvent, Event, system::System};
use codec::Decode;
use serde::{Serialize, ser::{Serializer, SerializeMap}};

use sp_runtime::generic::Block;
use sp_runtime::traits::{Block as _Block, Header as _Header};

use super::frame::{
    deip_proposal::{self, DeipProposal},
    deip::{self, Deip},
    deip_org::{self, DeipOrg}
};

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
    where T: Deip + DeipProposal + DeipOrg
{
    fn from(source: DomainEvent<T>) -> Self { Self::Domain(source) }
}

impl<T> From<InfrastructureEvent<T>> for SpecializedEvent<T>
    where T: Deip + DeipProposal + DeipOrg
{
    fn from(source: InfrastructureEvent<T>) -> Self { Self::Infrastructure(source) }
}

impl<T: DeipProposal + Deip + DeipOrg> Serialize for DomainEventData<T> {
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
            ProjectTokenSaleCreated(e) => e.serialize(serializer),
            ProjectTokenSaleActivated(e) => e.serialize(serializer),
            ProjectTokenSaleFinished(e) => e.serialize(serializer),
            ProjectTokenSaleExpired(e) => e.serialize(serializer),
            ProjectTokenSaleContributed(e) => e.serialize(serializer),
            // =============== DeipOrg:
            OrgCreate(e) => e.serialize(serializer),
            OrgTransferOwnership(e) => e.serialize(serializer),
        }
    }
}

pub use DomainEventData::*;

#[derive(Debug)]
pub enum DomainEventData<T: DeipProposal + Deip + DeipOrg> {
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
    ProjectTokenSaleCreated(deip::ProjectTokenSaleCreatedEvent<T>),
    ProjectTokenSaleActivated(deip::ProjectTokenSaleActivatedEvent<T>),
    ProjectTokenSaleFinished(deip::ProjectTokenSaleFinishedEvent<T>),
    ProjectTokenSaleExpired(deip::ProjectTokenSaleExpiredEvent<T>),
    ProjectTokenSaleContributed(deip::ProjectTokenSaleContributedEvent<T>),
    // DeipOrg:
    OrgCreate(deip_org::OrgCreateEvent<T>),
    OrgTransferOwnership(deip_org::OrgTransferOwnershipEvent<T>),
}

pub fn known_domain_events<T: DeipProposal + Deip + DeipOrg + Debug>(
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
            deip::ProjectTokenSaleCreatedEvent::<T>::MODULE,
            deip::ProjectTokenSaleCreatedEvent::<T>::EVENT
        ) => DomainEvent {
            name: "project_tokenSaleCreated".to_string(),
            data: decode_event_data(raw).map(ProjectTokenSaleCreated)?,
            meta,
        },
        (                               
            deip::ProjectTokenSaleActivatedEvent::<T>::MODULE,
            deip::ProjectTokenSaleActivatedEvent::<T>::EVENT
        ) => DomainEvent {
            name: "project_tokenSaleActivated".to_string(),
            data: decode_event_data(raw).map(ProjectTokenSaleActivated)?,
            meta,
        },
        (                               
            deip::ProjectTokenSaleFinishedEvent::<T>::MODULE,
            deip::ProjectTokenSaleFinishedEvent::<T>::EVENT
        ) => DomainEvent {
            name: "project_tokenSaleFinished".to_string(),
            data: decode_event_data(raw).map(ProjectTokenSaleFinished)?,
            meta,
        },
        (                               
            deip::ProjectTokenSaleExpiredEvent::<T>::MODULE,
            deip::ProjectTokenSaleExpiredEvent::<T>::EVENT
        ) => DomainEvent {
            name: "project_tokenSaleExpired".to_string(),
            data: decode_event_data(raw).map(ProjectTokenSaleExpired)?,
            meta,
        },
        (                               
            deip::ProjectTokenSaleContributedEvent::<T>::MODULE,
            deip::ProjectTokenSaleContributedEvent::<T>::EVENT
        ) => DomainEvent {
            name: "project_tokenSaleContributed".to_string(),
            data: decode_event_data(raw).map(ProjectTokenSaleContributed)?,
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
        _ => return Ok(None),
    };
    Ok(Some(event.into()))
}

fn decode_event_data<T: Decode>(e: &RawEvent) -> Result<T, codec::Error> {
    T::decode(&mut &e.data[..])
}
