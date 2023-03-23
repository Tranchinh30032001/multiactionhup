use crate::*;
use near_sdk::collections::LookupMap;
impl Contract {
    pub(crate) fn internal_unwrap_balance(
        &self,
        account_id: &AccountId,
        token: &Category,
    ) -> Balance {
        match self.contributors.get(account_id) {
            Some(token_accounts) => match token_accounts.get(&token) {
                Some(balance) => {
                    return balance;
                }
                None => 0,
            },
            None => 0,
        }
    }
    pub(crate) fn get_balance_owner(&self, category: &Category) -> Balance {
        match self.owner.get(&category) {
            Some(balance) => balance,
            None => 0,
        }
    }
    pub(crate) fn internal_deposit(
        &mut self,
        account_id: &AccountId,
        category: &Category,
        amount: Balance,
    ) {
        let balance_contributor = self.internal_unwrap_balance(account_id, category);
        let balance_owner = self.get_balance_owner(category);

        if let Some(new_balance) = balance_contributor.checked_add(amount) {
            let mut token_account: LookupMap<Category, Balance> =
                LookupMap::new(Prefix::ContributorsNest);
            token_account.insert(category, &new_balance);
            self.contributors.insert(account_id, &token_account);
        }

        if let Some(new_owner_balance) = balance_owner.checked_add(amount) {
            self.owner.insert(category, &new_owner_balance);
        }
    }
}
