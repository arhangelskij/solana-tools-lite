use serde::Deserialize;
use crate::models::transaction::MessageHeader;

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

#[derive(Debug, Deserialize)]
pub struct UiRawMessage {
    pub header: MessageHeader,
    pub account_keys: Vec<String>,
    pub recent_blockhash: String,
    pub instructions: Vec<UiCompiledInstruction>,
}

#[derive(Debug, Deserialize)]
pub struct UiCompiledInstruction {
    pub program_id_index: u8,
    pub accounts: Vec<u8>,
    pub data: String
}