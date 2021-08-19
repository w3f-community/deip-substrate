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

#[derive(Debug)]
pub struct TypedEvent<T: Deip + DeipProposal + DeipOrg> {
    r#type: String,
    data: KnownEvents<T>,
    block: BlockMetadata<T>,
}

impl<T: Deip + DeipProposal + DeipOrg> Serialize for TypedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_map(None)?;
        s.serialize_entry("type", &self.r#type)?;
        s.serialize_entry("data", &self.data)?;
        s.serialize_entry("block", &self.block)?;
        s.end()
    }
}

impl<T: DeipProposal + Deip + DeipOrg> Serialize for KnownEvents<T> {
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

pub use KnownEvents::*;

#[derive(Debug)]
pub enum KnownEvents<T: DeipProposal + Deip + DeipOrg> {
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

pub fn known_events<T: DeipProposal + Deip + DeipOrg + Debug>(
    raw: &RawEvent,
    block: &Block<<T as System>::Header, <T as System>::Extrinsic>
)
    -> Result<Option<TypedEvent<T>>, codec::Error> 
{
    let block = BlockMetadata {
        number: block.header().number().to_owned(),
        hash: block.header().hash(),
        parent_hash: block.header().parent_hash().to_owned()
    };
    let event = match (raw.module.as_str(), raw.variant.as_str()) {
        // =========== DeipProposal:
        (
            deip_proposal::ProposedEvent::<T>::MODULE,
            deip_proposal::ProposedEvent::<T>::EVENT
        ) => TypedEvent {
            r#type: "proposal_proposed".to_string(),
            data: decode_event_data(raw).map(ProposalProposed)?,
            block,
        },
        (
            deip_proposal::ApprovedEvent::<T>::MODULE,
            deip_proposal::ApprovedEvent::<T>::EVENT
        ) => TypedEvent {
            r#type: "proposal_approved".to_string(),
            data: decode_event_data(raw).map(ProposalApproved)?,
            block,
        },
        (
            deip_proposal::RevokedApprovalEvent::<T>::MODULE,
            deip_proposal::RevokedApprovalEvent::<T>::EVENT
        ) => TypedEvent {
            r#type: "proposal_revokedApproval".to_string(),
            data: decode_event_data(raw).map(ProposalRevokedApproval)?,
            block,
        },
        (
            deip_proposal::ResolvedEvent::<T>::MODULE,
            deip_proposal::ResolvedEvent::<T>::EVENT
        ) => TypedEvent {
            r#type: "proposal_resolved".to_string(),
            data: decode_event_data(raw).map(ProposalResolved)?,
            block,
        },
        (
            deip_proposal::ExpiredEvent::<T>::MODULE,
            deip_proposal::ExpiredEvent::<T>::EVENT
        ) => TypedEvent {
            r#type: "proposal_expired".to_string(),
            data: decode_event_data(raw).map(ProposalExpired)?,
            block,
        },
        // =========== Deip:
        (
            deip::ProjectCreatedEvent::<T>::MODULE,
            deip::ProjectCreatedEvent::<T>::EVENT
        ) => TypedEvent {
            r#type: "project_created".to_string(),
            data: decode_event_data(raw).map(ProjectCreated)?,
            block,
        },   
        (                               
            deip::ProjectRemovedEvent::<T>::MODULE,
            deip::ProjectRemovedEvent::<T>::EVENT
        ) => TypedEvent {
            r#type: "project_removed".to_string(),
            data: decode_event_data(raw).map(ProjectRemoved)?,
            block,
        },
        (                               
            deip::ProjectUpdatedEvent::<T>::MODULE,
            deip::ProjectUpdatedEvent::<T>::EVENT
        ) => TypedEvent {
            r#type: "project_updated".to_string(),
            data: decode_event_data(raw).map(ProjectUpdated)?,
            block,
        },
        (                               
            deip::ProjectContentCreatedEvent::<T>::MODULE,
            deip::ProjectContentCreatedEvent::<T>::EVENT
        ) => TypedEvent {
            r#type: "project_contentCreated".to_string(),
            data: decode_event_data(raw).map(ProjectContentCreated)?,
            block,
        },
        (                               
            deip::NdaCreatedEvent::<T>::MODULE,
            deip::NdaCreatedEvent::<T>::EVENT
        ) => TypedEvent {
            r#type: "project_ndaCreated".to_string(),
            data: decode_event_data(raw).map(NdaCreated)?,
            block,
        },
        (                               
            deip::NdaAccessRequestCreatedEvent::<T>::MODULE,
            deip::NdaAccessRequestCreatedEvent::<T>::EVENT
        ) => TypedEvent {
            r#type: "project_ndaAccessRequestCreated".to_string(),
            data: decode_event_data(raw).map(NdaAccessRequestCreated)?,
            block,
        },
        (                               
            deip::NdaAccessRequestFulfilledEvent::<T>::MODULE,
            deip::NdaAccessRequestFulfilledEvent::<T>::EVENT
        ) => TypedEvent {
            r#type: "project_ndaAccessRequestFulfilled".to_string(),
            data: decode_event_data(raw).map(NdaAccessRequestFulfilled)?,
            block,
        },
        (                               
            deip::NdaAccessRequestRejectedEvent::<T>::MODULE,
            deip::NdaAccessRequestRejectedEvent::<T>::EVENT
        ) => TypedEvent {
            r#type: "project_ndaAccessRequestRejected".to_string(),
            data: decode_event_data(raw).map(NdaAccessRequestRejected)?,
            block,
        },
        (                               
            deip::DomainAddedEvent::<T>::MODULE,
            deip::DomainAddedEvent::<T>::EVENT
        ) => TypedEvent {
            r#type: "project_domainAdded".to_string(),
            data: decode_event_data(raw).map(DomainAdded)?,
            block,
        },
        (                               
            deip::ReviewCreatedEvent::<T>::MODULE,
            deip::ReviewCreatedEvent::<T>::EVENT
        ) => TypedEvent {
            r#type: "project_reviewCreated".to_string(),
            data: decode_event_data(raw).map(ReviewCreated)?,
            block,
        },
        (                               
            deip::ProjectTokenSaleCreatedEvent::<T>::MODULE,
            deip::ProjectTokenSaleCreatedEvent::<T>::EVENT
        ) => TypedEvent {
            r#type: "project_tokenSaleCreated".to_string(),
            data: decode_event_data(raw).map(ProjectTokenSaleCreated)?,
            block,
        },
        (                               
            deip::ProjectTokenSaleActivatedEvent::<T>::MODULE,
            deip::ProjectTokenSaleActivatedEvent::<T>::EVENT
        ) => TypedEvent {
            r#type: "project_tokenSaleActivated".to_string(),
            data: decode_event_data(raw).map(ProjectTokenSaleActivated)?,
            block,
        },
        (                               
            deip::ProjectTokenSaleFinishedEvent::<T>::MODULE,
            deip::ProjectTokenSaleFinishedEvent::<T>::EVENT
        ) => TypedEvent {
            r#type: "project_tokenSaleFinished".to_string(),
            data: decode_event_data(raw).map(ProjectTokenSaleFinished)?,
            block,
        },
        (                               
            deip::ProjectTokenSaleExpiredEvent::<T>::MODULE,
            deip::ProjectTokenSaleExpiredEvent::<T>::EVENT
        ) => TypedEvent {
            r#type: "project_tokenSaleExpired".to_string(),
            data: decode_event_data(raw).map(ProjectTokenSaleExpired)?,
            block,
        },
        (                               
            deip::ProjectTokenSaleContributedEvent::<T>::MODULE,
            deip::ProjectTokenSaleContributedEvent::<T>::EVENT
        ) => TypedEvent {
            r#type: "project_tokenSaleContributed".to_string(),
            data: decode_event_data(raw).map(ProjectTokenSaleContributed)?,
            block,
        },
        // =========== DeipOrg:
        (                               
            deip_org::OrgCreateEvent::<T>::MODULE,
            deip_org::OrgCreateEvent::<T>::EVENT
        ) => TypedEvent {
            r#type: "dao_create".to_string(),
            data: decode_event_data(raw).map(OrgCreate)?,
            block,
        },
        (                               
            deip_org::OrgTransferOwnershipEvent::<T>::MODULE,
            deip_org::OrgTransferOwnershipEvent::<T>::EVENT
        ) => TypedEvent {
            r#type: "dao_transferOwnership".to_string(),
            data: decode_event_data(raw).map(OrgTransferOwnership)?,
            block,
        },
        _ => return Ok(None),
    };
    Ok(Some(event))
}

fn decode_event_data<T: Decode>(e: &RawEvent) -> Result<T, codec::Error> {
    T::decode(&mut &e.data[..])
}
