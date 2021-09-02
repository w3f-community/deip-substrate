use crate::*;

use sp_runtime::traits::Zero;

pub type Id = H160;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum Terms<AssetId, AssetBalance> {
    TechnologyLicenseAgreementTerms {
        source: ProjectId,
        price: (AssetId, AssetBalance),
    },
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum Agreement<AccountId, Hash, Moment, AssetId, AssetBalance> {
    None,
    TechnologyLicense(TechnologyLicenseStatus<AccountId, Hash, Moment, AssetId, AssetBalance>),
}

impl<AccountId, Hash, Moment, AssetId, AssetBalance> Default
    for Agreement<AccountId, Hash, Moment, AssetId, AssetBalance>
{
    fn default() -> Self {
        Agreement::None
    }
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct TechnologyLicense<AccountId, Hash, Moment, AssetId, AssetBalance> {
    id: Id,
    licenser: AccountId,
    licensee: AccountId,
    hash: Hash,
    start_time: Option<Moment>,
    end_time: Option<Moment>,
    project_id: ProjectId,
    price: (AssetId, AssetBalance),
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum TechnologyLicenseStatus<AccountId, Hash, Moment, AssetId, AssetBalance> {
    Unsigned(TechnologyLicense<AccountId, Hash, Moment, AssetId, AssetBalance>),
    SignedByLicenser(TechnologyLicense<AccountId, Hash, Moment, AssetId, AssetBalance>),
    Signed(TechnologyLicense<AccountId, Hash, Moment, AssetId, AssetBalance>),
}

impl<T: Config> Module<T> {
    pub(super) fn create_contract_agreement_impl(
        account: AccountIdOf<T>,
        id: Id,
        creator: AccountIdOf<T>,
        parties: Vec<T::DeipAccountId>,
        hash: HashOf<T>,
        start_time: Option<MomentOf<T>>,
        end_time: Option<MomentOf<T>>,
        terms: Terms<DeipAssetIdOf<T>, DeipAssetBalanceOf<T>>,
    ) -> DispatchResult {
        ensure!(account == creator, Error::<T>::NoPermission);
        ensure!(!parties.is_empty(), Error::<T>::ContractAgreementNoParties);

        for (i, party) in parties.iter().enumerate() {
            for other_party in parties.iter().skip(i + 1) {
                ensure!(
                    party != other_party,
                    Error::<T>::ContractAgreementDuplicateParties
                );
            }
        }

        let now = pallet_timestamp::Module::<T>::get();
        if let Some(s) = start_time {
            ensure!(
                now <= s,
                Error::<T>::ContractAgreementStartTimeMustBeLaterOrEqualCurrentMoment
            );
        }

        if let Some(e) = end_time {
            let start_time = match start_time {
                None => now,
                Some(s) => s,
            };

            ensure!(
                start_time < e,
                Error::<T>::ContractAgreementEndTimeMustBeLaterStartTime
            );
        }

        ensure!(
            !ContractAgreementMap::<T>::contains_key(id),
            Error::<T>::ContractAgreementAlreadyExists
        );
        match terms {
            Terms::TechnologyLicenseAgreementTerms { source, price } => {
                Self::create_project_license(
                    id, creator, parties, hash, start_time, end_time, source, price,
                )
            }
        }
    }

    fn create_project_license(
        id: Id,
        creator: AccountIdOf<T>,
        mut parties: Vec<T::DeipAccountId>,
        hash: HashOf<T>,
        start_time: Option<MomentOf<T>>,
        end_time: Option<MomentOf<T>>,
        project_id: ProjectId,
        price: (DeipAssetIdOf<T>, DeipAssetBalanceOf<T>),
    ) -> DispatchResult {
        let project =
            ProjectMap::<T>::try_get(project_id).map_err(|_| Error::<T>::NoSuchProject)?;

        ensure!(
            creator == project.team_id,
            Error::<T>::ProjectNotBelongToTeam
        );

        ensure!(
            price.1 > Zero::zero(),
            Error::<T>::ContractAgreementFeeMustBePositive
        );

        ensure!(
            parties.len() == 2,
            Error::<T>::ContractAgreementLicenseTwoPartiesRequired
        );

        let second: AccountIdOf<T> = parties.pop().unwrap().into();
        let first: AccountIdOf<T> = parties.pop().unwrap().into();
        let (licenser, licensee) = if first == project.team_id {
            (first, second)
        } else if second == project.team_id {
            (second, first)
        } else {
            return Err(Error::<T>::ContractAgreementLicenseNoLicenser.into());
        };

        let license = TechnologyLicense {
            id,
            licenser,
            licensee,
            hash,
            start_time,
            end_time,
            project_id,
            price,
        };

        ContractAgreementMap::<T>::insert(
            id,
            Agreement::TechnologyLicense(TechnologyLicenseStatus::Unsigned(license)),
        );

        Self::deposit_event(RawEvent::ContractAgreementCreated(id));

        Ok(())
    }

    pub(super) fn accept_contract_agreement_impl(
        account: AccountIdOf<T>,
        id: Id,
        party: AccountIdOf<T>,
    ) -> DispatchResult {
        ensure!(account == party, Error::<T>::NoPermission);

        let agreement = ContractAgreementMap::<T>::try_get(id)
            .map_err(|_| Error::<T>::ContractAgreementNotFound)?;

        match agreement {
            ContractAgreement::TechnologyLicense(status) => {
                Self::accept_project_license(party, status)
            }
            ContractAgreement::None => {
                Err(Error::<T>::ContractAgreementAcceptWrongAgreement.into())
            }
        }
    }

    fn accept_project_license(
        party: AccountIdOf<T>,
        status: TechnologyLicenseStatus<
            AccountIdOf<T>,
            HashOf<T>,
            MomentOf<T>,
            DeipAssetIdOf<T>,
            DeipAssetBalanceOf<T>,
        >,
    ) -> DispatchResult {
        match status {
            TechnologyLicenseStatus::Unsigned(license) => {
                Self::accept_project_license_by_licenser(party, license)
            }
            TechnologyLicenseStatus::SignedByLicenser(license) => {
                todo!();
            }
            TechnologyLicenseStatus::Signed(_) => {
                Err(Error::<T>::ContractAgreementLicenseAlreadyAccepted.into())
            }
        }
    }

    fn accept_project_license_by_licenser(
        licenser: AccountIdOf<T>,
        license: TechnologyLicense<
            AccountIdOf<T>,
            HashOf<T>,
            MomentOf<T>,
            DeipAssetIdOf<T>,
            DeipAssetBalanceOf<T>,
        >,
    ) -> DispatchResult {
        ensure!(
            licenser == license.licenser,
            Error::<T>::ContractAgreementLicensePartyIsNotLicenser
        );

        let now = pallet_timestamp::Module::<T>::get();
        match license.end_time {
            Some(end_time) => {
                ensure!(now <= end_time, Error::<T>::ContractAgreementLicenseExpired)
            }
            None => (),
        }

        let id = license.id;
        let status = TechnologyLicenseStatus::SignedByLicenser(license);
        ContractAgreementMap::<T>::insert(id, ContractAgreement::TechnologyLicense(status));

        Self::deposit_event(RawEvent::ContractAgreementAccepted(id, licenser));

        Ok(())
    }
}
