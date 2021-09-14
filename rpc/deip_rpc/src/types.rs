use super::Domain;

use codec::Decode;
use serde::{Serialize, Deserialize};

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct DomainWrapper {
    #[serde(flatten)]
    domain: Domain,
}

impl common_rpc::GetError for DomainWrapper {
    fn get_error() -> common_rpc::Error {
        common_rpc::Error::DomainDecodeFailed
    }
}

impl Decode for DomainWrapper {
    fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
        Domain::decode(input).map(|domain| Self { domain })
    }
}
