use near_sdk::{
    borsh::{self, BorshSerialize},
    FunctionError,
};
use thiserror::Error;

#[derive(BorshSerialize, Debug, Error, FunctionError)]
pub enum ContractError {
    #[error("User has not enough balance")]
    UserBalanceNotEnough,
    #[error("Could not parse aurora account")]
    ParseAuroraAccountId(String),
    #[error("Error while converting aurora account")]
    AccountConversion,
    #[error("Account is not registered")]
    AccountNotRegistered(String),
    #[error("Deserialize")]
    Deserialize,
}
