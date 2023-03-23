use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, log, near_bindgen, require, AccountId, Balance, BorshStorageKey, Gas, PanicOnDefault,
    PromiseOrValue,
};
mod external;
mod internal;
mod test;
mod utils;
use external::*;
use utils::*;

pub const FT_TRANSFER_GAS: Gas = Gas(10_000_000_000_000);

pub type Category = String;

#[derive(BorshSerialize, BorshStorageKey)]

pub enum Prefix {
    Owner,
    Contributors,
    ContributorsNest,
    ListToken,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub list_token_address: LookupMap<Category, AccountId>,
    pub owner: LookupMap<Category, Balance>,
    pub contributors: LookupMap<AccountId, LookupMap<Category, Balance>>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            list_token_address: LookupMap::new(Prefix::ListToken),
            owner: LookupMap::new(Prefix::Owner),
            contributors: LookupMap::new(Prefix::Contributors),
        }
    }
}

#[near_bindgen]
impl Contract {
    #[private]
    pub fn update_list_token(&mut self, new_symbol_token: Category, new_token_account: AccountId) {
        self.list_token_address
            .insert(&new_symbol_token, &new_token_account);
    }

    #[payable]
    pub fn deposit_token(&mut self, amount: U128, category: &Category, msg: String) {
        assert_at_least_one_yocto();
        let init_storage = env::storage_usage();
        let amount = amount.into();
        let account_id = env::predecessor_account_id();
        if category == "Near" {
            let attached_deposit = env::attached_deposit();
            require!(
                attached_deposit == amount,
                "The attached_deposit must equal to the amount"
            )
        } else {
            let token_address = match self.list_token_address.get(&category) {
                Some(account_id) => account_id,
                None => env::panic_str(
                    format!(
                        "The fungible token {} is unvalid in this smartcontract",
                        category
                    )
                    .as_str(),
                ),
            };
            ext_ft_fungible_token::ext(token_address.clone())
                .with_attached_deposit(1)
                .with_static_gas(FT_TRANSFER_GAS)
                .ft_transfer_call(
                    env::current_account_id(),
                    amount.into(),
                    Some(String::from("ok")),
                    msg,
                );
        }
        self.internal_deposit(&account_id, &category, amount);
        if category != "Near" {
            let final_storage = env::storage_usage();
            let storage_used = final_storage - init_storage;
            refund_deposit(storage_used);
        }
    }

    // View method
    pub fn get_balance(&self, account_id: &AccountId, token: &Category) -> Balance {
        let balance = self.internal_unwrap_balance(account_id, token);

        balance
    }

    pub fn get_info_owner(&self, category: &Category) -> Balance {
        self.get_balance_owner(category)
    }

    pub fn check_ft_exits(&self, category: Category) -> bool {
        self.list_token_address.contains_key(&category)
    }
}
