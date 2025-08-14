use ed25519_dalek::SigningKey;
use solana_tools_lite::adapters::io_adapter::parse_signing_key_content;
use std::fs;
use std::error::Error;

use bs58;
use data_encoding::BASE64;


use solana_tools_lite::adapters::io_adapter::{is_base58, read_input_transaction, read_secret_key_file};
use solana_tools_lite::adapters::io_adapter::read_mnemonic;
use solana_tools_lite::errors::SignError;
use solana_tools_lite::models::input_transaction::InputTransaction;

// Validate that is_base58 accepts a correct Base58 string
#[test]
fn test_is_base58_valid() {
    let plain = "hello world";
    let encoded = bs58::encode(plain.as_bytes()).into_string();
    assert!(is_base58(&encoded));
}

// Reject strings containing characters outside the Base58 alphabet
#[test]
fn test_is_base58_invalid() {
    // contains characters not in Base58 alphabet
    let not_bs58 = "0OIl+/=";
    assert!(!is_base58(not_bs58));
}

// Read Base58-encoded transaction from file
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

// Read Base64-encoded transaction from file
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

// Read and trim secret key from file
#[test]
fn test_read_secret_key_file_ok() -> Result<(), Box<dyn std::error::Error>> { //TODO: 🟠 why box here?
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

// Error when secret key file is not found
#[test]
fn test_read_secret_key_file_not_found() {
    let path = "nonexistent_secret.txt";
    let err = read_secret_key_file(path).unwrap_err();
    match err {
        SignError::IoWithPath { path: Some(p), .. } => assert_eq!(p, path.to_string()),
        _ => panic!("Expected IoWithPath error for missing file, got {:?}", err),
    }
}

// Error when secret key path is a directory
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

// Parse SigningKey from 64-byte JSON array
#[test]
fn test_parse_signing_key_content_json_array_64() {
    let seed = build_seed32();
    let kp = keypair_bytes_from_seed(&seed);
    let json = format!("[{}]", kp.iter().map(|b| b.to_string()).collect::<Vec<_>>().join(","));
    let sk = parse_signing_key_content(&json).expect("parse array64");
    assert_eq!(sk.verifying_key().as_bytes(), &seed_pk_bytes(&seed));
}

// Parse SigningKey from 32-byte JSON array
#[test]
fn test_parse_signing_key_content_json_array_32() {
    let seed = build_seed32();
    let json = format!("[{}]", seed.iter().map(|b| b.to_string()).collect::<Vec<_>>().join(","));
    let sk = parse_signing_key_content(&json).expect("parse array32");
    assert_eq!(sk.verifying_key().as_bytes(), &seed_pk_bytes(&seed));
}

// Parse SigningKey from Solana CLI JSON with 64-byte Base58 secretKey
#[test]
fn test_parse_signing_key_content_keypair_json_64() {
    let seed = build_seed32();
    let kp = keypair_bytes_from_seed(&seed);
    let pk_b58 = bs58::encode(seed_pk_bytes(&seed)).into_string();
    let sec_b58 = bs58::encode(kp).into_string();
    let json = format!(
        "{{\"publicKey\":\"{}\",\"secretKey\":\"{}\"}}",
        pk_b58, sec_b58
    );
    let sk = parse_signing_key_content(&json).expect("parse keypair json 64");
    assert_eq!(sk.verifying_key().as_bytes(), &seed_pk_bytes(&seed));
}

// Parse SigningKey from Solana CLI JSON with 32-byte Base58 secretKey
#[test]
fn test_parse_signing_key_content_keypair_json_32() {
    let seed = build_seed32();
    let pk_b58 = bs58::encode(seed_pk_bytes(&seed)).into_string();
    let sec_b58 = bs58::encode(seed).into_string();
    let json = format!(
        "{{\"publicKey\":\"{}\",\"secretKey\":\"{}\"}}",
        pk_b58, sec_b58
    );
    let sk = parse_signing_key_content(&json).expect("parse keypair json 32");
    assert_eq!(sk.verifying_key().as_bytes(), &seed_pk_bytes(&seed));
}

// Parse SigningKey from raw 64-byte Base58 string
#[test]
fn test_parse_signing_key_content_raw_base58_64() {
    let seed = build_seed32();
    let kp = keypair_bytes_from_seed(&seed);
    let s = bs58::encode(kp).into_string();
    let sk = parse_signing_key_content(&s).expect("parse raw b58 64");
    assert_eq!(sk.verifying_key().as_bytes(), &seed_pk_bytes(&seed));
}

// Parse SigningKey from raw 32-byte Base58 seed
#[test]
fn test_parse_signing_key_content_raw_base58_32() {
    let seed = build_seed32();
    let s = bs58::encode(seed).into_string();
    let sk = parse_signing_key_content(&s).expect("parse raw b58 32");
    assert_eq!(sk.verifying_key().as_bytes(), &seed_pk_bytes(&seed));
}

// Error on invalid Base58 input
#[test]
fn test_parse_signing_key_content_invalid_base58() {
    let bad = "0OIl+/="; // not valid base58
    let err = parse_signing_key_content(bad).unwrap_err();
    matches!(err, SignError::InvalidBase58);
}

// Error on invalid signature length (not 32 or 64 bytes)
#[test]
fn test_parse_signing_key_content_invalid_length_array() {
    let json = "[1,2,3,4,5]"; // neither 32 nor 64
    let err = parse_signing_key_content(json).unwrap_err();
    matches!(err, SignError::InvalidKeyLength);
}

// Test helpers

fn build_seed32() -> [u8; 32] {
    let mut seed = [0u8; 32];
    for i in 0..32 { seed[i] = i as u8; }
    seed
}

fn seed_pk_bytes(seed: &[u8; 32]) -> [u8; 32] {
    let sk = SigningKey::from_bytes(seed);
    *sk.verifying_key().as_bytes()
}

fn keypair_bytes_from_seed(seed: &[u8; 32]) -> [u8; 64] {
    let pk = seed_pk_bytes(seed);
    let mut kp = [0u8; 64];
    kp[..32].copy_from_slice(seed);
    kp[32..].copy_from_slice(&pk);
    kp
}

// Read key JSON object (secretKey Base58 of 64-byte keypair) and parse
#[test]
fn test_read_secret_key_file_and_parse_keypair_json_64() -> Result<(), Box<dyn std::error::Error>> {
    let seed = build_seed32();
    let kp = keypair_bytes_from_seed(&seed);
    let pk_b58 = bs58::encode(seed_pk_bytes(&seed)).into_string();
    let sec_b58 = bs58::encode(kp).into_string();
    let json = format!(
        "{{\"publicKey\":\"{}\",\"secretKey\":\"{}\"}}",
        pk_b58, sec_b58
    );
    let path = "test_secret_key_obj64.json";
    fs::write(path, &json)?;

    let text = read_secret_key_file(path)?;
    let sk = parse_signing_key_content(&text).expect("parse keypair json 64 from file");
    assert_eq!(sk.verifying_key().as_bytes(), &seed_pk_bytes(&seed));

    fs::remove_file(path)?;
    Ok(())
}

// Read key JSON object (secretKey Base58 of 32-byte seed) and parse
#[test]
fn test_read_secret_key_file_and_parse_keypair_json_32() -> Result<(), Box<dyn std::error::Error>> {
    let seed = build_seed32();
    let pk_b58 = bs58::encode(seed_pk_bytes(&seed)).into_string();
    let sec_b58 = bs58::encode(seed).into_string();
    let json = format!(
        "{{\"publicKey\":\"{}\",\"secretKey\":\"{}\"}}",
        pk_b58, sec_b58
    );
    let path = "test_secret_key_obj32.json";
    fs::write(path, &json)?;

    let text = read_secret_key_file(path)?;
    let sk = parse_signing_key_content(&text).expect("parse keypair json 32 from file");
    assert_eq!(sk.verifying_key().as_bytes(), &seed_pk_bytes(&seed));

    fs::remove_file(path)?;
    Ok(())
}

// Read key as JSON array of 64 bytes and parse
#[test]
fn test_read_secret_key_file_and_parse_json_array_64() -> Result<(), Box<dyn std::error::Error>> {
    let seed = build_seed32();
    let kp = keypair_bytes_from_seed(&seed);
    let json = format!("[{}]", kp.iter().map(|b| b.to_string()).collect::<Vec<_>>().join(","));
    let path = "test_secret_key_arr64.json";
    fs::write(path, &json)?;

    let text = read_secret_key_file(path)?;
    let sk = parse_signing_key_content(&text).expect("parse array64 from file");
    assert_eq!(sk.verifying_key().as_bytes(), &seed_pk_bytes(&seed));

    fs::remove_file(path)?;
    Ok(())
}

// Read key as JSON array of 32 bytes and parse
#[test]
fn test_read_secret_key_file_and_parse_json_array_32() -> Result<(), Box<dyn std::error::Error>> {
    let seed = build_seed32();
    let json = format!("[{}]", seed.iter().map(|b| b.to_string()).collect::<Vec<_>>().join(","));
    let path = "test_secret_key_arr32.json";
    fs::write(path, &json)?;

    let text = read_secret_key_file(path)?;
    let sk = parse_signing_key_content(&text).expect("parse array32 from file");
    assert_eq!(sk.verifying_key().as_bytes(), &seed_pk_bytes(&seed));

    fs::remove_file(path)?;
    Ok(())
}

// Read mnemonic from file and normalize whitespace
#[test]
fn test_read_mnemonic_file_normalize() -> Result<(), Box<dyn std::error::Error>> {
    let path = "test_mnemonic.txt";
    // Write mnemonic with varied whitespace and newlines
    let content = "word1  word2\nword3\tword4  ";
    fs::write(path, content)?;
    // Should collapse whitespace into single spaces
    let normalized = read_mnemonic(path)?;
    assert_eq!(normalized, "word1 word2 word3 word4");
    fs::remove_file(path)?;
    Ok(())
}