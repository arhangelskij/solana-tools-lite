use std::fs;
use std::error::Error;

use bs58;
use data_encoding::BASE64;


use solana_tools_lite::adapters::io_adapter::{is_base58, read_input_transaction, read_secret_key_file};
use solana_tools_lite::errors::SignError;
use solana_tools_lite::models::input_transaction::InputTransaction;

#[test]
fn test_is_base58_valid() {
    let plain = "hello world";
    let encoded = bs58::encode(plain.as_bytes()).into_string();
    assert!(is_base58(&encoded));
}

#[test]
fn test_is_base58_invalid() {
    // contains characters not in Base58 alphabet
    let not_bs58 = "0OIl+/=";
    assert!(!is_base58(not_bs58));
}

#[test]
fn test_read_input_transaction_base58() -> Result<(), Box<dyn Error>> {
    let path = "test_io_adapter_bs58.txt";
    let plain = "adapter test";
    let encoded = bs58::encode(plain.as_bytes()).into_string();
    
    fs::write(path, &encoded)?;
    
    let variant = read_input_transaction(Some(&path.to_string()))?;
    match variant {
        InputTransaction::Base58(s) => assert_eq!(s, encoded),
        InputTransaction::Base64(_) | InputTransaction::Json(_) => {
            panic!("Expected Base58 variant")
        }
    }
    fs::remove_file(path)?;
    Ok(())
}

#[test]
fn test_read_input_transaction_base64() -> Result<(), Box<dyn Error>> {
    let path = "test_io_adapter_b64.txt";
    let plain = "adapter test";
    let encoded = BASE64.encode(plain.as_bytes());
    
    fs::write(path, &encoded)?;
    
    let variant = read_input_transaction(Some(&path.to_string()))?;
    match variant {
        InputTransaction::Base64(s) => assert_eq!(s, encoded),
        InputTransaction::Base58(_) | InputTransaction::Json(_) => {
            panic!("Expected Base64 variant")
        }
    }
    fs::remove_file(path)?;
    Ok(())
}

#[test]
fn test_read_secret_key_file_ok() -> Result<(), Box<dyn std::error::Error>> {
    let path = "test_secret_key.txt";
    let secret = "mysecretkey";
    // Write secret with trailing newline
    fs::write(path, format!("{}\n", secret))?;
    // Should read and trim newline
    let s = read_secret_key_file(path)?;
    assert_eq!(s, secret);
    fs::remove_file(path)?;
    Ok(())
}

#[test]
fn test_read_secret_key_file_not_found() {
    let path = "nonexistent_secret.txt";
    let err = read_secret_key_file(path).unwrap_err();
    match err {
        SignError::IoWithPath { path: Some(p), .. } => assert_eq!(p, path.to_string()),
        _ => panic!("Expected IoWithPath error for missing file, got {:?}", err),
    }
}

#[test]
fn test_read_secret_key_file_not_a_file() -> Result<(), Box<dyn std::error::Error>> {
    let dir = "test_secret_dir";
    fs::create_dir(dir)?;
    let err = read_secret_key_file(dir).unwrap_err();
    match err {
        SignError::IoWithPath { path: Some(p), .. } => assert_eq!(p, dir.to_string()),
        _ => panic!("Expected IoWithPath error for directory, got {:?}", err),
    }
    fs::remove_dir(dir)?;
    Ok(())
}