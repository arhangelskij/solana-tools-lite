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
    Base58
}

pub enum OutputFormat {
    Json { pretty: bool },
    Base64,
    Base58
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

pub fn read_secret_key_file(path: &str) -> std::result::Result<String, SignError> {
    // For security reasons, reading secret keys from stdin is disabled.
    if path == "-" {
        return Err(SignError::IoWithPath {
            source: std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "reading secret key from stdin is disabled",
            ),
            path: Some("-".to_string()),
        });
    }

    let p = Path::new(path);

    if !p.exists() {
        return Err(SignError::IoWithPath {
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "secret key file not found"),
            path: Some(path.to_string()),
        });
    }

    if !p.is_file() {
        return Err(SignError::IoWithPath {
            source: std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "secret key path is not a file",
            ),
            path: Some(path.to_string()),
        });
    }

    let s = read_input(Some(path))?;

    Ok(s.trim().to_string())
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
    output: Option<&str>
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

/////////////
use crate::models::keypair_json::KeypairJson;
use ed25519_dalek::SigningKey;
use std::convert::TryInto;

/// Build SigningKey from decoded bytes: accept 32-byte seed or 64-byte keypair bytes.
fn signing_key_from_decoded(bytes: Vec<u8>) -> Result<SigningKey, SignError> {
    match bytes.len() {
        64 => {
            // keypair bytes => take first 32 as seed
            let mut seed = [0u8; 32];
            seed.copy_from_slice(&bytes[..32]);
            Ok(SigningKey::from_bytes(&seed))
        }
        32 => {
            // raw 32-byte seed
            let arr: [u8; 32] = bytes
                .as_slice()
                .try_into()
                .map_err(|_| SignError::InvalidKeyLength)?;
            Ok(SigningKey::from_bytes(&arr))
        }
        _ => Err(SignError::InvalidKeyLength),
    }
}

/// Parse signing key from *content* (no I/O here).
/// Supported formats:
/// 1) JSON array of 64 bytes: [u8; 64]
/// 2) Keypair JSON: {"publicKey": "...", "secretKey": "<base58>"}
/// 3) Raw Base58 string (32-byte seed or 64-byte keypair bytes)
pub fn parse_signing_key_content(content: &str) -> Result<SigningKey, SignError> {
    let text = content.trim();

    // 1) JSON array of bytes (supports 64-byte keypair or 32-byte seed)
    if let Ok(arr) = serde_json::from_str::<Vec<u8>>(text) {
        return match arr.len() {
            64 => {
                let mut seed = [0u8; 32];
                seed.copy_from_slice(&arr[..32]);
                
                Ok(SigningKey::from_bytes(&seed))
            }
            32 => {
                let mut seed = [0u8; 32];
                seed.copy_from_slice(&arr[..32]);
                
                Ok(SigningKey::from_bytes(&seed))
            }
            _ => Err(SignError::InvalidKeyLength)
        };
    }

    // 2) Keypair JSON with Base58 secretKey
    if let Ok(kp_json) = serde_json::from_str::<KeypairJson>(text) {
        let sec = kp_json.secret_key.trim();
        let bytes = bs58::decode(sec)
            .into_vec()
            .map_err(|_| SignError::InvalidBase58)?;
        return signing_key_from_decoded(bytes);
    }

    // 3) Raw Base58 string
    let decoded = bs58::decode(text)
        .into_vec()
        .map_err(|_| SignError::InvalidBase58)?;
    signing_key_from_decoded(decoded)
}


/// Mnemonic 

/// Read mnemonic from file or stdin (`-`) and normalize whitespace.
pub fn read_mnemonic(input: &str) -> Result<String, SignError> {
    let raw = read_input(if input == "-" { None } else { Some(input) })?;
    Ok(raw.split_whitespace().collect::<Vec<_>>().join(" "))
}

//TODO: 🟡 unused?
/// Read a single-line secret-like text (file or stdin), trimmed.
pub fn read_text(input: &str) -> Result<String, SignError> {
    let raw = read_input(if input == "-" { None } else { Some(input) })?;
    Ok(raw.trim().to_string())
}