//! # DEIP ...
//!
//! - [`Config`](./trait.Config.html)
//!
//! ## Overview
//! The pallet ...
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! * [`create_asset`](./enum.Call.html#variant.create_asset)
//!
//! [`Config`]: ./trait.Config.html

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

pub mod traits;

#[doc(inline)]
pub use pallet::*;

use sp_std::marker::PhantomData;
use sp_std::fmt::Debug;
use sp_std::mem;
use sp_std::result;
use sp_runtime::traits::MaybeSerializeDeserialize;
use sp_runtime::traits::Saturating;
use sp_runtime::traits::Zero;
use codec::FullCodec;
use sp_runtime::traits::AtLeast32BitUnsigned;
use sp_runtime::transaction_validity::TransactionValidityError;
use sp_runtime::traits::PostDispatchInfoOf;
use sp_runtime::traits::DispatchInfoOf;
use sp_runtime::transaction_validity::InvalidTransaction;
use sp_runtime::DispatchError;
use sp_runtime::RuntimeDebug;
use frame_support::traits::TryDrop;
use frame_support::traits::Imbalance;
use frame_support::traits::Get;
use frame_support::ensure;

const NON_LOCAL: u8 = 102;

#[frame_support::pallet]
#[doc(hidden)]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::{pallet_prelude::*};
    use codec::FullCodec;
    use frame_support::pallet_prelude::MaybeSerializeDeserialize;
    use sp_runtime::traits::AtLeast32BitUnsigned;
    use sp_runtime::traits::{One, StaticLookup, Zero};
    use sp_std::fmt::Debug;
    use frame_system::offchain::{SendTransactionTypes, SubmitTransaction};

    pub(super) type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
    pub(super) type BandwidthBalanceOf<T> = <T as Config>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config + SendTransactionTypes<Call<Self>> {
        type Balance: AtLeast32BitUnsigned + FullCodec + Copy + MaybeSerializeDeserialize + Debug + Default;

        /// Value of bandwidth points that any account has by default
        #[pallet::constant]
        type BaseBandwidth: Get<Self::Balance>;

        /// Period (in blocks) to wait for full recover of bandwidth points
        #[pallet::constant]
        type Period: Get<Self::BlockNumber>;
    }

    #[doc(hidden)]
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[doc(hidden)]
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn offchain_worker(n: T::BlockNumber) {
            if !sp_io::offchain::is_validator() {
                return;
            }

            if n % T::Period::get() != Zero::zero() {
                return;
            }

            let call = Call::restore_bandwidth();
            let _submit =
                SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into());
        }
    }

    #[pallet::validate_unsigned]
    impl<T: Config> ValidateUnsigned for Pallet<T> {
        type Call = Call<T>;

        fn validate_unsigned(source: TransactionSource, call: &Self::Call) -> TransactionValidity {
            if !matches!(
                source,
                TransactionSource::Local | TransactionSource::InBlock
            ) {
                return InvalidTransaction::Custom(super::NON_LOCAL).into();
            }

            if let Call::restore_bandwidth() = call {
                if BandwidthMap::<T>::iter_values().next().is_none() {
                    return InvalidTransaction::Stale.into();
                }

                ValidTransaction::with_tag_prefix("DeipBandwidthOffchainWorker")
                    .propagate(false)
                    .longevity(5)
                    .and_provides(())
                    .build()
            } else {
                InvalidTransaction::Call.into()
            }
        }
    }

    #[pallet::error]
    pub enum Error<T> {}

    #[pallet::storage]
    pub(super) type BandwidthMap<T: Config> =
        StorageMap<_, Identity, AccountIdOf<T>, BandwidthBalanceOf<T>, OptionQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub(super) fn restore_bandwidth(
            origin: OriginFor<T>,
        ) -> DispatchResultWithPostInfo {
            ensure_none(origin)?;

            BandwidthMap::<T>::remove_all();

            Ok(None.into())
        }
    }
}

impl<T: Config> Pallet<T> {
    fn get_default_balance(_who: &AccountIdOf<T>) -> BandwidthBalanceOf<T> {
        T::BaseBandwidth::get()
    }
}

#[derive(RuntimeDebug, PartialEq, Eq)]
pub struct PositiveImbalance<Balance>(Balance);

impl<Balance> PositiveImbalance<Balance> {
    /// Create a new positive imbalance from a balance.
    pub fn new(amount: Balance) -> Self {
        PositiveImbalance(amount)
    }
}

/// Opaque, move-only struct with private fields that serves as a token denoting that
/// funds have been destroyed without any equal and opposite accounting.
#[derive(RuntimeDebug, PartialEq, Eq)]
pub struct NegativeImbalance<Balance>(Balance);

impl<Balance> NegativeImbalance<Balance> {
    /// Create a new negative imbalance from a balance.
    pub fn new(amount: Balance) -> Self {
        NegativeImbalance(amount)
    }
}

impl<Balance: AtLeast32BitUnsigned + FullCodec + Copy + MaybeSerializeDeserialize + Debug + Default> TryDrop for PositiveImbalance<Balance> {
    fn try_drop(self) -> result::Result<(), Self> {
        self.drop_zero()
    }
}

impl<Balance: AtLeast32BitUnsigned + FullCodec + Copy + MaybeSerializeDeserialize + Debug + Default> Imbalance<Balance> for PositiveImbalance<Balance> {
    type Opposite = NegativeImbalance<Balance>;

    fn zero() -> Self {
        Self(Zero::zero())
    }

    fn drop_zero(self) -> result::Result<(), Self> {
        if self.0.is_zero() {
            Ok(())
        } else {
            Err(self)
        }
    }

    fn split(self, amount: Balance) -> (Self, Self) {
        let first = self.0.min(amount);
        let second = self.0 - first;

        mem::forget(self);
        (Self(first), Self(second))
    }

    fn merge(mut self, other: Self) -> Self {
        self.0 = self.0.saturating_add(other.0);
        mem::forget(other);

        self
    }

    fn subsume(&mut self, other: Self) {
        self.0 = self.0.saturating_add(other.0);
        mem::forget(other);
    }

    fn offset(self, other: Self::Opposite) -> result::Result<Self, Self::Opposite> {
        let (a, b) = (self.0, other.0);
        mem::forget((self, other));

        if a >= b {
            Ok(Self(a - b))
        } else {
            Err(NegativeImbalance::new(b - a))
        }
    }

    fn peek(&self) -> Balance {
        self.0.clone()
    }
}

impl<Balance: AtLeast32BitUnsigned + FullCodec + Copy + MaybeSerializeDeserialize + Debug + Default> TryDrop for NegativeImbalance<Balance> {
    fn try_drop(self) -> result::Result<(), Self> {
        self.drop_zero()
    }
}

impl<Balance: AtLeast32BitUnsigned + FullCodec + Copy + MaybeSerializeDeserialize + Debug + Default> Imbalance<Balance> for NegativeImbalance<Balance> {
    type Opposite = PositiveImbalance<Balance>;

    fn zero() -> Self {
        Self(Zero::zero())
    }
    fn drop_zero(self) -> result::Result<(), Self> {
        if self.0.is_zero() {
            Ok(())
        } else {
            Err(self)
        }
    }
    fn split(self, amount: Balance) -> (Self, Self) {
        let first = self.0.min(amount);
        let second = self.0 - first;

        mem::forget(self);
        (Self(first), Self(second))
    }
    fn merge(mut self, other: Self) -> Self {
        self.0 = self.0.saturating_add(other.0);
        mem::forget(other);

        self
    }
    fn subsume(&mut self, other: Self) {
        self.0 = self.0.saturating_add(other.0);
        mem::forget(other);
    }
    fn offset(self, other: Self::Opposite) -> result::Result<Self, Self::Opposite> {
        let (a, b) = (self.0, other.0);
        mem::forget((self, other));

        if a >= b {
            Ok(Self(a - b))
        } else {
            Err(PositiveImbalance::new(b - a))
        }
    }
    fn peek(&self) -> Balance {
        self.0.clone()
    }
}

impl<T: Config> traits::BandwidthPoints<AccountIdOf<T>> for Pallet<T> {
    type Balance = BandwidthBalanceOf<T>;
    type PositiveImbalance = self::PositiveImbalance<Self::Balance>;
    type NegativeImbalance = self::NegativeImbalance<Self::Balance>;

    fn decrease(
        who: &AccountIdOf<T>,
        value: Self::Balance,
    ) -> Result<Self::NegativeImbalance, ()> {
        BandwidthMap::<T>::try_mutate(who, |maybe| {
            let bandwidth = match maybe {
                Some(b) => *b,
                None => Self::get_default_balance(who),
            };

            ensure!(bandwidth >= value, ());
            let new_bandwidth = bandwidth - value;
            *maybe = Some(new_bandwidth);

            Ok(NegativeImbalance::new(value))
        })
    }
}
