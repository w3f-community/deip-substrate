use crate::*;

/// Unique contribution ID reference
pub type Id = H160;

impl<T: Config> Module<T> {
    pub(super) fn contribute_to_project_token_sale_impl() -> DispatchResult {
        Ok(())
    }
}
