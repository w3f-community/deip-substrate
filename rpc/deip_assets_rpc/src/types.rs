use codec::Decode;
use serde::{Serialize, Deserialize};

// copied from pallet_assets since struct members are not public
#[derive(Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct AssetDetails<
    Balance: Decode,
    AccountId: Decode,
    DepositBalance: Decode + Default,
> {
    owner: AccountId,
    issuer: AccountId,
    admin: AccountId,
    freezer: AccountId,
    supply: Balance,
    // Skip temporary due to https://github.com/paritytech/substrate/issues/4641
    #[serde(skip)]
    _deposit: DepositBalance,
    max_zombies: u32,
    min_balance: Balance,
    zombies: u32,
    accounts: u32,
    is_frozen: bool,
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct AssetDetailsWithId<
    AssetId,
    Balance: Decode,
    AccountId: Decode,
    DepositBalance: Decode + Default,
> {
    pub id: AssetId,
    #[serde(flatten)]
    pub details: AssetDetails<Balance, AccountId, DepositBalance>,
}
