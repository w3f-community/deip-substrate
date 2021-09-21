
use substrate_subxt::system::System;
use substrate_subxt::{module, Event};

use sp_std::prelude::*;
use codec::{Decode};
use frame_support::{Parameter};
use sp_runtime::traits::Member;

use serde::{Serialize, ser::{Serializer, SerializeStruct}};

#[module]
pub trait DeipProposal: System {
    type ProposalBatch: Parameter + Member;
    type InputProposalBatch: Parameter + Member;
    type ProposalId: Parameter + Member + Serialize;
    type Call: Parameter + Member;
    type BatchItem: Parameter + Member;
    type ProposalState: Parameter + Member + Serialize;
    /// Wrapper type to perform data transformations before serialization
    type WrappedBatch: Parameter + Member + Serialize;
    /// Wrapper type to perform data transformations before serialization
    type WrappedInputBatch: Parameter + Member + Serialize;
    /// Wrapper type to perform data transformations before serialization
    type WrappedCall: Parameter + Member + Serialize;
    
    fn wrap_batch<T: From<Self::WrappedBatch>>(batch: &Self::ProposalBatch) -> T;
    
    fn wrap_input_batch(batch: &Self::InputProposalBatch) -> Self::WrappedInputBatch;
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ProposedEvent<T: DeipProposal> {
    pub author: <T as System>::AccountId,
    pub batch: T::ProposalBatch,
    pub proposal_id: T::ProposalId,
}
impl<T: DeipProposal> Serialize for ProposedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_struct("ProposedEvent", 3)?;
        s.serialize_field("author", &self.author)?;
        s.serialize_field("batch", &T::wrap_batch::<T::WrappedBatch>(&self.batch))?;
        s.serialize_field("proposal_id", &self.proposal_id)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ApprovedEvent<T: DeipProposal> {
    pub member: <T as System>::AccountId,
    pub proposal_id: T::ProposalId,
}
impl<T: DeipProposal> Serialize for ApprovedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_struct("ApprovedEvent", 2)?;
        s.serialize_field("member", &self.member)?;
        s.serialize_field("proposal_id", &self.proposal_id)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct RevokedApprovalEvent<T: DeipProposal> {
    pub member: <T as System>::AccountId,
    pub proposal_id: T::ProposalId,
}
impl<T: DeipProposal> Serialize for RevokedApprovalEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_struct("RevokedApprovalEvent", 2)?;
        s.serialize_field("member", &self.member)?;
        s.serialize_field("proposal_id", &self.proposal_id)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ResolvedEvent<T: DeipProposal> {
    pub member: <T as System>::AccountId,
    pub proposal_id: T::ProposalId,
    pub state: T::ProposalState
}
impl<T: DeipProposal> Serialize for ResolvedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_struct("ResolvedEvent", 3)?;
        s.serialize_field("member", &self.member)?;
        s.serialize_field("proposal_id", &self.proposal_id)?;
        s.serialize_field("state", &self.state)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ExpiredEvent<T: DeipProposal> {
    pub proposal_id: T::ProposalId,
}
impl<T: DeipProposal> Serialize for ExpiredEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_struct("ExpiredEvent", 1)?;
        s.serialize_field("proposal_id", &self.proposal_id)?;
        s.end()
    }
}
