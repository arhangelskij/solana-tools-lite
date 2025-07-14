use crate::errors::TransactionParseError;
use crate::errors::{Result};
use crate::models::input_transaction::{InputTransaction, UiTransaction};
use crate::layers::io::*;
use serde_json;

pub enum InputFormat {
    Json,
    Base64,
    Base58
}

pub enum OutputFormat {
    Json { pretty: bool },
    Base64,
    Base58,
}

pub fn read_input_transaction(input: Option<&String>) -> Result<InputTransaction> {
    let input_str = read_input(input.map(|s| s.as_str()))?;

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


use base64::{Engine as _, engine::general_purpose};
use bs58;

pub fn is_base64(s: &str) -> bool {
    general_purpose::STANDARD.decode(s).is_ok()
}

pub fn is_base58(s: &str) -> bool {
    bs58::decode(s).into_vec().is_ok()
}

/// Serialize a `UiTransaction` into the specified `OutputFormat` and write it out.
pub fn write_output_transaction(
    transaction: &UiTransaction,
    format: OutputFormat,
    output: Option<&str>,
) -> Result<()> {
    // First, serialize to JSON (for both JSON and encoded formats)
    let json_str = serde_json::to_string(transaction)
        .map_err(|e| TransactionParseError::Serialization(e.to_string()))?;

    // Build the output string based on desired format
    let out_str = match format {
        OutputFormat::Json { pretty } => {
            if pretty {
                serde_json::to_string_pretty(transaction)
                    .map_err(|e| TransactionParseError::Serialization(e.to_string()))?
            } else {
                json_str.clone()
            }
        }
        OutputFormat::Base64 => {
            general_purpose::STANDARD.encode(&json_str)
        }
        OutputFormat::Base58 => {
            bs58::encode(&json_str).into_string()
        }
    };

    // Write to file or stdout
    write_output(output, &out_str)
        .map_err(|e| e)?;

    Ok(())
}
