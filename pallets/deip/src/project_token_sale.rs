use crate::*;

/// Unique ProjectTokenSale ID reference
pub type Id = H160;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum Status {
    Active,
    Finished,
    Expired,
    Inactive,
}

impl Default for Status {
    fn default() -> Self {
        Status::Inactive
    }
}

/// The object represents a sale of project's tokens with
/// various parameters.
/// It is connected to the specific project.
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct Info<Moment> {
    /// Reference for external world and uniques control
    external_id: Id,
    /// Reference to the Project
    project_id: ProjectId,
    /// When the sale starts
    start_time: Moment,
    /// When it supposed to end
    end_time: Moment,
    status: Status,
    /// How many contributions already reserved
    total_amount: (),
    soft_cap: (),
    hard_cap: (),
}

impl<T: Config> Module<T> {
    pub(super) fn create_project_token_sale_impl(account: T::AccountId,
        external_id: Id,
        project_id: ProjectId,
        start_time: T::Moment,
        end_time: T::Moment,
        soft_cap: (),
        hard_cap: (),
    ) -> DispatchResult {
        let timestamp = pallet_timestamp::Module::<T>::get();
        ensure!(start_time >= timestamp, Error::<T>::TokenSaleStartDateMustBeLaterOrEqualCurrentMoment);

        Ok(())
    }
}
