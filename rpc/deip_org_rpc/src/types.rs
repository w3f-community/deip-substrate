use super::*;

use codec::Decode;
use serde::{Serialize, Deserialize};

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct DaoWrapper<AccountId, DaoId> {
    #[serde(flatten)]
    org: Org<AccountId, DaoId>,
}

impl<AccountId, DaoId> common_rpc::GetError for DaoWrapper<AccountId, DaoId> {
    fn get_error() -> common_rpc::Error {
        common_rpc::Error::DaoDecodeFailed
    }
}

impl<AccountId: Decode, DaoId: Decode> Decode for DaoWrapper<AccountId, DaoId> {
    fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
        Org::<AccountId, DaoId>::decode(input).map(|org| Self { org })
    }
}
