//! # DEIP Portal Module
//! A module for manage Portals
//! 
//! - [`Config`](./trait.Config.html)
//! - [`Call`](./enum.Call.html)
//!
//! ## Overview
//! A module for manage Portals
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! * `create` - Create a DAO.
//! * `alter_authority` - Alter DAO's authority.
//! * `on_behalf` - Perform action on behalf of a DAO.
//!
//! [`Call`]: ./enum.Call.html
//! [`Config`]: ./trait.Config.html

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

#[cfg(test)]
mod tests;
mod extensions;
mod transaction_ctx;


#[doc(inline)]
pub use pallet::*;
pub use extensions::*;

#[frame_support::pallet]
#[doc(hidden)]
pub mod pallet {
    use frame_system::pallet_prelude::*;
    use frame_system::RawOrigin;
    
    use frame_support::pallet_prelude::*;
    use frame_support::{Hashable};
    use frame_support::weights::{PostDispatchInfo, GetDispatchInfo};
    
    use frame_support::traits::{UnfilteredDispatchable, IsSubType};
    
    use sp_std::prelude::*;
    use sp_std::collections::{btree_map::BTreeMap};
    use sp_std::iter::FromIterator;
    
    use sp_runtime::{MultiSigner, traits::{Dispatchable, IdentifyAccount}, DispatchResultWithInfo};
    use frame_support::dispatch::DispatchResult;
    
    use pallet_deip_toolkit::storage_ops::StorageOpsTransaction;
    
    pub trait PortalProvider {
        type Portal;
        fn provide() -> Self::Portal;
    }
    impl PortalProvider for () {
        type Portal = ();

        fn provide() -> Self::Portal {}
    }

    /// Configuration trait
    #[pallet::config]
    pub trait Config: frame_system::Config {
        
        type PortalId: Member + Parameter + Default;
        type Portal;
        type PortalProvider: PortalProvider<Portal = Self::Portal>;
        
        type Call: Parameter +
             Dispatchable<Origin = Self::Origin, PostInfo = PostDispatchInfo> +
             GetDispatchInfo +
             From<frame_system::pallet::Call<Self>> +
             UnfilteredDispatchable<Origin = Self::Origin> +
             frame_support::dispatch::Codec + 
             IsSubType<Call<Self>>;
    }
    
    #[doc(hidden)]
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);
    
    #[doc(hidden)]
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
    
    #[pallet::error]
    pub enum Error<T> {}
    
    // #[pallet::event]
    // #[pallet::metadata(u32 = "SpecialU32")]
    // #[pallet::generate_deposit(fn deposit_event)]
    // pub enum Event<T: Config> {}
    
    #[doc(hidden)]
    #[pallet::genesis_config]
    #[derive(Default)]
    pub struct GenesisConfig {}
    
    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig {
        fn build(&self) {}
    }
    
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn on_behalf(
            origin: OriginFor<T>,
            portal_id: T::PortalId,
            call: Box<<T as Config>::Call>,
        )
            -> DispatchResultWithPostInfo
        {
            ctx_enter::<T>(portal_id);
            let r = call.dispatch(origin);
            ctx_exit::<T>();
            r
        }
    }
    
    // ==== Storage ====:
    
    pub type ExtrinsicIndex = u32;
    
    #[pallet::storage]
    pub(super) type PortalTag<T: Config> = StorageDoubleMap<_,
        Twox64Concat,
        BlockNumberFor<T>,
        Blake2_128Concat,
        T::PortalId,
        Vec<ExtrinsicIndex>,
        OptionQuery
    >;
    
    #[pallet::storage]
    pub(super) type PortalCtx<T: Config> = StorageValue<_, T::PortalId>;
    
    pub fn ctx_enter<T: Config>(ctx: T::PortalId) {
        PortalCtx::<T>::put(ctx);
    }
    pub fn ctx_exit<T: Config>() -> Option<T::PortalId> {
        PortalCtx::<T>::take()
    }
    pub fn ctx_get<T: Config>() -> Option<T::PortalId> {
        PortalCtx::<T>::get()
    }
    
    use storage_ops::*;
    #[doc(no_inline)]
    /// Module contains abstractions over pallet storage operations
    pub mod storage_ops {
        use sp_std::prelude::*;
        use pallet_deip_toolkit::storage_ops::StorageOp;
        use super::{Config, Pallet};
        // 
        // /// Storage operations
        // pub enum StorageOps<T: Config> {
        //     /// Deposit event
        //     DepositEvent(Event<T>),
        //     /// Create DAO
        //     CreateDao(DaoOf<T>),
        //     /// Update DAO
        //     UpdateDao(DaoOf<T>),
        //     
        // }
        // impl<T: Config> StorageOp for StorageOps<T> {
        //     fn exec(self) {
        //         match self {
        //             Self::DepositEvent(e) => {
        //                 Pallet::<T>::deposit_event(e)
        //             },
        //             Self::CreateDao(dao) => {
        //                 DaoLookup::<T>::insert(dao.dao_key().clone(), dao.id().clone());
        //                 DaoRepository::<T>::insert(*dao.id(), dao);
        //             }
        //             Self::UpdateDao(dao) => {
        //                 DaoRepository::<T>::insert(*dao.id(), dao);
        //             }
        //         }
        //     }
        // }
    }
}
