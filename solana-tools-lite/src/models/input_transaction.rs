use crate::models::transaction::{MessageHeader, Transaction};
use serde::{Deserialize, Serialize};
//use crate::models::transaction::serde_bytes_base58;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputTransaction {
    Base58(String),
    Base64(String),
    Json(UiTransaction),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UiTransaction {
    pub signatures: Vec<String>,
    pub message: UiRawMessage,
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
pub struct UiRawMessage {
    pub header: MessageHeader,
    pub account_keys: Vec<String>,
    pub recent_blockhash: String,
    pub instructions: Vec<UiCompiledInstruction>
}

// TODO: add comments with detailed explanation
#[derive(Debug, Serialize, Deserialize)]
pub struct UiCompiledInstruction {
    pub program_id_index: u8,
    pub accounts: Vec<u8>,
    pub data: String
}
