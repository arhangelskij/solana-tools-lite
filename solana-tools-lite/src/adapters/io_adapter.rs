use crate::errors::Result;
use crate::errors::SignError;
use crate::errors::TransactionParseError;
use crate::layers::io::*;
use crate::models::input_transaction::{InputTransaction, UiTransaction};
use serde_json;
use std::io;
use std::path::Path;

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
//TODO: 🟡 why sign error?
fn read_raw_input(input: Option<&str>) -> std::result::Result<String, SignError> {
    if let Some(p) = input {
        let p = p.trim();

        if p != "-" {
            let path = Path::new(p);

            if path.exists() {
                println!("🟡 path is exist!");
                if path.is_file() {
                    return read_input(Some(p));
                } else {
                    println!(" it isnt file 🤷🏾‍♂️");
                    // path exists but is not a file (e.g. a directory)
                    return Err(SignError::IoWithPath {
                        source: io::Error::new(
                            io::ErrorKind::InvalidInput,
                            "input path is not a file",
                        ),
                        path: Some(p.to_string()),
                    });
                }
            } else {
                // literal input string
                return Ok(p.to_string());
            }
        }
    }
    // None or "-" => read from stdin
    read_input(None)
}

pub fn read_input_transaction(input: Option<&str>) -> Result<InputTransaction> {
    let input_str = read_raw_input(input)
        .map_err(|e| TransactionParseError::InvalidFormat(format!("I/O error: {}", e)))?;

    //TODO: 🔴 check it
    let trimmed_input = input_str.trim();

    if let Ok(json_tx) = serde_json::from_str::<UiTransaction>(&trimmed_input) {
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

use bs58;
use data_encoding::BASE64;

fn is_base64(s: &str) -> bool {
    // check safety
    if s.len() % 4 != 0 {
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
        OutputFormat::Base64 => BASE64.encode(&json_str.as_bytes()),
        OutputFormat::Base58 => bs58::encode(&json_str).into_string(),
    };

    // Write to file or stdout
    write_output(output, &out_str).map_err(|e| e)?;

    Ok(())
}
