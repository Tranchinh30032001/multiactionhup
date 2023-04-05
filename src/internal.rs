use crate::*;
use near_sdk::collections::LookupMap;
use near_sdk::{log, CryptoHash, PromiseResult};
impl Contract {
    pub(crate) fn internal_unwrap_balance(
        &self,
        account_id: &AccountId,
        token: &Category,
    ) -> Option<Balance> {
        match self.clients.get(account_id) {
            Some(token_accounts) => match token_accounts.get(&token) {
                Some(balance) => {
                    return Some(balance);
                }
                None => {
                    log!(
                        "The account {} havent's deposit {} token yet",
                        account_id,
                        token
                    );
                    Some(0)
                }
            },
            None => {
                log!("This account wasn't deposit to this smartcontract anytime");
                None
            }
        }
    }

    pub(crate) fn internal_deposit(
        &mut self,
        account_id: &AccountId,
        category: &Category,
        amount: Balance,
    ) {
        if let Some(balance) = self.internal_unwrap_balance(account_id, category) {
            if let Some(new_balance) = balance.checked_add(amount) {
                let mut token_account = self
                    .clients
                    .get(account_id)
                    .unwrap_or_else(|| env::panic_str("Error deposit token"));
                token_account.insert(category, &new_balance);
            } else {
                env::panic_str("Balance Overflow");
            }
        } else {
            // the first time
            let mut token_account: LookupMap<Category, Balance> =
                LookupMap::new(Prefix::ClientsNest {
                    time_stamp: env::block_timestamp(),
                });
            token_account.insert(category, &amount);
            self.clients.insert(account_id, &token_account);
        }
    }
}
