//! # DEIP Assets Module
//! A module provides functionality of User Issued Assets.
//!
//! - [`Config`](./trait.Config.html)
//!
//! ## Overview
//! The pallet wraps Substrate [`pallet_assets`](../pallet_assets/index.html) and
//! adds additional constraints/features.
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! * [`create_asset`](./enum.Call.html#variant.create_asset)
//! * [`destroy`](./enum.Call.html#variant.destroy)
//! * [`issue_asset`](./enum.Call.html#variant.issue_asset)
//! * [`burn`](./enum.Call.html#variant.burn)
//! * [`transfer`](./enum.Call.html#variant.transfer)
//! * [`freeze`](./enum.Call.html#variant.freeze)
//! * [`thaw`](./enum.Call.html#variant.thaw)
//! * [`freeze_asset`](./enum.Call.html#variant.freeze_asset)
//! * [`thaw_asset`](./enum.Call.html#variant.thaw_asset)
//! * [`transfer_ownership`](./enum.Call.html#variant.transfer_ownership)
//! * [`set_team`](./enum.Call.html#variant.set_team)
//! * [`set_max_zombies`](./enum.Call.html#variant.set_max_zombies)
//! * [`set_metadata`](./enum.Call.html#variant.set_metadata)
//!
//! [`Config`]: ./trait.Config.html

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

pub mod traits;

pub mod serializable;
pub use serializable::{AssetBalance as SerializableAssetBalance, AssetId as SerializableAssetId};

#[doc(inline)]
pub use pallet::*;

#[frame_support::pallet]
#[doc(hidden)]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_support::{
        traits::{Currency, ExistenceRequirement, UnfilteredDispatchable, WithdrawReasons},
        transactional,
    };
    use frame_system::{pallet_prelude::*, RawOrigin};
    use sp_runtime::traits::{One, StaticLookup, Zero};
    use sp_std::{prelude::*, vec};

    #[cfg(feature = "std")]
    use frame_support::traits::GenesisBuild;

    use pallet_assets::WeightInfo;

    use super::traits::DeipProjectsInfo;

    type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
    type DeipProjectIdOf<T> =
        <<T as Config>::ProjectsInfo as DeipProjectsInfo<AccountIdOf<T>>>::ProjectId;
    type DeipInvestmentIdOf<T> =
        <<T as Config>::ProjectsInfo as DeipProjectsInfo<AccountIdOf<T>>>::InvestmentId;
    pub(crate) type AssetsAssetIdOf<T> = <T as pallet_assets::Config>::AssetId;
    pub(crate) type AssetsBalanceOf<T> = <T as pallet_assets::Config>::Balance;
    type AssetsWeightInfoOf<T> = <T as pallet_assets::Config>::WeightInfo;

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_assets::Config {
        type ProjectsInfo: DeipProjectsInfo<Self::AccountId>;
        type DeipAccountId: Into<Self::AccountId> + Parameter + Member;
    }

    #[doc(hidden)]
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[doc(hidden)]
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::error]
    pub enum Error<T> {
        ProjectDoesNotExist,
        ProjectDoesNotBelongToTeam,
        ProjectSecurityTokenCannotBeDestroyed,
        ProjectSecurityTokenCannotBeBurned,
        ProjectSecurityTokenCannotBeFreezed,
        ProjectSecurityTokenAccountCannotBeFreezed,
    }

    #[pallet::storage]
    pub(super) type AssetIdByProjectId<T: Config> =
        StorageMap<_, Identity, DeipProjectIdOf<T>, Vec<AssetsAssetIdOf<T>>, OptionQuery>;

    #[pallet::storage]
    pub(super) type ProjectIdByAssetId<T: Config> =
        StorageMap<_, Identity, AssetsAssetIdOf<T>, DeipProjectIdOf<T>, OptionQuery>;

    #[pallet::storage]
    pub(super) type CoreAssetId<T> = StorageValue<_, AssetsAssetIdOf<T>, ValueQuery>;

    #[pallet::storage]
    pub(super) type InvestmentsByAssetId<T: Config> =
        StorageMap<_, Identity, AssetsAssetIdOf<T>, DeipInvestmentIdOf<T>, OptionQuery>;

    #[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
    struct Investment<AccountId, AssetId> {
        creator: AccountId,
        assets: Vec<AssetId>,
    }

    #[pallet::storage]
    pub(super) type InvestmentMap<T: Config> = StorageMap<
        _,
        Identity,
        DeipInvestmentIdOf<T>,
        Investment<AccountIdOf<T>, AssetsAssetIdOf<T>>,
        OptionQuery,
    >;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub core_asset_admin: AccountIdOf<T>,
        pub core_asset_id: super::serializable::AssetId<T>,
        pub balances: Vec<(AccountIdOf<T>, super::serializable::AssetBalance<T>)>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                core_asset_admin: Default::default(),
                core_asset_id: Default::default(),
                balances: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            CoreAssetId::<T>::put(self.core_asset_id.0);

            let admin_source = T::Lookup::unlookup(self.core_asset_admin.clone());
            let call = pallet_assets::Call::<T>::create(
                self.core_asset_id.0,
                admin_source,
                u32::MAX,
                One::one(),
            );
            let result = call
                .dispatch_bypass_filter(RawOrigin::Signed(self.core_asset_admin.clone()).into());
            assert!(result.is_ok());

            // ensure no duplicates exist.
            let endowed_accounts = self
                .balances
                .iter()
                .map(|(x, _)| x)
                .cloned()
                .collect::<std::collections::BTreeSet<_>>();

            assert!(
                endowed_accounts.len() == self.balances.len(),
                "duplicate balances in genesis."
            );

            for (ref who, amount) in &self.balances {
                let who_source = <T::Lookup as StaticLookup>::unlookup(who.clone());
                let call =
                    pallet_assets::Call::<T>::mint(self.core_asset_id.0, who_source, amount.0);
                let result = call.dispatch_bypass_filter(
                    RawOrigin::Signed(self.core_asset_admin.clone()).into(),
                );
                assert!(result.is_ok());
            }
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn project_key(id: &DeipProjectIdOf<T>) -> T::AccountId {
            let entropy =
                (b"deip/projects/", id.as_ref()).using_encoded(sp_io::hashing::blake2_256);
            T::AccountId::decode(&mut &entropy[..]).unwrap_or_default()
        }

        pub fn investment_key(id: &DeipInvestmentIdOf<T>) -> T::AccountId {
            let entropy =
                (b"deip/investments/", id.as_ref()).using_encoded(sp_io::hashing::blake2_256);
            T::AccountId::decode(&mut &entropy[..]).unwrap_or_default()
        }

        pub fn try_get_tokenized_project(id: &T::AssetId) -> Option<DeipProjectIdOf<T>> {
            match ProjectIdByAssetId::<T>::try_get(*id) {
                Ok(project_id) => Some(project_id),
                Err(_) => None,
            }
        }

        #[transactional]
        pub fn transactionally_reserve(
            account: &T::AccountId,
            id: DeipInvestmentIdOf<T>,
            security_tokens_on_sale: &[(T::AssetId, T::Balance)],
        ) -> Result<(), deip_assets_error::ReserveError> {
            use deip_assets_error::ReserveError;

            let id_account = Self::investment_key(&id);
            let id_source = <T::Lookup as StaticLookup>::unlookup(id_account);

            let reserved = T::Currency::withdraw(
                account,
                T::Currency::minimum_balance(),
                WithdrawReasons.RESERVE,
                ExistenceRequirement::AllowDeath,
            )
            .map_err(|_| ReserveError::NotEnoughBalance)?;

            T::Currency::resolve_creating(&id_account, reserved);

            for (asset, amount) in security_tokens_on_sale {
                let call = pallet_assets::Call::<T>::transfer(*asset, id_source.clone(), *amount);
                let result = call.dispatch_bypass_filter(RawOrigin::Signed(account.clone()).into());
                if result.is_err() {
                    return Err(ReserveError::NotEnoughAsset);
                }
            }

            Ok(())
        }

        /// This could fail if the new project team is a zombie in pallet_assets-terms.
        #[transactional]
        pub fn transactionally_unreserve(
            project_id: DeipProjectIdOf<T>,
            account: &T::AccountId,
        ) -> Result<(), ()> {
            let project_account = Self::project_key(&project_id);
            let account_source = <T::Lookup as StaticLookup>::unlookup(account.clone());

            let security_tokens = match AssetIdByProjectId::<T>::try_get(project_id) {
                Err(_) => return Err(()),
                Ok(c) => c,
            };

            for id in &security_tokens {
                let amount = pallet_assets::Module::<T>::balance(*id, project_account.clone());
                if amount.is_zero() {
                    continue;
                }

                let call = pallet_assets::Call::<T>::transfer(*id, account_source.clone(), amount);
                let result =
                    call.dispatch_bypass_filter(RawOrigin::Signed(project_account.clone()).into());
                if result.is_err() {
                    return Err(());
                }
            }

            Ok(())
        }

        pub fn transfer_from_project(
            project_id: DeipProjectIdOf<T>,
            who: &T::AccountId,
            id: T::AssetId,
            amount: T::Balance,
        ) -> Result<(), ()> {
            let project_account = Self::project_key(&project_id);
            let account_source = <T::Lookup as StaticLookup>::unlookup(who.clone());

            let call = pallet_assets::Call::<T>::transfer(id, account_source, amount);
            let result =
                call.dispatch_bypass_filter(RawOrigin::Signed(project_account.clone()).into());
            if result.is_err() {
                return Err(());
            }

            Ok(())
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(AssetsWeightInfoOf::<T>::create())]
        pub(super) fn create_asset(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::AssetId,
            admin: T::DeipAccountId,
            max_zombies: u32,
            min_balance: AssetsBalanceOf<T>,
            project_id: Option<DeipProjectIdOf<T>>,
        ) -> DispatchResultWithPostInfo {
            if let Some(ref id) = project_id {
                match T::ProjectsInfo::try_get_project_team(id) {
                    None => return Err(Error::<T>::ProjectDoesNotExist.into()),
                    Some(team_id) => {
                        let account = ensure_signed(origin.clone())?;
                        ensure!(team_id == account, Error::<T>::ProjectDoesNotBelongToTeam)
                    }
                };
            }

            let admin_source = <T::Lookup as StaticLookup>::unlookup(admin.into());
            let call = pallet_assets::Call::<T>::create(id, admin_source, max_zombies, min_balance);
            let result = call.dispatch_bypass_filter(origin);
            if result.is_err() {
                return result;
            }

            if let Some(project_id) = project_id {
                ProjectIdByAssetId::<T>::insert(id, project_id.clone());
                AssetIdByProjectId::<T>::mutate_exists(project_id, |security_tokens| {
                    match security_tokens.as_mut() {
                        None => *security_tokens = Some(vec![id]),
                        Some(c) => c.push(id),
                    };
                });
            }

            result
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::destroy(*zombies_witness))]
        pub(super) fn destroy(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::AssetId,
            #[pallet::compact] zombies_witness: u32,
        ) -> DispatchResultWithPostInfo {
            ensure!(
                !ProjectIdByAssetId::<T>::contains_key(id),
                Error::<T>::ProjectSecurityTokenCannotBeDestroyed
            );

            let call = pallet_assets::Call::<T>::destroy(id, zombies_witness);
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::mint())]
        pub(super) fn issue_asset(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::AssetId,
            beneficiary: T::DeipAccountId,
            #[pallet::compact] amount: AssetsBalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let beneficiary_source = <T::Lookup as StaticLookup>::unlookup(beneficiary.into());
            let call = pallet_assets::Call::<T>::mint(id, beneficiary_source, amount);
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::burn())]
        pub(super) fn burn(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::AssetId,
            who: T::DeipAccountId,
            #[pallet::compact] amount: AssetsBalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            ensure!(
                !ProjectIdByAssetId::<T>::contains_key(id),
                Error::<T>::ProjectSecurityTokenCannotBeBurned
            );

            let who_source = <T::Lookup as StaticLookup>::unlookup(who.into());
            let call = pallet_assets::Call::<T>::burn(id, who_source, amount);
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::transfer())]
        pub(super) fn transfer(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::AssetId,
            target: T::DeipAccountId,
            #[pallet::compact] amount: AssetsBalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let target_source = <T::Lookup as StaticLookup>::unlookup(target.into());
            let call = pallet_assets::Call::<T>::transfer(id, target_source, amount);
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::freeze())]
        pub(super) fn freeze(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::AssetId,
            who: T::DeipAccountId,
        ) -> DispatchResultWithPostInfo {
            ensure!(
                !ProjectIdByAssetId::<T>::contains_key(id),
                Error::<T>::ProjectSecurityTokenAccountCannotBeFreezed
            );

            let who_source = <T::Lookup as StaticLookup>::unlookup(who.into());
            let call = pallet_assets::Call::<T>::freeze(id, who_source);
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::thaw())]
        pub(super) fn thaw(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::AssetId,
            who: T::DeipAccountId,
        ) -> DispatchResultWithPostInfo {
            let who_source = <T::Lookup as StaticLookup>::unlookup(who.into());
            let call = pallet_assets::Call::<T>::thaw(id, who_source);
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::freeze_asset())]
        pub(super) fn freeze_asset(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::AssetId,
        ) -> DispatchResultWithPostInfo {
            ensure!(
                !ProjectIdByAssetId::<T>::contains_key(id),
                Error::<T>::ProjectSecurityTokenCannotBeFreezed
            );

            let call = pallet_assets::Call::<T>::freeze_asset(id);
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::thaw_asset())]
        pub(super) fn thaw_asset(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::AssetId,
        ) -> DispatchResultWithPostInfo {
            let call = pallet_assets::Call::<T>::thaw_asset(id);
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::transfer_ownership())]
        pub(super) fn transfer_ownership(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::AssetId,
            owner: T::DeipAccountId,
        ) -> DispatchResultWithPostInfo {
            let owner_source = <T::Lookup as StaticLookup>::unlookup(owner.into());
            let call = pallet_assets::Call::<T>::transfer_ownership(id, owner_source);
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::set_team())]
        pub(super) fn set_team(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::AssetId,
            issuer: T::DeipAccountId,
            admin: T::DeipAccountId,
            freezer: T::DeipAccountId,
        ) -> DispatchResultWithPostInfo {
            let issuer_source = <T::Lookup as StaticLookup>::unlookup(issuer.into());
            let admin_source = <T::Lookup as StaticLookup>::unlookup(admin.into());
            let freezer_source = <T::Lookup as StaticLookup>::unlookup(freezer.into());
            let call =
                pallet_assets::Call::<T>::set_team(id, issuer_source, admin_source, freezer_source);
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::set_max_zombies())]
        pub(super) fn set_max_zombies(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::AssetId,
            #[pallet::compact] max_zombies: u32,
        ) -> DispatchResultWithPostInfo {
            let call = pallet_assets::Call::<T>::set_max_zombies(id, max_zombies);
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::set_metadata(name.len() as u32, symbol.len() as u32))]
        pub(super) fn set_metadata(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::AssetId,
            name: Vec<u8>,
            symbol: Vec<u8>,
            decimals: u8,
        ) -> DispatchResultWithPostInfo {
            let call = pallet_assets::Call::<T>::set_metadata(id, name, symbol, decimals);
            call.dispatch_bypass_filter(origin)
        }
    }
}

#[cfg(feature = "std")]
impl<T: Config> GenesisConfig<T> {
    /// Direct implementation of `GenesisBuild::build_storage`.
    ///
    /// Kept in order not to break dependency.
    pub fn build_storage(&self) -> Result<sp_runtime::Storage, String> {
        <Self as frame_support::traits::GenesisBuild<T>>::build_storage(self)
    }

    /// Direct implementation of `GenesisBuild::assimilate_storage`.
    ///
    /// Kept in order not to break dependency.
    pub fn assimilate_storage(&self, storage: &mut sp_runtime::Storage) -> Result<(), String> {
        <Self as frame_support::traits::GenesisBuild<T>>::assimilate_storage(self, storage)
    }
}
