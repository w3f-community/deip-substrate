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

#[doc(inline)]
pub use pallet::*;

#[frame_support::pallet]
#[doc(hidden)]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_support::{traits::UnfilteredDispatchable, transactional};
    use frame_system::{pallet_prelude::*, RawOrigin};
    use sp_runtime::traits::{StaticLookup, Zero};
    use sp_std::prelude::*;

    use pallet_assets::WeightInfo;

    use super::traits::DeipProjectsInfo;

    type DeipProjectIdOf<T> = <<T as Config>::ProjectsInfo as DeipProjectsInfo>::ProjectId;
    type AssetsAssetIdOf<T> = <T as pallet_assets::Config>::AssetId;
    type AssetsBalanceOf<T> = <T as pallet_assets::Config>::Balance;
    type AssetsWeightInfoOf<T> = <T as pallet_assets::Config>::WeightInfo;

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_assets::Config {
        type ProjectsInfo: DeipProjectsInfo;
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
        ProjectSecurityTokenCannotBeDestroyed,
        ProjectSecurityTokenCannotBeBurned,
    }

    #[pallet::storage]
    pub(super) type AssetIdByProjectId<T: Config> =
        StorageMap<_, Identity, DeipProjectIdOf<T>, Vec<AssetsAssetIdOf<T>>, OptionQuery>;

    #[pallet::storage]
    pub(super) type ProjectIdByAssetId<T: Config> =
        StorageMap<_, Identity, AssetsAssetIdOf<T>, DeipProjectIdOf<T>, OptionQuery>;

    impl<T: Config> Pallet<T> {
        pub fn project_key(id: &DeipProjectIdOf<T>) -> T::AccountId {
            let entropy =
                (b"deip/projects/", id.as_ref()).using_encoded(sp_io::hashing::blake2_256);
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
            project_id: DeipProjectIdOf<T>,
            security_tokens_on_sale: &[(T::AssetId, T::Balance)],
        ) -> Result<(), ()> {
            let project_account = Self::project_key(&project_id);
            let project_source = <T::Lookup as StaticLookup>::unlookup(project_account);

            for (id, amount) in security_tokens_on_sale {
                let call = pallet_assets::Call::<T>::transfer(*id, project_source.clone(), *amount);
                let result = call.dispatch_bypass_filter(RawOrigin::Signed(account.clone()).into());
                if result.is_err() {
                    return Err(());
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
                let result = call.dispatch_bypass_filter(RawOrigin::Signed(project_account.clone()).into());
                if result.is_err() {
                    return Err(());
                }
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
            admin: <T::Lookup as StaticLookup>::Source,
            max_zombies: u32,
            min_balance: AssetsBalanceOf<T>,
            project_id: Option<DeipProjectIdOf<T>>,
        ) -> DispatchResultWithPostInfo {
            if let Some(ref id) = project_id {
                ensure!(T::ProjectsInfo::exists(id), Error::<T>::ProjectDoesNotExist);
            }

            let call = pallet_assets::Call::<T>::create(id, admin, max_zombies, min_balance);
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
            beneficiary: <T::Lookup as StaticLookup>::Source,
            #[pallet::compact] amount: AssetsBalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let call = pallet_assets::Call::<T>::mint(id, beneficiary, amount);
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::burn())]
        pub(super) fn burn(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::AssetId,
            who: <T::Lookup as StaticLookup>::Source,
            #[pallet::compact] amount: AssetsBalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            ensure!(
                !ProjectIdByAssetId::<T>::contains_key(id),
                Error::<T>::ProjectSecurityTokenCannotBeBurned
            );

            let call = pallet_assets::Call::<T>::burn(id, who, amount);
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::transfer())]
        pub(super) fn transfer(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::AssetId,
            target: <T::Lookup as StaticLookup>::Source,
            #[pallet::compact] amount: AssetsBalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let call = pallet_assets::Call::<T>::transfer(id, target, amount);
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::freeze())]
        pub(super) fn freeze(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::AssetId,
            who: <T::Lookup as StaticLookup>::Source,
        ) -> DispatchResultWithPostInfo {
            let call = pallet_assets::Call::<T>::freeze(id, who);
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::thaw())]
        pub(super) fn thaw(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::AssetId,
            who: <T::Lookup as StaticLookup>::Source,
        ) -> DispatchResultWithPostInfo {
            let call = pallet_assets::Call::<T>::thaw(id, who);
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::freeze_asset())]
        pub(super) fn freeze_asset(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::AssetId,
        ) -> DispatchResultWithPostInfo {
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
            owner: <T::Lookup as StaticLookup>::Source,
        ) -> DispatchResultWithPostInfo {
            let call = pallet_assets::Call::<T>::transfer_ownership(id, owner);
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::set_team())]
        pub(super) fn set_team(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::AssetId,
            issuer: <T::Lookup as StaticLookup>::Source,
            admin: <T::Lookup as StaticLookup>::Source,
            freezer: <T::Lookup as StaticLookup>::Source,
        ) -> DispatchResultWithPostInfo {
            let call = pallet_assets::Call::<T>::set_team(id, issuer, admin, freezer);
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
