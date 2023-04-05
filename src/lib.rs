use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U128;
use near_sdk::{
    env, log, near_bindgen, require, AccountId, Balance, BorshStorageKey, CryptoHash, Gas,
    PanicOnDefault, PromiseOrValue,
};
mod external;
mod internal;
mod utils;

use external::*;
use internal::*;
use utils::*;

pub const FT_TRANSFER_GAS: Gas = Gas(10_000_000_000_000);

pub type Category = String;

#[derive(BorshSerialize, BorshStorageKey)]

pub enum Prefix {
    Owner,
    Clients,
    ClientsNest { time_stamp: u64 },
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    // account_id of fungible token
    token_address: AccountId,
    // AccountId of Depositer -> Category Token -> Balance
    pub clients: LookupMap<AccountId, LookupMap<Category, Balance>>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            // Node: fungible token according to nep141 standards. not modify anything.
            token_address: AccountId::new_unchecked(String::from("fungible2.testnet")),
            clients: LookupMap::new(Prefix::Clients),
        }
    }
}

#[near_bindgen]
impl Contract {
    // Now category can be anything
    #[payable]
    pub fn deposit_ft(&mut self, amount: U128, category: &Category) {
        // kiem tra client co du so ft de deposit khong
        assert_at_least_one_yocto();
        let sender_id = env::predecessor_account_id();
        let attached_deposit = env::attached_deposit();

        // call storage_deposit of fungible token
        ext_ft_storage::ext(self.token_address.clone())
            .with_attached_deposit(attached_deposit)
            .with_static_gas(FT_TRANSFER_GAS)
            //register smartcontract account_id to receive ft token.
            .storage_deposit(Some(env::current_account_id()), None);
        log!("ok da xuong day");
        // call ft_transfer deposit ft into smartcontract
        ext_ft_fungible_token::ext(self.token_address.clone())
            .with_attached_deposit(1)
            .with_static_gas(FT_TRANSFER_GAS)
            .ft_transfer(env::current_account_id(), amount, None);

        self.internal_deposit(&sender_id, category, amount.0);
    }
}
