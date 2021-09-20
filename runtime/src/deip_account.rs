use sp_std::prelude::*;

use frame_support::pallet_prelude::*;

use sp_core::crypto::AccountId32;

use pallet_deip_org::org::OrgName;

#[cfg(feature = "std")]
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Eq, PartialEq, Encode, Decode, Ord, PartialOrd)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum DeipAccountId<Native> {
    Native(Native),
    Org(OrgName),
}

impl Into<AccountId32> for DeipAccountId<AccountId32> {
    fn into(self) -> AccountId32 {
        match self {
            Self::Native(native) => { native },
            Self::Org(name) => {
                pallet_deip_org::org_key::<AccountId32>(&name)
            },
        }
    }
}
