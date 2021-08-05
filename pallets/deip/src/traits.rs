use crate::*;

use sp_runtime::traits::AtLeast32BitUnsigned;
use codec::HasCompact;

pub trait DeipAssetSystem {
    /// The units in which asset balances are recorded.
    type Balance: Member + Parameter + AtLeast32BitUnsigned + Default + Copy;

    /// The arithmetic type of asset identifier.
    type AssetId: Member + Parameter + Default + Copy + HasCompact;

    /// Returns `Some(project_id)` if it is secured with token specified by `id`.
    fn try_get_tokenized_project(id: &Self::AssetId) -> Option<ProjectId>;
}
