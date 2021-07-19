use crate::*;

/// Unique ProjectTokenSale ID reference
pub type Id = H160;

#[derive(Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, PartialOrd, Ord)]
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
pub struct Info<Moment, Balance> {
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
    total_amount: Balance,
    soft_cap: Balance,
    hard_cap: Balance,
}

impl<T: Config> Module<T> {
    pub(super) fn create_project_token_sale_impl(
        account: T::AccountId,
        external_id: Id,
        project_id: ProjectId,
        start_time: T::Moment,
        end_time: T::Moment,
        soft_cap: BalanceOf<T>,
        hard_cap: BalanceOf<T>,
    ) -> DispatchResult {
        ensure!(
            !ProjectTokenSaleMap::<T>::contains_key(external_id),
            Error::<T>::TokenSaleAlreadyExists
        );

        let timestamp = pallet_timestamp::Module::<T>::get();
        ensure!(
            start_time >= timestamp,
            Error::<T>::TokenSaleStartTimeMustBeLaterOrEqualCurrentMoment
        );
        ensure!(
            end_time > start_time,
            Error::<T>::TokenSaleEndTimeMustBeLaterStartTime
        );

        ensure!(soft_cap > 0u32.into(), Error::<T>::TokenSaleSoftCapShouldBePositive);
        ensure!(
            hard_cap >= soft_cap,
            Error::<T>::TokenSaleHardCapShouldBeGreaterOrEqualSoftCap
        );

        let projects = Projects::<T>::get();
        match projects.binary_search_by_key(&project_id, |&(p, _)| p) {
            Ok(index) => {
                if projects[index].1 != account {
                    return Err(Error::<T>::ProjectNotBelongToTeam.into());
                }
            }
            Err(_) => return Err(Error::<T>::NoSuchProject.into()),
        }

        let mut token_sales = ProjectTokenSaleByProjectIdStatus::get();
        if let Ok(_) = token_sales.binary_search_by_key(
            &(project_id, ProjectTokenSaleStatus::Active),
            |&(p, s, _)| (p, s),
        ) {
            return Err(Error::<T>::TokenSaleScheduledAlready.into());
        }

        let index = match token_sales.binary_search_by_key(
            &(project_id, ProjectTokenSaleStatus::Inactive),
            |&(p, s, _)| (p, s),
        ) {
            Ok(_) => return Err(Error::<T>::TokenSaleScheduledAlready.into()),
            Err(i) => i,
        };

        let new_project_token_sale = ProjectTokenSale {
            external_id: external_id,
            project_id: project_id,
            start_time: start_time,
            end_time: end_time,
            status: ProjectTokenSaleStatus::Inactive,
            soft_cap: soft_cap,
            hard_cap: hard_cap,
            ..Default::default()
        };

        token_sales.insert(
            index,
            (project_id, ProjectTokenSaleStatus::Inactive, external_id),
        );
        ProjectTokenSaleByProjectIdStatus::put(token_sales);
        ProjectTokenSaleMap::<T>::insert(external_id, new_project_token_sale.clone());

        Self::deposit_event(RawEvent::ProjectTokenSaleCreated(
            project_id,
            new_project_token_sale,
        ));

        Ok(())
    }
}
