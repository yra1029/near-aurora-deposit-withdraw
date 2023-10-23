use std::convert::TryInto;
use std::fmt::Display;

use crate::error::ContractError;
use crate::types::XccAccountId;
use borsh::BorshDeserialize;
use near_sdk::borsh::{self, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, AccountId};

pub type AuroraId = [u8; 20];

#[derive(
    BorshDeserialize,
    BorshSerialize,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Deserialize,
    Serialize,
)]
#[serde(crate = "near_sdk::serde")]
pub enum ContractAccountId {
    Near(AccountId),
    Aurora(AuroraId),
}

impl ContractAccountId {
    pub fn try_from_aurora_id(
        xcc_account_id: Option<XccAccountId>,
        aurora_engine: &str,
    ) -> Result<Self, ContractError> {
        if let Some(account_id) = xcc_account_id {
            match account_id {
                XccAccountId::Aurora(aurora_id) => {
                    if env::predecessor_account_id()
                        .as_str()
                        .ends_with(aurora_engine)
                    {
                        Ok(Self::Aurora(hex_str_to_aurora_id(&aurora_id)?))
                    } else {
                        Err(ContractError::AccountConversion)
                    }
                }
            }
        } else {
            Ok(env::signer_account_id().into())
        }
    }
}

impl From<AccountId> for ContractAccountId {
    fn from(value: AccountId) -> Self {
        ContractAccountId::Near(value)
    }
}

impl From<ContractAccountId> for Result<AccountId, ContractError> {
    fn from(value: ContractAccountId) -> Result<AccountId, ContractError> {
        if let ContractAccountId::Near(account_id) = value {
            Ok(account_id)
        } else {
            Err(ContractError::AccountConversion)
        }
    }
}

impl Display for ContractAccountId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ContractAccountId::Near(acc) => acc.to_string(),
            ContractAccountId::Aurora(acc) => hex::encode(acc),
        };
        f.write_str(value.as_str())
    }
}

pub fn hex_str_to_aurora_id(s: &str) -> Result<AuroraId, ContractError> {
    let start_index = if s.starts_with("0x") && s.len() == 42 {
        2
    } else if s.len() == 40 {
        0
    } else {
        return Err(ContractError::ParseAuroraAccountId(s.to_string()));
    };
    Ok(hex::decode(&s[start_index..]).unwrap().try_into().unwrap())
}
