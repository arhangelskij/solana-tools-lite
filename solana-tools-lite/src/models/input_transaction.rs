use serde::{Deserialize, Serialize};
use crate::models::transaction::MessageHeader;
//use crate::models::transaction::serde_bytes_base58;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum InputTransaction {
    Base58(String),
    Base64(String),
    Json(UiTransaction),
}

#[derive(Debug, Deserialize)]
pub struct UiTransaction {
    pub signatures: Vec<String>,
    pub message: UiRawMessage,
}
// TODO: add comments with detailed explanation
#[derive(Debug, Deserialize)]
pub struct UiRawMessage {
    pub header: MessageHeader,
    pub account_keys: Vec<String>,
    pub recent_blockhash: String,
    pub instructions: Vec<UiCompiledInstruction>,
}

// TODO: add comments with detailed explanation
#[derive(Debug, Serialize, Deserialize)]
pub struct UiCompiledInstruction {
    pub program_id_index: u8,
   //  #[serde(with = "serde_bytes_base58")] //TODO:🔴 check it. don't neede in UI
    pub accounts: Vec<u8>,
    pub data: String
}