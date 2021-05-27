#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_mut_passed)]

pub use pallet_deip::*;
use sp_std::vec::Vec;
use codec::{Codec};




// Here we declare the runtime API. It is implemented it the `impl` block in
// runtime amalgamator file (the `runtime/src/lib.rs`)
sp_api::decl_runtime_apis! {
	pub trait DeipApi<AccountId>
		where AccountId: Codec,
	{
		fn get_projects() -> Vec<(ProjectId, AccountId)>;
		fn get_project(project_id: &ProjectId) -> Project<H256, AccountId>;
	}
}