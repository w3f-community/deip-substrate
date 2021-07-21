use crate::*;

use sp_runtime::traits::Saturating;

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct Contribution<AccountId, Balance, Moment> {
    sale_id: ProjectTokenSaleId,
    owner: AccountId,
    amount: Balance,
    time: Moment,
}

impl<T: Config> Module<T> {
    pub(super) fn contribute_to_project_token_sale_impl(
        account: T::AccountId,
        sale_id: ProjectTokenSaleId,
        amount: BalanceOf<T>,
    ) -> DispatchResult {
        let sale = ProjectTokenSaleMap::<T>::try_get(sale_id)
            .map_err(|_| Error::<T>::ContributionProjectTokenSaleNotFound)?;

        ensure!(
            matches!(sale.status, ProjectTokenSaleStatus::Active),
            Error::<T>::ContributionProjectTokenSaleNotActive
        );

        let is_hard_cap_reached = if sale.total_amount.saturating_add(amount) >= sale.hard_cap {
            true
        } else {
            false
        };

        let amount_to_contribute = if is_hard_cap_reached {
            sale.hard_cap.saturating_sub(sale.total_amount)
        } else {
            amount
        };

        ensure!(
            T::Currency::reserve(&account, amount_to_contribute).is_ok(),
            Error::<T>::ContributionNotEnoughFunds
        );

        let contribution =
            ProjectTokenSaleContributionBySaleIdOwner::<T>::try_get(&(sale_id, account.clone()));
        let new_contribution = Contribution {
            sale_id: sale_id,
            owner: account.clone(),
            amount: amount_to_contribute
                .saturating_add(contribution.as_ref().map_or_else(|_| 0u32.into(), |c| c.amount)),
            time: contribution.map_or_else(|_| pallet_timestamp::Module::<T>::get(), |c| c.time),
        };

        ProjectTokenSaleContributionBySaleIdOwner::<T>::insert(
            (sale_id, account),
            new_contribution,
        );

        // update total_amount in token sale 

        if is_hard_cap_reached {
            unimplemented!();
        }

        Ok(())
    }
}
