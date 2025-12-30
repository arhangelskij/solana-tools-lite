use crate::errors::TransactionParseError;
use crate::models::input_transaction::{InputTransaction, UiTransaction};
use data_encoding::BASE64;

/// Backward-friendly wrapper: same detection, but accepts Option<&str>.
/// Returns InvalidFormat if input is None.
pub fn parse_input_transaction(
    input: Option<&str>,
) -> Result<InputTransaction, TransactionParseError> {
    match input {
        Some(s) => parse_input_transaction_str(s),
        None => Err(TransactionParseError::InvalidFormat("missing input".into())),
    }
}

/// Detect input format from a string and return the corresponding variant.
///
/// Behavior
/// - Detects JSON (UiTransaction), then Base64, then Base58
/// - Trims the input before checks
/// - Returns InvalidFormat on unknown content
///
/// This function does not perform any I/O and expects the caller to have read the text already.
fn parse_input_transaction_str(s: &str) -> Result<InputTransaction, TransactionParseError> {
    let trimmed = s.trim();

    if let Ok(json_tx) = serde_json::from_str::<UiTransaction>(trimmed) {
        return Ok(InputTransaction::Json(json_tx));
    }

    if is_base64(trimmed) {
        return Ok(InputTransaction::Base64(trimmed.to_string()));
    }

    if is_base58(trimmed) {
        return Ok(InputTransaction::Base58(trimmed.to_string()));
    }

    Err(TransactionParseError::InvalidFormat(
        "Unknown input format".into(),
    ))
}

/// Returns true if `s` is non-empty and valid Base64.
///
/// Notes
/// - Trims input before validation
/// - Rejects empty strings
/// - Performs a cheap alphabet/length check before decode attempt
pub fn is_base64(s: &str) -> bool {
    let s = s.trim();
    if s.is_empty() || s.len() % 4 != 0 {
        return false;
    }
    if !s
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=')
    {
        return false;
    }
    BASE64.decode(s.as_bytes()).is_ok()
}

/// Returns true if `s` is non-empty and valid Base58.
///
/// Notes
/// - Trims input before validation
/// - Rejects empty strings
pub fn is_base58(s: &str) -> bool {
    let s = s.trim();
    if s.is_empty() {
        return false;
    }
    bs58::decode(s).into_vec().is_ok()
}
