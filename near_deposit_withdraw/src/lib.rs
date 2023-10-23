mod contract_account_id;
mod error;
mod types;

use crate::types::StorageKey;
use crate::types::XccAccountId;
use contract_account_id::{hex_str_to_aurora_id, ContractAccountId};
use error::ContractError;
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde_json;
use near_sdk::store::UnorderedMap;
use near_sdk::PromiseOrValue;
use near_sdk::{env, near_bindgen, AccountId};
use types::ext_fungible_token;
use types::FtDepositMessage;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct AuroraDepositWithdrawContract {
    pub aurora_engine: AccountId,
    pub user_tokens: UnorderedMap<ContractAccountId, UnorderedMap<AccountId, u128>>,
}

impl Default for AuroraDepositWithdrawContract {
    fn default() -> Self {
        env::panic_str("The contract should be initialized before usage")
    }
}

#[near_bindgen]
impl AuroraDepositWithdrawContract {
    /// Initializes the contract with the given NEAR foundation account ID.
    #[init]
    pub fn new(aurora_engine: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        assert!(
            env::is_valid_account_id(aurora_engine.as_bytes()),
            "The Aurora engine ID is invalid"
        );
        Self {
            aurora_engine,
            user_tokens: UnorderedMap::new(StorageKey::UserTokens),
        }
    }

    pub fn get_user_balance(&self, user: ContractAccountId, token: AccountId) -> U128 {
        self.user_tokens
            .get(&user)
            .and_then(|balances| balances.get(&token))
            .and_then(|el| Some(U128::from(*el)))
            .unwrap_or(0.into())
    }

    #[payable]
    #[handle_result]
    pub fn withdraw_to_aurora_acc(
        &mut self,
        token_id: AccountId,
        amount: U128,
        xcc_account_id: Option<XccAccountId>,
    ) -> Result<PromiseOrValue<()>, ContractError> {
        if amount.0 == 0 {
            return Ok(PromiseOrValue::Value(()));
        }

        let receiver_id =
            ContractAccountId::try_from_aurora_id(xcc_account_id, self.aurora_engine.as_str())?;
        let account = self
            .user_tokens
            .get_mut(&receiver_id)
            .ok_or_else(|| ContractError::AccountNotRegistered(receiver_id.to_string()))?;

        // Check whether there is enough balance
        if account.get(&token_id).is_some_and(|el| *el < amount.0) {
            return Err(ContractError::UserBalanceNotEnough);
        }

        account.get_mut(&token_id).map(|el| {
            *el -= amount.0;
        });

        let ft_transfer_promise = match receiver_id {
            ContractAccountId::Near(_) => ext_fungible_token::ext(token_id)
                .with_attached_deposit(1)
                .with_unused_gas_weight(1)
                .ft_transfer(
                    Result::from(receiver_id)?,
                    amount,
                    Some("withdraw".to_string()),
                ),
            ContractAccountId::Aurora(aurora_id) => ext_fungible_token::ext(token_id)
                .with_attached_deposit(1)
                .with_unused_gas_weight(1)
                .ft_transfer_call(
                    self.aurora_engine.clone(),
                    amount,
                    hex::encode(aurora_id),
                    Some("withdraw".to_string()),
                ),
        };

        Ok(PromiseOrValue::Promise(ft_transfer_promise))
    }

    #[handle_result]
    fn internal_ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> Result<PromiseOrValue<U128>, ContractError> {
        let msg: FtDepositMessage =
            serde_json::from_str(&msg).map_err(|_| ContractError::Deserialize)?;

        let account_id = match msg.xcc_account_id {
            Some(account_id) => match account_id {
                XccAccountId::Aurora(aurora_id) => {
                    ContractAccountId::Aurora(hex_str_to_aurora_id(&aurora_id)?)
                }
            },
            None => ContractAccountId::Near(sender_id),
        };

        let token = env::predecessor_account_id();

        let mut user_balances_opt = self.user_tokens.get_mut(&account_id);

        if user_balances_opt.is_none() {
            self.user_tokens.insert(
                account_id.clone(),
                UnorderedMap::new(StorageKey::UserBalances {
                    user: account_id.clone(),
                }),
            );
            user_balances_opt = self.user_tokens.get_mut(&account_id);
        }

        user_balances_opt.unwrap().get_mut(&token).map(|el| {
            *el += amount.0;
        });

        Ok(PromiseOrValue::Value(0.into()))
    }
}

#[near_bindgen]
impl FungibleTokenReceiver for AuroraDepositWithdrawContract {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        match self.internal_ft_on_transfer(sender_id, amount, msg) {
            Ok(res) => res,
            Err(err) => {
                panic!("{}", err);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use near_sdk::{testing_env, MockedBlockchain};

    // mod test_utils;
    // use test_utils::*;

    // #[test]
    // fn test_whitelist() {
    //     let mut context = VMContextBuilder::new()
    //         .current_account_id(account_whitelist())
    //         .predecessor_account_id(account_near())
    //         .finish();
    //     testing_env!(context.clone());

    //     let mut contract = WhitelistContract::new(account_near());

    //     // Check initial whitelist
    //     context.is_view = true;
    //     testing_env!(context.clone());
    //     assert!(!contract.is_whitelisted(account_pool()));

    //     // Adding to whitelist by foundation
    //     context.is_view = false;
    //     testing_env!(context.clone());
    //     assert!(contract.add_staking_pool(account_pool()));

    //     // Checking it's whitelisted now
    //     context.is_view = true;
    //     testing_env!(context.clone());
    //     assert!(contract.is_whitelisted(account_pool()));

    //     // Adding again. Should return false
    //     context.is_view = false;
    //     testing_env!(context.clone());
    //     assert!(!contract.add_staking_pool(account_pool()));

    //     // Checking the pool is still whitelisted
    //     context.is_view = true;
    //     testing_env!(context.clone());
    //     assert!(contract.is_whitelisted(account_pool()));

    //     // Removing from the whitelist.
    //     context.is_view = false;
    //     testing_env!(context.clone());
    //     assert!(contract.remove_staking_pool(account_pool()));

    //     // Checking the pool is not whitelisted anymore
    //     context.is_view = true;
    //     testing_env!(context.clone());
    //     assert!(!contract.is_whitelisted(account_pool()));

    //     // Removing again from the whitelist, should return false.
    //     context.is_view = false;
    //     testing_env!(context.clone());
    //     assert!(!contract.remove_staking_pool(account_pool()));

    //     // Checking the pool is still not whitelisted
    //     context.is_view = true;
    //     testing_env!(context.clone());
    //     assert!(!contract.is_whitelisted(account_pool()));

    //     // Adding again after it was removed. Should return true
    //     context.is_view = false;
    //     testing_env!(context.clone());
    //     assert!(contract.add_staking_pool(account_pool()));

    //     // Checking the pool is now whitelisted again
    //     context.is_view = true;
    //     testing_env!(context.clone());
    //     assert!(contract.is_whitelisted(account_pool()));
    // }

    // #[test]
    // #[should_panic(expected = "Can only be called by NEAR Foundation")]
    // fn test_factory_whitelist_fail() {
    //     let mut context = VMContextBuilder::new()
    //         .current_account_id(account_whitelist())
    //         .predecessor_account_id(account_near())
    //         .finish();
    //     testing_env!(context.clone());

    //     let mut contract = WhitelistContract::new(account_near());

    //     // Trying ot add to the whitelist by NOT whitelisted factory.
    //     context.is_view = false;
    //     context.predecessor_account_id = account_factory();
    //     testing_env!(context.clone());
    //     assert!(contract.add_staking_pool(account_pool()));
    // }

    // #[test]
    // #[should_panic(expected = "Can only be called by NEAR Foundation")]
    // fn test_trying_to_whitelist_factory() {
    //     let mut context = VMContextBuilder::new()
    //         .current_account_id(account_whitelist())
    //         .predecessor_account_id(account_near())
    //         .finish();
    //     testing_env!(context.clone());

    //     let mut contract = WhitelistContract::new(account_near());

    //     // Trying ot whitelist the factory not by the NEAR Foundation.
    //     context.is_view = false;
    //     context.predecessor_account_id = account_factory();
    //     testing_env!(context.clone());
    //     assert!(contract.add_factory(account_factory()));
    // }

    // #[test]
    // #[should_panic(expected = "Can only be called by NEAR Foundation")]
    // fn test_trying_to_remove_by_factory() {
    //     let mut context = VMContextBuilder::new()
    //         .current_account_id(account_whitelist())
    //         .predecessor_account_id(account_near())
    //         .finish();
    //     testing_env!(context.clone());

    //     let mut contract = WhitelistContract::new(account_near());

    //     // Adding factory
    //     context.is_view = false;
    //     testing_env!(context.clone());
    //     assert!(contract.add_factory(account_factory()));

    //     // Trying to remove the pool by the factory.
    //     context.predecessor_account_id = account_factory();
    //     testing_env!(context.clone());
    //     assert!(contract.remove_staking_pool(account_pool()));
    // }

    // #[test]
    // fn test_whitelist_factory() {
    //     let mut context = VMContextBuilder::new()
    //         .current_account_id(account_whitelist())
    //         .predecessor_account_id(account_near())
    //         .finish();
    //     testing_env!(context.clone());

    //     let mut contract = WhitelistContract::new(account_near());

    //     // Check the factory is not whitelisted
    //     context.is_view = true;
    //     testing_env!(context.clone());
    //     assert!(!contract.is_factory_whitelisted(account_factory()));

    //     // Whitelisting factory
    //     context.is_view = false;
    //     testing_env!(context.clone());
    //     assert!(contract.add_factory(account_factory()));

    //     // Check the factory is whitelisted now
    //     context.is_view = true;
    //     testing_env!(context.clone());
    //     assert!(contract.is_factory_whitelisted(account_factory()));
    //     // Check the pool is not whitelisted
    //     assert!(!contract.is_whitelisted(account_pool()));

    //     // Adding to whitelist by foundation
    //     context.is_view = false;
    //     context.predecessor_account_id = account_factory();
    //     testing_env!(context.clone());
    //     assert!(contract.add_staking_pool(account_pool()));

    //     // Checking it's whitelisted now
    //     context.is_view = true;
    //     testing_env!(context.clone());
    //     assert!(contract.is_whitelisted(account_pool()));

    //     // Removing the pool from the whitelisted by the NEAR foundation.
    //     context.is_view = false;
    //     context.predecessor_account_id = account_near();
    //     testing_env!(context.clone());
    //     assert!(contract.remove_staking_pool(account_pool()));

    //     // Checking the pool is not whitelisted anymore
    //     context.is_view = true;
    //     testing_env!(context.clone());
    //     assert!(!contract.is_whitelisted(account_pool()));

    //     // Removing the factory
    //     context.is_view = false;
    //     testing_env!(context.clone());
    //     assert!(contract.remove_factory(account_factory()));

    //     // Check the factory is not whitelisted anymore
    //     context.is_view = true;
    //     testing_env!(context.clone());
    //     assert!(!contract.is_factory_whitelisted(account_factory()));
    // }
}
