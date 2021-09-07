#![allow(dead_code)]

use super::{Deip, DeipAssets, DeipOrg, DeipProposal, DomainEventData};

use node_template_runtime::{Event, Runtime};

///
/// Code in this module is not intended to be called.
/// This is just for compile-time check that every Event in
/// the node Runtime has corresponding entities in `frame/{mod_name}.rs`
/// and corresponding variants in the enum DomainEventData.
///
/// The common recipe is the following:
/// 1. add corresponding entity to `frame/{mod_name}.rs`
/// 2. if the entity uses new types then register them in `types.rs`.
///     You have to set them in RuntimeT too; compiler will help
/// 3. add corresponding variant to DomainEventData
/// 4. add corresponding arm to match in the `known_domain_events`
/// 5. edit this file to settle the compile failure.
///

fn match_event<T>(e: &Event) -> DomainEventData<T>
where
    T: DeipProposal + Deip + DeipOrg + DeipAssets,
{
    match e {
        Event::pallet_deip_org(deip_org_event) => match_event_deip_org(deip_org_event),

        Event::pallet_deip(deip_event) => match_event_deip(deip_event),

        Event::pallet_deip_proposal(deip_proposal_event) => {
            match_event_deip_proposal(deip_proposal_event)
        }

        Event::pallet_assets(assets_event) => match_event_deip_assets(assets_event),

        Event::frame_system(_)
        | Event::pallet_utility(_)
        | Event::pallet_grandpa(_)
        | Event::pallet_balances(_)
        | Event::pallet_sudo(_)
        | Event::pallet_template(_)
        | Event::pallet_multisig(_) => unreachable!(),
    }
}

fn match_event_deip_org<T>(e: &pallet_deip_org::Event<Runtime>) -> DomainEventData<T>
where
    T: DeipProposal + Deip + DeipOrg + DeipAssets,
{
    use pallet_deip_org::Event::*;

    match e {
        OrgCreate(_) => {
            /* deip_org::OrgCreateEvent */
            unimplemented!()
        }
        OrgTransferOwnership(_) => {
            /* deip_org::OrgTransferOwnershipEvent */
            unimplemented!()
        }
        __Ignore(..) => unreachable!(),
    }
}

fn match_event_deip_proposal<T>(e: &pallet_deip_proposal::Event<Runtime>) -> DomainEventData<T>
where
    T: DeipProposal + Deip + DeipOrg + DeipAssets,
{
    use pallet_deip_proposal::Event::*;

    match e {
        Proposed { .. } => {
            /* deip_proposal::ProposedEvent */
            unimplemented!()
        }
        Approved { .. } => {
            /* deip_proposal::ApprovedEvent */
            unimplemented!()
        }
        RevokedApproval { .. } => {
            /* deip_proposal::RevokedApprovalEvent */
            unimplemented!()
        }
        Resolved { .. } => {
            /* deip_proposal::ResolvedEvent */
            unimplemented!()
        }
        Expired { .. } => {
            /* deip_proposal::ExpiredEvent */
            unimplemented!()
        }
        __Ignore(..) => unreachable!(),
    }
}

fn match_event_deip_assets<T>(
    e: &pallet_deip_assets::pallet_assets::Event<Runtime>,
) -> DomainEventData<T>
where
    T: DeipProposal + Deip + DeipOrg + DeipAssets,
{
    use pallet_deip_assets::pallet_assets::Event::*;

    match e {
        Created(..) => {
            /* deip_assets::CreatedEvent */
            unimplemented!()
        }
        Issued(..) => {
            /* deip_assets::IssuedEvent */
            unimplemented!()
        }
        Transferred(..) => {
            /* deip_assets::TransferredEvent */
            unimplemented!()
        }
        Burned(..) => {
            /* deip_assets::BurnedEvent */
            unimplemented!()
        }
        TeamChanged(..) => {
            /* deip_assets::TeamChangedEvent */
            unimplemented!()
        }
        OwnerChanged(..) => {
            /* deip_assets::OwnerChangedEvent */
            unimplemented!()
        }
        ForceTransferred(..) => {
            /* deip_assets::ForceTransferredEvent */
            unimplemented!()
        }
        Frozen(..) => {
            /* deip_assets::FrozenEvent */
            unimplemented!()
        }
        Thawed(..) => {
            /* deip_assets::ThawedEvent */
            unimplemented!()
        }
        AssetFrozen(..) => {
            /* deip_assets::AssetFrozenEvent */
            unimplemented!()
        }
        AssetThawed(..) => {
            /* deip_assets::AssetThawedEvent */
            unimplemented!()
        }
        Destroyed(..) => {
            /* deip_assets::DestroyedEvent */
            unimplemented!()
        }
        ForceCreated(..) => {
            /* deip_assets::ForceCreatedEvent */
            unimplemented!()
        }
        MaxZombiesChanged(..) => {
            /* deip_assets::MaxZombiesChangedEvent */
            unimplemented!()
        }
        MetadataSet(..) => {
            /* deip_assets::MetadataSetEvent */
            unimplemented!()
        }
        __Ignore(..) => unreachable!(),
    }
}

fn match_event_deip<T>(e: &pallet_deip::Event<Runtime>) -> DomainEventData<T>
where
    T: DeipProposal + Deip + DeipOrg + DeipAssets,
{
    use pallet_deip::RawEvent::*;

    match e {
        ProjectCreated(..) => {
            /* deip::ProjectCreatedEvent */
            unimplemented!()
        }
        ProjectRemoved(..) => {
            /* deip::ProjectRemovedEvent */
            unimplemented!()
        }
        ProjectUpdated(..) => {
            /* deip::ProjectUpdatedEvent */
            unimplemented!()
        }
        ProjectContnetCreated(..) => {
            /* deip::ProjectContentCreatedEvent */
            unimplemented!()
        }
        NdaCreated(..) => {
            /* deip::NdaCreatedEvent */
            unimplemented!()
        }
        NdaAccessRequestCreated(..) => {
            /* deip::NdaAccessRequestCreatedEvent */
            unimplemented!()
        }
        NdaAccessRequestFulfilled(..) => {
            /* deip::NdaAccessRequestFulfilledEvent */
            unimplemented!()
        }
        NdaAccessRequestRejected(..) => {
            /* deip::NdaAccessRequestRejectedEvent */
            unimplemented!()
        }
        DomainAdded(..) => {
            /* deip::DomainAddedEvent */
            unimplemented!()
        }
        ReviewCreated(..) => {
            /* deip::ReviewCreatedEvent */
            unimplemented!()
        }
        ReviewUpvoted(..) => {
            /* deip::ReviewUpvotedEvent */
            unimplemented!()
        }
        SimpleCrowdfundingCreated(..) => {
            /* deip::SimpleCrowdfundingCreatedEvent */
            unimplemented!()
        }
        SimpleCrowdfundingActivated(..) => {
            /* deip::SimpleCrowdfundingActivatedEvent */
            unimplemented!()
        }
        SimpleCrowdfundingFinished(..) => {
            /* deip::SimpleCrowdfundingFinishedEvent */
            unimplemented!()
        }
        SimpleCrowdfundingExpired(..) => {
            /* deip::SimpleCrowdfundingExpiredEvent */
            unimplemented!()
        }
        Invested(..) => {
            /* deip::InvestedEvent */
            unimplemented!()
        }
        ContractAgreementCreated(..) => {
            /* deip::ContractAgreementCreatedEvent */
            unimplemented!()
        }
        ContractAgreementAccepted(..) => {
            /* deip::ContractAgreementAcceptedEvent */
            unimplemented!()
        }
    }
}
