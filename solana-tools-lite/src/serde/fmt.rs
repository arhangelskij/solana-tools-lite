use crate::errors::TransactionParseError;
use crate::models::input_transaction::UiTransaction;

#[derive(Clone, Copy, Debug)]
pub enum OutputFormat {
    Json { pretty: bool },
    Base64,
    Base58
}
//TODO: 🔴 6/9
/// Serialize a `UiTransaction` to JSON according to the specified `OutputFormat`.
///
/// Supported:
/// - `OutputFormat::Json { pretty }` → JSON string (pretty or compact)
///
/// Not supported here:
/// - `OutputFormat::Base64` / `OutputFormat::Base58` — these are reserved for wire bytes of the
///   signed domain `Transaction` (use adapters::io_adapter::write_signed_transaction).
pub fn encode_ui_transaction(
    tx: &UiTransaction,
    format: OutputFormat,
) -> Result<String, TransactionParseError> {
    match format {
        OutputFormat::Json { pretty } => {
            if pretty {
                serde_json::to_string_pretty(tx)
                    .map_err(|e| TransactionParseError::Serialization(e.to_string()))
            } else {
                serde_json::to_string(tx)
                    .map_err(|e| TransactionParseError::Serialization(e.to_string()))
            }
        }
        OutputFormat::Base64 | OutputFormat::Base58 => Err(TransactionParseError::InvalidFormat(
            "Base64/Base58 for UiTransaction is not supported; use wire serializer on domain Transaction"
                .into(),
        )),
    }
}
