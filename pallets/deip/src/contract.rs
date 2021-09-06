use crate::traits::DeipAssetSystem;
use crate::*;

use sp_runtime::{traits::Zero, Percent, SaturatedConversion};
use sp_std::vec;

pub type Id = H160;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum Terms<Asset> {
    TechnologyLicenseAgreementTerms {
        source: ProjectId,
        price: Asset,
    },
}

pub type TermsOf<T> = Terms<DeipAssetOf<T>>;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum Agreement<AccountId, Hash, Moment, Asset> {
    None,
    TechnologyLicense(TechnologyLicenseStatus<AccountId, Hash, Moment, Asset>),
}

pub type AgreementOf<T> = Agreement<AccountIdOf<T>, HashOf<T>, MomentOf<T>, DeipAssetOf<T>>;

impl<AccountId, Hash, Moment, Asset> Default
    for Agreement<AccountId, Hash, Moment, Asset>
{
    fn default() -> Self {
        Agreement::None
    }
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct TechnologyLicense<AccountId, Hash, Moment, Asset> {
    id: Id,
    licenser: AccountId,
    licensee: AccountId,
    hash: Hash,
    start_time: Option<Moment>,
    end_time: Option<Moment>,
    project_id: ProjectId,
    price: Asset,
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum TechnologyLicenseStatus<AccountId, Hash, Moment, Asset> {
    Unsigned(TechnologyLicense<AccountId, Hash, Moment, Asset>),
    SignedByLicenser(TechnologyLicense<AccountId, Hash, Moment, Asset>),
    Signed(TechnologyLicense<AccountId, Hash, Moment, Asset>),
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
        terms: TermsOf<T>,
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
        price: DeipAssetOf<T>,
    ) -> DispatchResult {
        let project =
            ProjectMap::<T>::try_get(project_id).map_err(|_| Error::<T>::NoSuchProject)?;

        ensure!(
            creator == project.team_id,
            Error::<T>::ProjectNotBelongToTeam
        );

        ensure!(
            price.amount > Zero::zero(),
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
            Agreement::TechnologyLicense(status) => {
                Self::accept_project_license(party, status)
            }
            Agreement::None => {
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
            DeipAssetOf<T>,
        >,
    ) -> DispatchResult {
        match status {
            TechnologyLicenseStatus::Unsigned(license) => {
                Self::accept_project_license_by_licenser(party, license)
            }
            TechnologyLicenseStatus::SignedByLicenser(license) => {
                Self::accept_project_license_by_licensee(party, license)
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
            DeipAssetOf<T>,
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
        ContractAgreementMap::<T>::insert(id, Agreement::TechnologyLicense(status));

        Self::deposit_event(RawEvent::ContractAgreementAccepted(id, licenser));

        Ok(())
    }

    fn accept_project_license_by_licensee(
        licensee: AccountIdOf<T>,
        license: TechnologyLicense<
            AccountIdOf<T>,
            HashOf<T>,
            MomentOf<T>,
            DeipAssetOf<T>,
        >,
    ) -> DispatchResult {
        ensure!(
            licensee == license.licensee,
            Error::<T>::ContractAgreementLicensePartyIsNotLicensee
        );

        let now = pallet_timestamp::Module::<T>::get();
        match license.end_time {
            Some(end_time) => {
                ensure!(now <= end_time, Error::<T>::ContractAgreementLicenseExpired)
            }
            None => (),
        }

        // this percent should be specified in the corresponding revenue stream
        let distribute_percent = Percent::from_percent(100);
        Self::distribute_revenue(
            &licensee,
            &license.price.id,
            &license.price.amount,
            distribute_percent,
            &license.project_id,
        )?;

        let id = license.id;
        let status = TechnologyLicenseStatus::Signed(license);
        ContractAgreementMap::<T>::insert(id, Agreement::TechnologyLicense(status));

        Self::deposit_event(RawEvent::ContractAgreementAccepted(id, licensee));

        Ok(())
    }

    fn distribute_revenue(
        from: &AccountIdOf<T>,
        asset: &DeipAssetIdOf<T>,
        fee: &DeipAssetBalanceOf<T>,
        distribute_percent: Percent,
        project_id: &ProjectId,
    ) -> DispatchResult {
        ensure!(
            T::AssetSystem::account_balance(&from, &asset) >= *fee,
            Error::<T>::ContractAgreementLicenseNotEnoughBalance
        );

        let fee_to_distribute = distribute_percent.mul_floor(*fee);

        let mut total_revenue: DeipAssetBalanceOf<T> = Zero::zero();
        let mut transfer_info = vec![];
        let beneficiary_tokens = T::AssetSystem::get_security_tokens(project_id);
        // simple model is used: if there are several security token classes then
        // the whole amount is distributed uniformly among the classes
        let token_count: u128 = beneficiary_tokens.len().saturated_into();
        for token in &beneficiary_tokens {
            let token_supply: u128 = T::AssetSystem::total_supply(token).saturated_into();
            let token_balances = if let Some(balances) = T::AssetSystem::get_security_token_balances(token) {
                balances
            } else {
                continue
            };

            for token_balance in &token_balances {
                let balance = T::AssetSystem::account_balance(&token_balance, token);
                let revenue: u128 = (fee_to_distribute * balance).saturated_into();
                let revenue: DeipAssetBalanceOf<T> =
                    (revenue / (token_supply * token_count)).saturated_into();
                if revenue.is_zero() {
                    continue;
                }

                transfer_info.push((revenue, token_balance.clone()));

                total_revenue += revenue;
            }
        }

        if total_revenue < *fee {
            // transfer the rest to the project team
            let project = ProjectMap::<T>::get(*project_id);
            transfer_info.push((*fee - total_revenue, project.team_id.clone()));
        }

        ensure!(
            T::AssetSystem::transactionally_transfer(from, *asset, &transfer_info).is_ok(),
            Error::<T>::ContractAgreementLicenseFailedToChargeFee
        );

        Ok(())
    }
}
