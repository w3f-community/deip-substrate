use core::ops::Deref;
use codec::Decode;
use serde::{Serialize, Deserialize};

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct Domain {
    #[serde(flatten)]
    domain: super::Domain,
}

impl common_rpc::GetError for Domain {
    fn get_error() -> common_rpc::Error {
        common_rpc::Error::DomainDecodeFailed
    }
}

impl Decode for Domain {
    fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
        super::Domain::decode(input).map(|domain| Self { domain })
    }
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
#[serde(transparent)]
pub struct DomainId {
    pub id: super::DomainId,
}

impl common_rpc::GetError for DomainId {
    fn get_error() -> common_rpc::Error {
        common_rpc::Error::DomainIdDecodeFailed
    }
}

impl Decode for DomainId {
    fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
        super::DomainId::decode(input).map(|id| Self { id })
    }
}

impl codec::WrapperTypeEncode for DomainId {}
impl codec::EncodeLike<super::DomainId> for DomainId {}

impl Deref for DomainId {
    type Target = super::DomainId;
    fn deref(&self) -> &Self::Target { &self.id }
}
