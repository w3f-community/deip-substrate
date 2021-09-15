use core::ops::Deref;
use codec::Decode;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct Dao<AccountId, DaoId> {
    #[serde(flatten)]
    org: super::Org<AccountId, DaoId>,
}

impl<AccountId, DaoId> common_rpc::GetError for Dao<AccountId, DaoId> {
    fn get_error() -> common_rpc::Error {
        common_rpc::Error::DaoDecodeFailed
    }
}

impl<AccountId: Decode, DaoId: Decode> Decode for Dao<AccountId, DaoId> {
    fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
        super::Org::<AccountId, DaoId>::decode(input).map(|org| Self { org })
    }
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
#[serde(transparent)]
pub struct DaoId {
    pub id: super::OrgName,
}

impl common_rpc::GetError for DaoId {
    fn get_error() -> common_rpc::Error {
        common_rpc::Error::DaoIdDecodeFailed
    }
}

impl Decode for DaoId {
    fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
        super::OrgName::decode(input).map(|id| Self { id })
    }
}

impl codec::WrapperTypeEncode for DaoId {}
impl codec::EncodeLike<super::OrgName> for DaoId {}

impl Deref for DaoId {
    type Target = super::OrgName;
    fn deref(&self) -> &Self::Target { &self.id }
}
