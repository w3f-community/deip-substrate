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
//! * [`mint`](./enum.Call.html#variant.mint)
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
    use frame_system::pallet_prelude::*;
    use sp_std::prelude::*;
    use frame_support::traits::UnfilteredDispatchable;
    use sp_runtime::traits::StaticLookup;

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
    }
}
