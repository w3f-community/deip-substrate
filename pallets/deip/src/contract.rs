use crate::*;

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

impl<T: Config> Module<T> {
    pub(super) fn create_contract_agreement_impl(
        account: AccountIdOf<T>,
        id: ContractAgreementId,
        creator: AccountIdOf<T>,
        parties: Vec<T::DeipAccountId>,
        hash: HashOf<T>,
        start_time: Option<MomentOf<T>>,
        end_time: Option<MomentOf<T>>,
        terms: ContractAgreementTerms<DeipAssetIdOf<T>, DeipAssetBalanceOf<T>>,
    ) -> DispatchResult {
        ensure!(account == creator, Error::<T>::NoPermission);
        ensure!(!parties.is_empty(), Error::<T>::ContractAgreementNoParties);

        todo!();
    }
}
