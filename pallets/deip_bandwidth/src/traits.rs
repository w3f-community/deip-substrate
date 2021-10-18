use codec::FullCodec;
use frame_support::{pallet_prelude::MaybeSerializeDeserialize, traits::Imbalance};
use sp_runtime::{DispatchError, traits::AtLeast32BitUnsigned};
use sp_std::fmt::Debug;

pub trait BandwidthPoints<AccountId> {
    type Balance: AtLeast32BitUnsigned + FullCodec + Copy + MaybeSerializeDeserialize + Debug + Default;
    type PositiveImbalance: Imbalance<Self::Balance, Opposite = Self::NegativeImbalance>;
    type NegativeImbalance: Imbalance<Self::Balance, Opposite = Self::PositiveImbalance>;

    fn decrease(
        who: &AccountId,
        value: Self::Balance,
    ) -> Result<Self::NegativeImbalance, ()>;
}
