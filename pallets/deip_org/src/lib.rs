//! # DEIP Org Module
//! A module for manage virtual organisations and perform actions on behalf of it
//! 
//! - [`Config`](./trait.Config.html)
//! - [`Call`](./enum.Call.html)
//!
//! ## Overview
//! A module for manage virtual organisations and perform actions on behalf of it
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! * `create` - Create an Org.
//! * `transfer_ownership` - Transfer ownership of an Org to another account.
//! * `on_behalf` - Perform action on behalf of an Org.
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

pub mod api;

#[doc(inline)]
pub use pallet::*;

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
    
    use sp_runtime::{MultiSigner, traits::{Dispatchable, IdentifyAccount}};
    use frame_support::dispatch::DispatchResult;
    
    // use sp_core::crypto::Pair;
    // use sp_core::ed25519;
    
    use pallet_deip_toolkit::storage_ops::StorageOpsTransaction;

    /// Configuration trait
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Type represents events
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        /// Type represents particular call from batch-transaction 
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
    pub enum Error<T> {
        /// Already exists (unique by `name`)
        Exists,
        /// Not found
        NotFound,
        /// Access denied
        Forbidden,
        ///
        AuthorityMismatch
    }
    
    #[pallet::event]
    #[pallet::metadata(u32 = "SpecialU32")]
    #[pallet::generate_deposit(fn deposit_event)]
    pub enum Event<T: Config> {
        /// Emits when organisation created
        OrgCreate(OrgOf<T>),
        /// Emits when authority alteration
        OrgAlterAuthority(OrgOf<T>),
    }
    
    #[doc(hidden)]
    #[pallet::genesis_config]
    #[derive(Default)]
    pub struct GenesisConfig {}
    
    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig {
        fn build(&self) {}
    }
    
    use org::*;
    pub mod org {
        use sp_std::prelude::*;
        use frame_support::pallet_prelude::*;
        use super::{Config, OrgRepository, Error, OrgLookup};
        
        #[cfg(feature = "std")]
        use serde::{Serialize, Deserialize};
        use frame_system::Key;
        use crate::Error::AuthorityMismatch;
        use codec::Codec;

        #[allow(type_alias_bounds)]
        pub type OrgOf<T: Config> = Org<T::AccountId, OrgName>;
        pub type OrgName = sp_core::H160;
        
        pub enum KeyType<'a, K> {
            Members(&'a K),
            Own(&'a K)
        }
        impl<'a, K> KeyType<'a, K> {
            pub fn members(k: &'a K) -> Self { Self::Members(k) }
            pub fn own(k: &'a K) -> Self { Self::Own(k) }
        }
        pub trait MatchKey<T: Config> {
            fn match_key(&self, org: &OrgOf<T>) -> bool;
        }
        impl<T: Config> MatchKey<T> for KeyType<'_, T::AccountId> {
            fn match_key(&self, org: &OrgOf<T>) -> bool {
                match self {
                    Self::Members(k) => {
                        *k == org.authority_key()
                    },
                    Self::Own(k) => {
                        *k == org.org_key()
                    },
                }
            }
        }
        
        pub enum LoadBy<'a, AccountId> {
            Name { name: &'a OrgName, who: KeyType<'a, AccountId> },
            OrgKey { org_key: &'a AccountId }
        }
        
        pub fn load_org<T: Config>(
            q: LoadBy<'_, T::AccountId>,
        )
            -> Result<OrgOf<T>, Error<T>>
        {
            let (org, who) = match q {
                LoadBy::Name { name, who } => {
                    let org = OrgRepository::<T>::get(name)
                        .ok_or(Error::<T>::NotFound)?;
                    (org, who)
                },
                LoadBy::OrgKey { org_key } => {
                    let name = OrgLookup::<T>::get(org_key)
                        .ok_or(Error::<T>::NotFound)?;
                    let org = OrgRepository::<T>::get(&name)
                        .ok_or(Error::<T>::NotFound)?;
                    (org, KeyType::Own(org_key))
                },
            };
            ensure!(MatchKey::<T>::match_key(&who, &org), Error::<T>::Forbidden);
            Ok(org)
        }
        
        #[derive(Debug, Clone, Eq, PartialEq, Encode, Decode)]
        #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
        pub struct Authority<AccountId> {
            signatories: Vec<AccountId>,
            threshold: u16 
        }
        #[derive(Debug, Clone, Eq, PartialEq, Encode, Decode)]
        #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
        pub struct InputAuthority<AccountId> {
            pub signatories: Vec<AccountId>,
            pub threshold: u16 
        }
        pub enum AuthorityAssert {
            EmptySignatories,
            /// We expect signatures list with exactly one element for plain account
            PlainAccountExpect,
            ThresholdMismatch,
            KeyMismatch,
        }
        impl<T: Config> From<AuthorityAssert> for Error<T> {
            fn from(source: AuthorityAssert) -> Self {
                Error::<T>::AuthorityMismatch
            }
        }
        pub trait AssertAuthority<T: Config> {
            // fn assert(self, origin: &T::AccountId) -> Result<Authority<T::AccountId>, AuthorityAssert>;
        }
        fn multi_account_id<T: Codec + Default>(who: &[T], threshold: u16) -> T {
            let entropy = (b"modlpy/utilisuba", who, threshold).using_encoded(sp_io::hashing::blake2_256);
            T::decode(&mut &entropy[..]).unwrap_or_default()
        }
        impl<T: Config> AssertAuthority<T> for InputAuthority<T::AccountId> {
            
        }
        impl<AccountId: Codec + Default + Clone> Authority<AccountId> {
            pub fn authority_key(&self) -> AccountId {
                if self.threshold == 0 { return unsafe { self.signatories.get_unchecked(0).clone() } }
                multi_account_id::<AccountId>(&self.signatories[..], self.threshold)
            }
        }
        impl<AccountId: Ord + Eq + PartialEq> Authority<AccountId> {
            pub fn add_member(&mut self, member: AccountId) {
                if let Err(pos) = self.signatories.binary_search(&member) {
                    self.signatories.insert(pos, member);
                    self.threshold += 1;
                }
            }
            pub fn remove_member(&mut self, member: AccountId) {
                if self.signatories.len() == 1 { return }
                if let Ok(pos) = self.signatories.binary_search(&member) {
                    self.signatories.remove(pos);
                    if self.signatories.len() == 1 {
                        self.threshold = 0;
                        return
                    }
                    if self.signatories.len() > 1 && (self.threshold - 1) > 0 {
                        self.threshold -= 1;
                    }
                }
            }
        }
        impl<AccountId: Codec + Default + Clone + Ord + Eq + PartialEq> InputAuthority<AccountId> {
            pub(crate) fn assert(self, authority_key: &AccountId) -> Result<Authority<AccountId>, AuthorityAssert>
            {
                let Self { mut signatories, threshold } = self;
                ensure!(!signatories.is_empty(), AuthorityAssert::EmptySignatories);
                signatories.sort();
                signatories.dedup_by(|x, y| x == y);
                
                // zero threshold adjusts plain non-multisig account
                if threshold == 0 {
                    ensure!(signatories.len() == 1, AuthorityAssert::PlainAccountExpect);
                } else {
                    ensure!(threshold as usize <= signatories.len(), AuthorityAssert::ThresholdMismatch);
                };
                
                let authority = Authority { signatories, threshold };
                ensure!(authority_key == &authority.authority_key(), AuthorityAssert::KeyMismatch);
                
                Ok(authority)
            }
        }
        
        #[derive(Debug, Clone, Eq, PartialEq, Encode, Decode)]
        #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
        pub struct Org<AccountId, Name> {
            /// Authority aka "control" key. Not fixed, may changes in future
            authority_key: AccountId,
            /// Details of control key: multi-sig or plain account
            authority: Authority<AccountId>,
            /// Unique organisation name aka ID
            name: Name,
            /// Own key of organization for keeping assets,
            /// to signing of an extrinsics calls dispatched on behalf of organization etc ..
            /// Must be generated internally when organisation will be created,
            /// nobody knows private half of this key
            org_key: AccountId
        }
        impl<AccountId, Name> Org<AccountId, Name> {
            pub fn new(
                members_key: AccountId,
                members_key_source: Authority<AccountId>,
                name: Name,
                org_key: AccountId,
            )
                -> Self
            {
                Self { authority_key: members_key, authority: members_key_source, name, org_key }
            }
            
            pub fn authority_key(&self) -> &AccountId { &self.authority_key }
            pub fn authority(&self) -> &Authority<AccountId>{ &self.authority }
            pub fn name(&self) -> &Name { &self.name }
            pub fn org_key(&self) -> &AccountId { &self.org_key }
            
            pub fn alter_authoriry(self, op: AlterAuthority<AccountId>) -> Result<Self, AuthorityAssert>
                where
                    AccountId: Codec + Default + Clone + Ord + Eq + PartialEq            {
                let Self {
                    authority_key: _,
                    mut authority,
                    name,
                    org_key } = self;
                match op {
                    AlterAuthority::AddMember { member } => {
                        authority.add_member(member);
                    },
                    AlterAuthority::RemoveMember { member } => {
                        authority.remove_member(member);
                    },
                    AlterAuthority::ReplaceAuthority { authority_key: new_authority_key, authority: new_authority } => {
                        authority = new_authority.assert(&new_authority_key)?;
                    },
                }
                Ok(Self::new(authority.authority_key(), authority, name, org_key))
            }
        }
        
        #[derive(Debug, Clone, Eq, PartialEq, Encode, Decode)]
        #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
        #[cfg_attr(feature = "std", serde(tag = "operation", content = "data"))]
        pub enum AlterAuthority<AccountId> {
            AddMember { member: AccountId },
            RemoveMember { member: AccountId},
            ReplaceAuthority { authority_key: AccountId, authority: InputAuthority<AccountId> }
        }
    }
    
    impl<T: Config> Pallet<T> {
        pub fn org_key(org_name: &OrgName) -> T::AccountId {
            org_key::<T::AccountId>(org_name)
        }
    }
    pub fn org_key<T: Decode + Default>(org_name: &OrgName) -> T {
        let entropy = (b"deip/DAOs/", org_name.as_bytes()).using_encoded(sp_io::hashing::blake2_256);
        T::decode(&mut &entropy[..]).unwrap_or_default()
    }
    pub fn org_key2<T: frame_system::Config>(org_name: &OrgName) -> T::AccountId {
        org_key::<T::AccountId>(org_name)
    }
    
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        
        #[pallet::weight(10_000)]
        pub fn create(
            origin: OriginFor<T>,
            name: OrgName,
            authority: InputAuthority<T::AccountId>,
        )
            -> DispatchResultWithPostInfo
        {
            let authority_key = ensure_signed(origin)?;
            let authority = authority.assert(&authority_key).map_err::<Error<T>, _>(Into::into)?;
            ensure!(!OrgRepository::<T>::contains_key(&name), Error::<T>::Exists);
            let org_key = Self::org_key(&name);
            let org = OrgOf::<T>::new(
                authority_key,
                authority,
                name,
                org_key
            );
            StorageOpsTransaction::<StorageOps<T>>::new()
                .commit(move |ops| {
                    ops.push_op(StorageOps::CreateOrg(org.clone()));
                    ops.push_op(StorageOps::DepositEvent(Event::<T>::OrgCreate(org)));
                });
            Ok(Some(0).into())
        }
        
        #[pallet::weight(10_000)]
        pub fn alter_authority(
            origin: OriginFor<T>,
            alter_authority: AlterAuthority<T::AccountId>,
        )
            -> DispatchResultWithPostInfo
        {
            let who = ensure_signed(origin)?;
            let mut org = load_org::<T>(LoadBy::OrgKey { org_key: &who })?;
            org = org.alter_authoriry(alter_authority).map_err::<Error<T>, _>(Into::into)?;
            StorageOpsTransaction::<StorageOps<T>>::new()
                .commit(move |ops| {
                    ops.push_op(StorageOps::UpdateOrg(org.clone()));
                    ops.push_op(StorageOps::DepositEvent(Event::<T>::OrgAlterAuthority(org)));
                });
            Ok(Some(0).into())
        }
        
        #[pallet::weight(10_000)]
        pub fn on_behalf(
            origin: OriginFor<T>,
            name: OrgName,
            call: Box<<T as Config>::Call>,
        ) -> DispatchResultWithPostInfo
        {
            let who = ensure_signed(origin)?;
            let org = load_org::<T>(LoadBy::Name {
                name: &name,
                who: KeyType::Members(&who)
            })?;
            call.dispatch(RawOrigin::Signed(org.org_key().clone()).into())
        }
    }
    
    // ==== Storage ====:
    
    #[pallet::storage]
    #[pallet::getter(fn get_org)]
    pub(super) type OrgRepository<T: Config> = StorageMap<_,
        Blake2_128Concat,
        OrgName,
        OrgOf<T>,
        OptionQuery
    >;
    
    #[pallet::storage]
    #[pallet::getter(fn lookup_org)]
    pub(super) type OrgLookup<T: Config> = StorageMap<_,
        Blake2_128Concat,
        T::AccountId,
        OrgName,
        OptionQuery
    >;
    
    use storage_ops::*;
    #[doc(no_inline)]
    /// Module contains abstractions over pallet storage operations
    pub mod storage_ops {
        use sp_std::prelude::*;
        use pallet_deip_toolkit::storage_ops::StorageOp;
        use super::{Config, Event, Pallet};
        use super::{OrgOf, OrgRepository, OrgLookup};

        /// Storage operations
        pub enum StorageOps<T: Config> {
            /// Deposit event
            DepositEvent(Event<T>),
            /// Create org
            CreateOrg(OrgOf<T>),
            /// Update org
            UpdateOrg(OrgOf<T>),
            
        }
        impl<T: Config> StorageOp for StorageOps<T> {
            fn exec(self) {
                match self {
                    Self::DepositEvent(e) => {
                        Pallet::<T>::deposit_event(e)
                    },
                    Self::CreateOrg(org) => {
                        OrgLookup::<T>::insert(org.org_key().clone(), org.name().clone());
                        OrgRepository::<T>::insert(*org.name(), org);
                    }
                    Self::UpdateOrg(org) => {
                        OrgRepository::<T>::insert(*org.name(), org);
                    }
                }
            }
        }
    }
}
