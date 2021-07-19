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

/// Contains information about tokens of the project
#[derive(Encode, Decode, Default)]
pub struct TokenInfo {
    pub total: u64,
    pub reserved: u64,
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
    /// How many tokens supposed to sale
    security_tokens_on_sale: u64,
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
        security_tokens_on_sale: u64,
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

        ensure!(
            soft_cap > 0u32.into(),
            Error::<T>::TokenSaleSoftCapShouldBePositive
        );
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
            security_tokens_on_sale: security_tokens_on_sale,
            ..Default::default()
        };

        ProjectTokens::mutate_exists(project_id, |maybe_project| -> DispatchResult {
            let project = maybe_project.as_mut().ok_or(Error::<T>::NoSuchProject)?;

            ensure!(
                security_tokens_on_sale <= project.total,
                Error::<T>::TokenSaleBalanceIsNotEnough
            );
            project
                .total
                .checked_sub(security_tokens_on_sale)
                .expect("total has appropriate value");
            project
                .reserved
                .checked_add(security_tokens_on_sale)
                .expect("reserved can't exceed total");

            Ok(())
        })?;

        token_sales.insert(
            index,
            (project_id, ProjectTokenSaleStatus::Inactive, external_id),
        );
        ProjectTokenSaleByProjectIdStatus::put(token_sales);
        ProjectTokenSaleMap::<T>::insert(external_id, new_project_token_sale.clone());
        ProjectTokenSaleEndTimes::<T>::mutate(|v| {
            let index = match v.binary_search_by_key(&end_time, |&(e, _)| e) {
                Ok(i) => i,
                Err(i) => i,
            };
            v.insert(index, (end_time, external_id));
        });

        Self::deposit_event(RawEvent::ProjectTokenSaleCreated(
            project_id,
            new_project_token_sale,
        ));

        Ok(())
    }

    pub(super) fn process_project_token_sales() {
        let now = pallet_timestamp::Module::<T>::get();

        let token_sales_by_end_time = ProjectTokenSaleEndTimes::<T>::get();
        for (_, sale_id) in token_sales_by_end_time {
            let sale = ProjectTokenSaleMap::<T>::get(sale_id);
            if sale.end_time <= now && matches!(sale.status, ProjectTokenSaleStatus::Active) {
                if sale.total_amount < sale.soft_cap {
                    Self::update_status(sale, ProjectTokenSaleStatus::Expired);
                    Self::refund_project_token_sale(sale_id);
                } else if sale.total_amount >= sale.soft_cap {
                    Self::update_status(sale, ProjectTokenSaleStatus::Finished);
                    Self::finish_project_token_sale(sale_id);
                }
            } else if sale.end_time > now {
                if now >= sale.start_time && matches!(sale.status, ProjectTokenSaleStatus::Inactive)
                {
                    Self::update_status(sale, ProjectTokenSaleStatus::Active);
                }
            }
        }
    }

    fn update_status(sale: ProjectTokenSaleOf<T>, new_status: ProjectTokenSaleStatus) {
        ProjectTokenSaleMap::<T>::mutate_exists(sale.external_id, |maybe_sale| -> () {
            let sale = maybe_sale.as_mut().expect("we keep collections in sync");
            sale.status = new_status;
        });

        let mut token_sales = ProjectTokenSaleByProjectIdStatus::get();
        match new_status {
            ProjectTokenSaleStatus::Finished
            | ProjectTokenSaleStatus::Expired
            | ProjectTokenSaleStatus::Active => {
                let old_index = token_sales
                    .binary_search_by_key(&(sale.project_id, sale.status), |&(p, t, _)| (p, t))
                    .expect("we keep collections in sync");
                token_sales.remove(old_index);

                let index = match token_sales
                    .binary_search_by_key(&(sale.project_id, new_status), |&(p, t, _)| (p, t))
                {
                    Ok(i) => i,
                    Err(i) => i,
                };

                token_sales.insert(index, (sale.project_id, new_status, sale.external_id));
                ProjectTokenSaleByProjectIdStatus::put(token_sales);
            }
            _ => (),
        }
    }

    fn refund_project_token_sale(id: ProjectTokenSaleId) {
        // unimplemented!();
    }

    fn finish_project_token_sale(id: ProjectTokenSaleId) {
        // unimplemented!();
    }
}
