use std::convert::TryFrom;

use crate::models::input_transaction::{InputTransaction, UiTransaction};
use crate::models::transaction::Transaction;

use crate::errors::TransactionParseError;
use bs58;
use data_encoding::BASE64;
use serde_json;
use crate::codec::deserialize_transaction;

/// Convert an `InputTransaction` into a domain `Transaction`.
///
/// - **Base64**: Decode wire-format bytes into a `Transaction`.
/// - **Base58**: Decode Base58, then parse the JSON `UiTransaction`.
/// - **Json**: Convert the structured `UiTransaction` directly.
///
/// # Errors
/// Returns `TransactionParseError` on Base64/Base58 decoding, wire deserialization, or JSON parsing.
impl TryFrom<InputTransaction> for Transaction {
    type Error = TransactionParseError;

    fn try_from(input: InputTransaction) -> Result<Self, Self::Error> {
        match input {
            InputTransaction::Base64(s) => {
                // Decode Base64-encoded raw Solana transaction bytes
                let raw = BASE64
                    .decode(s.as_bytes())
                    .map_err(|e| TransactionParseError::InvalidBase64(e.to_string()))?;

                // Decode raw wire bytes into a Transaction struct.
                let tx = deserialize_transaction(&raw)
                    .map_err(|e| TransactionParseError::InvalidFormat(e.to_string()))?;

                Ok(tx)
            }
            InputTransaction::Base58(s) => {
                // Decode Base58-encoded JSON
                let decoded = bs58::decode(s)
                    .into_vec()
                    .map_err(|e| TransactionParseError::InvalidFormat(e.to_string()))?;

                let ui_tx: UiTransaction = serde_json::from_slice(&decoded)
                    .map_err(|e| TransactionParseError::InvalidFormat(e.to_string()))?;
                Transaction::try_from(ui_tx)
            }
            InputTransaction::Json(ui_tx) => Transaction::try_from(ui_tx),
        }
    }
}
