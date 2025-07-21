use crate::models::transaction::{MessageHeader, Transaction};
use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputTransaction {
    Base58(String),
    Base64(String),
    Json(UiTransaction)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UiTransaction {
    pub signatures: Vec<String>,
    pub message: UiRawMessage
}

impl From<&Transaction> for UiTransaction {
    fn from(tx: &Transaction) -> Self {
        UiTransaction {
            signatures: tx
                .signatures
                .iter()
                .map(|sig| bs58::encode(sig.to_bytes()).into_string())
                .collect(),
            message: UiRawMessage {
                header: MessageHeader {
                    num_required_signatures: tx.message.header.num_required_signatures,
                    num_readonly_signed_accounts: tx.message.header.num_readonly_signed_accounts,
                    num_readonly_unsigned_accounts: tx
                        .message
                        .header
                        .num_readonly_unsigned_accounts,
                },
                account_keys: tx
                    .message
                    .account_keys
                    .iter()
                    .map(|k| k.to_string())
                    .collect(),
                recent_blockhash: tx.message.recent_blockhash.to_string(),
                instructions: tx
                    .message
                    .instructions
                    .iter()
                    .map(|ix| UiCompiledInstruction {
                        program_id_index: ix.program_id_index,
                        accounts: ix.accounts.clone(),
                        data: bs58::encode(&ix.data).into_string(),
                    })
                    .collect(),
            },
        }
    }
}

// TODO: add comments with detailed explanation
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UiRawMessage {
    pub header: MessageHeader,
    #[serde(alias = "accountKeys")]
    pub account_keys: Vec<String>,
    #[serde(alias = "recentBlockhash")]
    pub recent_blockhash: String,
    pub instructions: Vec<UiCompiledInstruction>
}


// TODO: add comments with detailed explanation
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UiCompiledInstruction {
    #[serde(alias = "programIdIndex")]
    pub program_id_index: u8,
    pub accounts: Vec<u8>,
    pub data: String
}

use crate::errors::TransactionParseError;
use crate::models::pubkey_base58::PubkeyBase58;
use crate::models::transaction::{Instruction, Message};
use bs58;
use ed25519_dalek::Signature as DalekSignature;
use std::convert::TryFrom;
use std::convert::TryInto;

/// Convert UI DTO into domain Transaction
impl TryFrom<UiTransaction> for Transaction {
    type Error = TransactionParseError;

    fn try_from(ui: UiTransaction) -> Result<Self, Self::Error> {
        // Decode signatures
        let signatures = ui
            .signatures
            .into_iter()
            .map(|s| {
                let bytes = bs58::decode(&s).into_vec().map_err(|e| {
                    TransactionParseError::InvalidFormat(format!("Invalid signature base58: {}", e))
                })?;
                let arr: [u8; 64] = bytes.as_slice().try_into().map_err(|_| {
                    TransactionParseError::InvalidFormat("Invalid signature length".into())
                })?;
                let sig = DalekSignature::try_from(&arr).map_err(|e| {
                    TransactionParseError::InvalidFormat(format!("Invalid signature bytes: {}", e))
                })?;
                Ok(sig)
            })
            .collect::<std::result::Result<Vec<_>, _>>()?;

        // Decode account keys
        let account_keys = ui
            .message
            .account_keys
            .into_iter()
            .map(|k| {
                PubkeyBase58::try_from(k.as_str()).map_err(|e| {
                    TransactionParseError::InvalidFormat(format!(
                        "Invalid account key base58: {}",
                        e
                    ))
                })
            })
            .collect::<std::result::Result<Vec<_>, _>>()?;

        // Parse recent blockhash
        let recent_blockhash = ui.message.recent_blockhash.parse().map_err(|e| {
            TransactionParseError::InvalidFormat(format!("Invalid blockhash: {}", e))
        })?;

        // Decode instructions
        let instructions = ui
            .message
            .instructions
            .into_iter()
            .map(|ix| {
                let data = bs58::decode(&ix.data).into_vec().map_err(|e| {
                    TransactionParseError::InvalidFormat(format!(
                        "Invalid instruction data base58: {}",
                        e
                    ))
                })?;
                Ok(Instruction {
                    program_id_index: ix.program_id_index,
                    accounts: ix.accounts,
                    data,
                })
            })
            .collect::<std::result::Result<Vec<_>, _>>()?;

        // Assemble message
        let message = Message {
            header: ui.message.header,
            account_keys,
            recent_blockhash,
            instructions,
        };

        Ok(Transaction {
            signatures,
            message,
        })
    }
}