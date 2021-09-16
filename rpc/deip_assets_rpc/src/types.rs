use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};

pub struct AssetIdError;
impl common_rpc::GetError for AssetIdError {
    fn get_error() -> common_rpc::Error {
        common_rpc::Error::AssetIdDecodeFailed
    }
}

// copied from pallet_assets since struct members are not public
#[derive(Decode, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

pub struct AssetKeyValue<AssetId, Balance, AccountId, DepositBalance> {
    pub id: AssetId,
    _m: std::marker::PhantomData<(Balance, AccountId, DepositBalance)>,
}

impl<AssetId, Balance, AccountId, DepositBalance>
    AssetKeyValue<AssetId, Balance, AccountId, DepositBalance>
{
    pub fn new(id: AssetId) -> Self {
        Self {
            id,
            _m: Default::default(),
        }
    }
}

impl<
        AssetId: 'static + Encode + Decode + Send,
        Balance: 'static + Decode + Send,
        AccountId: 'static + Decode + Send,
        DepositBalance: 'static + Decode + Default + Send,
    > common_rpc::KeyValueInfo for AssetKeyValue<AssetId, Balance, AccountId, DepositBalance>
{
    type Key = AssetId;
    type KeyError = AssetIdError;
    type Value = AssetDetails<Balance, AccountId, DepositBalance>;
    type ValueError = Self::Value;

    fn key(&self) -> &Self::Key {
        &self.id
    }
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
