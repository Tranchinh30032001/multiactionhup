use crate::*;
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::json_types::U128;
use near_sdk::{ext_contract, AccountId};

#[ext_contract(ext_ft_fungible_token)]
pub trait FungibleTokenCore {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
}

#[ext_contract(ext_ft_storage)]
pub trait StorageManagement {
    fn storage_deposit(&mut self, account_id: Option<AccountId>, registration_only: Option<bool>);
}
