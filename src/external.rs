use crate::*;
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::json_types::U128;
use near_sdk::{ext_contract, AccountId};

// #[ext_contract(ext_self)]
// pub trait CallbackSelf {
//     fn callback_transfer_call(
//         &self,
//         account_id: &AccountId,
//         category: &CategoryToken,
//         amount: &u128,
//     );
// }

#[ext_contract(ext_ft_fungible_token)]
pub trait FungibleTokenCore {
    fn ft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    );
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
}

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        // Verifying that we were called by fungible token contract that we expect.
        // require!(
        //     self.list_token_address
        //         .contains_key(&env::predecessor_account_id().into()),
        //     "Only supports the one fungible token in actionhup contract"
        // );
        log!(
            "in {} tokens from @{} ft_on_transfer, msg = {}",
            amount.0,
            sender_id.as_ref(),
            msg
        );
        match msg.as_str() {
            "cancel" => PromiseOrValue::Value(amount),
            _ => PromiseOrValue::Value(U128::from(0)),
        }
    }
}
