use crate::errors::Result;
use crate::errors::TransactionParseError;
use crate::models::input_transaction::{InputTransaction, UiTransaction};

pub enum InputFormat {
    Json,
    Base64,
    Base58,
}

pub enum OutputFormat {
    Json { pretty: bool },
    Base64,
    Base58,
}

pub fn read_input_transaction(input: Option<&String>) -> Result<InputTransaction> {
    let input_str = crate::utils::read_stdin_or_file(input)?;

    if let Ok(json_tx) = serde_json::from_str::<UiTransaction>(&input_str) {
        return Ok(InputTransaction::Json(json_tx));
    }

    if is_base64(&input_str) {
        return Ok(InputTransaction::Base64(input_str));
    }

    if is_base58(&input_str) {
        return Ok(InputTransaction::Base58(input_str));
    }

    Err(TransactionParseError::InvalidFormat("Unknown input format".into()).into())
}

// pub fn write_input_transaction(
//     output: Option<&String>,
//     tx: &InputTransaction,
//     pretty: bool,
// ) -> Result<()> {
// }

use base64::{engine::general_purpose, Engine as _};
use bs58;

pub fn is_base64(s: &str) -> bool {
    general_purpose::STANDARD.decode(s).is_ok()
}

pub fn is_base58(s: &str) -> bool {
    bs58::decode(s).into_vec().is_ok()
}