use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;
use near_contract_standards::storage_management::StorageBalance;
use near_sdk::borsh::{self, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{ext_contract, AccountId, BorshStorageKey};

use crate::contract_account_id::ContractAccountId;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub enum XccAccountId {
    Aurora(String),
}

#[derive(Debug, BorshStorageKey, BorshSerialize, PartialEq, Eq)]
pub enum StorageKey {
    UserTokens,
    UserBalances { user: ContractAccountId },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FtDepositMessageData {}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FtDepositMessage {
    pub xcc_account_id: Option<XccAccountId>,
    pub data: FtDepositMessageData,
}

#[ext_contract]
pub trait ExtFungibleToken {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
    fn ft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        msg: String,
        memo: Option<String>,
    );
    fn ft_metadata(&self) -> FungibleTokenMetadata;
    fn storage_balance_of(&self, account_id: AccountId) -> Option<StorageBalance>;
}
