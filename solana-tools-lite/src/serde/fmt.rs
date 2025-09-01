use crate::errors::TransactionParseError;
use crate::models::input_transaction::UiTransaction;
use data_encoding::BASE64;

#[derive(Clone, Copy, Debug)]
pub enum OutputFormat {
    Json { pretty: bool },
    Base64,
    Base58,
}

/// Serialize a `UiTransaction` according to the specified `OutputFormat`.
/// Note: current semantics encode Base64/Base58 from JSON string representation.
pub fn encode_ui_transaction(tx: &UiTransaction, format: OutputFormat) -> Result<String, TransactionParseError> {
    let json_str = serde_json::to_string(tx)
        .map_err(|e| TransactionParseError::Serialization(e.to_string()))?;

    let out = match format {
        OutputFormat::Json { pretty } => {
            if pretty {
                serde_json::to_string_pretty(tx)
                    .map_err(|e| TransactionParseError::Serialization(e.to_string()))?
            } else {
                json_str
            }
        }
        OutputFormat::Base64 => BASE64.encode(json_str.as_bytes()),
        OutputFormat::Base58 => bs58::encode(&json_str).into_string(),
    };

    Ok(out)
}