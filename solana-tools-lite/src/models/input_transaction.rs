use crate::models::{
    message::{Message, MessageAddressTableLookup, MessageHeader, MessageLegacy, MessageV0},
    transaction::Transaction,
};
use serde::{Deserialize, Serialize};
///
/// This enum allows the library to accept transactions in various wire formats
/// (Base58, Base64) or as a structured JSON object.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputTransaction {
    /// Transaction encoded as a Base58 string.
    Base58(String),
    /// Transaction encoded as a Base64 string.
    Base64(String),
    /// Transaction represented as a structured JSON object.
    Json(UiTransaction),
}

/// Output formats for a serialized Solana transaction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputFormat {
    /// JSON representation (pretty or compact).
    Json { pretty: bool },
    /// Raw transaction serialized as Base64.
    Base64,
    /// Raw transaction serialized as Base58.
    Base58,
}

impl InputTransaction {
    /// Returns the preferred output format for this input kind.
    pub fn default_output_format(&self, pretty: bool) -> OutputFormat {
        match self {
            InputTransaction::Json(_) => OutputFormat::Json { pretty },
            InputTransaction::Base64(_) => OutputFormat::Base64,
            InputTransaction::Base58(_) => OutputFormat::Base58,
        }
    }
}

/// A human-readable/serializable representation of a Solana transaction.
///
/// This struct corresponds to the standard JSON representation of transactions
/// often used in API responses and CLI outputs.
#[derive(Debug, Serialize, Deserialize)]
pub struct UiTransaction {
    /// List of signatures (Base58 strings).
    pub signatures: Vec<String>,
    /// The message content in a serializable format.
    pub message: UiRawMessage,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UiRawMessage {
    /// Legacy message layout without address table lookups.
    Legacy(UiRawMessageLegacy),
    /// V0 message layout with optional address table lookups.
    V0(UiRawMessageV0),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
/// Legacy message fields for JSON representation.
pub struct UiRawMessageLegacy {
    pub header: MessageHeader,
    #[serde(alias = "accountKeys")]
    pub account_keys: Vec<String>,
    #[serde(alias = "recentBlockhash")]
    pub recent_blockhash: String,
    pub instructions: Vec<UiCompiledInstruction>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// V0 message fields for JSON representation.
pub struct UiRawMessageV0 {
    pub header: MessageHeader,
    #[serde(alias = "accountKeys")]
    pub account_keys: Vec<String>,
    #[serde(alias = "recentBlockhash")]
    pub recent_blockhash: String,
    pub instructions: Vec<UiCompiledInstruction>,
    #[serde(alias = "addressTableLookups")]
    pub address_table_lookups: Vec<UiAddressTableLookup>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Address lookup table entry in JSON form.
pub struct UiAddressTableLookup {
    pub account_key: String,
    pub writable_indexes: Vec<u8>,
    pub readonly_indexes: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Compiled instruction fields in JSON form.
pub struct UiCompiledInstruction {
    #[serde(alias = "programIdIndex")]
    pub program_id_index: u8,
    pub accounts: Vec<u8>,
    pub data: String,
}

use crate::errors::TransactionParseError;
use crate::models::instruction::Instruction;
use crate::models::pubkey_base58::PubkeyBase58;
use bs58;
use ed25519_dalek::Signature as DalekSignature;
use std::convert::TryFrom;
use std::convert::TryInto;

// Helper to map generic parts of message
fn map_common_parts(
    account_keys: &[String],
    recent_blockhash: &str,
    instructions: &[UiCompiledInstruction],
) -> Result<
    (
        Vec<PubkeyBase58>,
        crate::models::hash_base58::HashBase58,
        Vec<Instruction>,
    ),
    TransactionParseError,
> {
    let keys = account_keys
        .iter()
        .map(|k| {
            PubkeyBase58::try_from(k.as_str()).map_err(|e| {
                TransactionParseError::InvalidFormat(format!("Invalid account key: {}", e))
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let blockhash = recent_blockhash
        .parse()
        .map_err(|e| TransactionParseError::InvalidFormat(format!("Invalid blockhash: {}", e)))?;

    let inst = instructions
        .iter()
        .map(|ix| {
            let data = bs58::decode(&ix.data).into_vec().map_err(|e| {
                TransactionParseError::InvalidFormat(format!("Invalid instruction data: {}", e))
            })?;
            Ok(Instruction {
                program_id_index: ix.program_id_index,
                accounts: ix.accounts.clone(),
                data,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok((keys, blockhash, inst))
}

impl From<&Transaction> for UiTransaction {
    fn from(tx: &Transaction) -> Self {
        let signatures = tx
            .signatures
            .iter()
            .map(|sig| bs58::encode(sig.to_bytes()).into_string())
            .collect();

        let message = match &tx.message {
            Message::Legacy(msg) => UiRawMessage::Legacy(UiRawMessageLegacy {
                header: MessageHeader { ..msg.header },
                account_keys: msg.account_keys.iter().map(|k| k.to_string()).collect(),
                recent_blockhash: msg.recent_blockhash.to_string(),
                instructions: msg
                    .instructions
                    .iter()
                    .map(|ix| UiCompiledInstruction {
                        program_id_index: ix.program_id_index,
                        accounts: ix.accounts.clone(),
                        data: bs58::encode(&ix.data).into_string(),
                    })
                    .collect(),
            }),
            Message::V0(msg) => UiRawMessage::V0(UiRawMessageV0 {
                header: MessageHeader { ..msg.header },
                account_keys: msg.account_keys.iter().map(|k| k.to_string()).collect(),
                recent_blockhash: msg.recent_blockhash.to_string(),
                instructions: msg
                    .instructions
                    .iter()
                    .map(|ix| UiCompiledInstruction {
                        program_id_index: ix.program_id_index,
                        accounts: ix.accounts.clone(),
                        data: bs58::encode(&ix.data).into_string(),
                    })
                    .collect(),
                address_table_lookups: msg
                    .address_table_lookups
                    .iter()
                    .map(|lut| UiAddressTableLookup {
                        account_key: lut.account_key.to_string(),
                        writable_indexes: lut.writable_indexes.clone(),
                        readonly_indexes: lut.readonly_indexes.clone(),
                    })
                    .collect(),
            }),
        };

        UiTransaction {
            signatures,
            message,
        }
    }
}

impl TryFrom<UiTransaction> for Transaction {
    type Error = TransactionParseError;

    fn try_from(ui: UiTransaction) -> Result<Self, Self::Error> {
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
                DalekSignature::try_from(&arr).map_err(|e| {
                    TransactionParseError::InvalidFormat(format!("Invalid signature bytes: {}", e))
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        let message = match ui.message {
            UiRawMessage::Legacy(msg) => {
                let (account_keys, recent_blockhash, instructions) =
                    map_common_parts(&msg.account_keys, &msg.recent_blockhash, &msg.instructions)?;
                Message::Legacy(MessageLegacy {
                    header: msg.header,
                    account_keys,
                    recent_blockhash,
                    instructions,
                })
            }
            UiRawMessage::V0(msg) => {
                let (account_keys, recent_blockhash, instructions) =
                    map_common_parts(&msg.account_keys, &msg.recent_blockhash, &msg.instructions)?;

                let address_table_lookups =
                    msg.address_table_lookups
                        .into_iter()
                        .map(|lut| {
                            Ok(MessageAddressTableLookup {
                                account_key: PubkeyBase58::try_from(lut.account_key.as_str())
                                    .map_err(|e| {
                                        TransactionParseError::InvalidFormat(format!(
                                            "Invalid lookup table key: {}",
                                            e
                                        ))
                                    })?,
                                writable_indexes: lut.writable_indexes,
                                readonly_indexes: lut.readonly_indexes,
                            })
                        })
                        .collect::<Result<Vec<_>, TransactionParseError>>()?;

                Message::V0(MessageV0 {
                    header: msg.header,
                    account_keys,
                    recent_blockhash,
                    instructions,
                    address_table_lookups,
                })
            }
        };

        Ok(Transaction {
            signatures,
            message,
        })
    }
}
