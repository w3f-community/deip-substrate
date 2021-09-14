use codec::Decode;
use serde::{Deserialize, Serialize};

// copied from pallet_assets since struct members are not public
#[derive(Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct AssetDetails<Balance: Decode, AccountId: Decode, DepositBalance: Decode + Default> {
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

impl<Balance: Decode, AccountId: Decode, DepositBalance: Decode + Default> common_rpc::GetError
    for AssetDetails<Balance, AccountId, DepositBalance>
{
    fn get_error() -> common_rpc::Error {
        common_rpc::Error::AssetDetailsDecodeFailed
    }
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

#[derive(Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct AssetBalance<Balance: Decode> {
    balance: Balance,
    is_frozen: bool,
    is_zombie: bool,
}

impl<Balance: Decode> common_rpc::GetError for AssetBalance<Balance> {
    fn get_error() -> common_rpc::Error {
        common_rpc::Error::AssetBalanceDecodeFailed
    }
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct AssetBalanceWithIds<AssetId, Balance: Decode, AccountId> {
    pub asset: AssetId,
    pub account: AccountId,
    #[serde(flatten)]
    pub balance: AssetBalance<Balance>,
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct AssetBalanceWithOwner<Balance: Decode, AccountId> {
    pub account: AccountId,
    #[serde(flatten)]
    pub balance: AssetBalance<Balance>,
}
