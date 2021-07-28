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
//! * [`create`](./enum.Call.html#variant.create)
//! * [`destroy`](./enum.Call.html#variant.destroy)
//! * [`mint`](./enum.Call.html#variant.mint)
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
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

#[doc(inline)]
pub use pallet::*;

#[frame_support::pallet]
#[doc(hidden)]
pub mod pallet {
    use frame_support::dispatch::DispatchResult;
    use frame_support::pallet_prelude::*;
    use frame_support::traits::UnfilteredDispatchable;
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::StaticLookup;
    use sp_std::prelude::*;

    use pallet_assets::WeightInfo;

    /// Configuration trait
    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_assets::Config {}

    #[doc(hidden)]
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[doc(hidden)]
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::error]
    pub enum Error<T> {}

    type AssetsBalanceOf<T> = <T as pallet_assets::Config>::Balance;
    type AssetsWeightInfoOf<T> = <T as pallet_assets::Config>::WeightInfo;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(AssetsWeightInfoOf::<T>::create())]
        pub(super) fn create(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::AssetId,
            admin: <T::Lookup as StaticLookup>::Source,
            max_zombies: u32,
            min_balance: AssetsBalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let call = pallet_assets::Call::<T>::create(id, admin, max_zombies, min_balance);
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::destroy(*zombies_witness))]
        pub(super) fn destroy(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::AssetId,
            #[pallet::compact] zombies_witness: u32,
        ) -> DispatchResultWithPostInfo {
            let call = pallet_assets::Call::<T>::destroy(id, zombies_witness);
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::mint())]
        pub(super) fn mint(
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
