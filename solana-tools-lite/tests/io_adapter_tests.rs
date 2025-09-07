use ed25519_dalek::SigningKey;
use solana_tools_lite::crypto::helpers::parse_signing_key_content;
use solana_tools_lite::errors::{IoError, Result, SignError, ToolError};
use std::fs;

use bs58;
use data_encoding::BASE64;

use solana_tools_lite::adapters::io_adapter::read_mnemonic;
use solana_tools_lite::adapters::io_adapter::read_text_source;
use solana_tools_lite::adapters::io_adapter::{
    read_input_transaction, read_secret_key_file, write_signed_transaction,
};
use solana_tools_lite::adapters::io_adapter::{read_message, read_pubkey, read_signature};
#[path = "utils.rs"]
mod test_utils;
use solana_tools_lite::models::input_transaction::InputTransaction;
use solana_tools_lite::serde::fmt::OutputFormat;
use solana_tools_lite::serde::input_tx::is_base58;

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
fn test_read_input_transaction_base58() -> Result<()> {
    let path = "test_io_adapter_bs58.txt";
    let plain = "adapter test";
    let encoded = bs58::encode(plain.as_bytes()).into_string();

    fs::write(path, &encoded).map_err(|e| ToolError::Io(IoError::Io(e)))?;

    let variant = read_input_transaction(Some(&path.to_string()))?;
    match variant {
        InputTransaction::Base58(s) => assert_eq!(s, encoded),
        InputTransaction::Base64(_) | InputTransaction::Json(_) => {
            panic!("Expected Base58 variant")
        }
    }
    fs::remove_file(path).map_err(|e| ToolError::Io(IoError::Io(e)))?;
    Ok(())
}

// Read Base64-encoded transaction from file
#[test]
fn test_read_input_transaction_base64() -> Result<()> {
    let path = "test_io_adapter_b64.txt";
    let plain = "adapter test";
    let encoded = BASE64.encode(plain.as_bytes());

    fs::write(path, &encoded).map_err(|e| ToolError::Io(IoError::Io(e)))?;

    let variant = read_input_transaction(Some(&path.to_string()))?;
    match variant {
        InputTransaction::Base64(s) => assert_eq!(s, encoded),
        InputTransaction::Base58(_) | InputTransaction::Json(_) => {
            panic!("Expected Base64 variant")
        }
    }
    fs::remove_file(path).map_err(|e| ToolError::Io(IoError::Io(e)))?;
    Ok(())
}

// Read and trim secret key from file
#[test]
fn test_read_secret_key_file_ok() -> Result<()> {
    //TODO: 🟠 why box here?
    let path = "test_secret_key.txt";
    let secret = "mysecretkey";

    // Write secret with trailing newline
    fs::write(path, format!("{}\n", secret)).map_err(|e| ToolError::Io(IoError::Io(e)))?;

    // Should read and trim newline
    let s = read_secret_key_file(path)?;
    assert_eq!(s, secret);

    fs::remove_file(path).map_err(|e| ToolError::Io(IoError::Io(e)))?;
    Ok(())
}

// Error when secret key file is not found
#[test]
fn test_read_secret_key_file_not_found() {
    let path = "nonexistent_secret.txt";
    let err = read_secret_key_file(path).unwrap_err();
    match err {
        ToolError::Io(IoError::IoWithPath { path: Some(p), .. }) => assert_eq!(p, path.to_string()),
        _ => panic!("Expected Io(IoWithPath) for missing file, got {:?}", err),
    }
}

// Error when secret key path is a directory
#[test]
fn test_read_secret_key_file_not_a_file() -> Result<()> {
    let dir = "test_secret_dir";
    fs::create_dir(dir).map_err(|e| ToolError::Io(IoError::Io(e)))?;
    let err = read_secret_key_file(dir).unwrap_err();
    match err {
        ToolError::Io(IoError::IoWithPath { path: Some(p), .. }) => assert_eq!(p, dir.to_string()),
        _ => panic!("Expected Io(IoWithPath) for directory, got {:?}", err),
    }
    fs::remove_dir(dir).map_err(|e| ToolError::Io(IoError::Io(e)))?;
    Ok(())
}

// Parse SigningKey from 64-byte JSON array
#[test]
fn test_parse_signing_key_content_json_array_64() {
    let seed = build_seed32();
    let kp = keypair_bytes_from_seed(&seed);
    let json = format!(
        "[{}]",
        kp.iter()
            .map(|b| b.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
    let sk = parse_signing_key_content(&json).expect("parse array64");
    assert_eq!(sk.verifying_key().as_bytes(), &seed_pk_bytes(&seed));
}

// Parse SigningKey from 32-byte JSON array
#[test]
fn test_parse_signing_key_content_json_array_32() {
    let seed = build_seed32();
    let json = format!(
        "[{}]",
        seed.iter()
            .map(|b| b.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
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
//TODO: 🟠 do we need in the place test_parse_signing_key_content_invalid_base58?
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
    for i in 0..32 {
        seed[i] = i as u8;
    }
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
fn test_read_secret_key_file_and_parse_keypair_json_64() -> Result<()> {
    let seed = build_seed32();
    let kp = keypair_bytes_from_seed(&seed);
    let pk_b58 = bs58::encode(seed_pk_bytes(&seed)).into_string();
    let sec_b58 = bs58::encode(kp).into_string();
    let json = format!(
        "{{\"publicKey\":\"{}\",\"secretKey\":\"{}\"}}",
        pk_b58, sec_b58
    );
    let path = "test_secret_key_obj64.json";
    fs::write(path, &json).map_err(|e| ToolError::Io(IoError::Io(e)))?;

    let text = read_secret_key_file(path)?;
    let sk = parse_signing_key_content(&text).expect("parse keypair json 64 from file");
    assert_eq!(sk.verifying_key().as_bytes(), &seed_pk_bytes(&seed));

    fs::remove_file(path).map_err(|e| ToolError::Io(IoError::Io(e)))?;
    Ok(())
}

// Read key JSON object (secretKey Base58 of 32-byte seed) and parse
#[test]
fn test_read_secret_key_file_and_parse_keypair_json_32() -> Result<()> {
    let seed = build_seed32();
    let pk_b58 = bs58::encode(seed_pk_bytes(&seed)).into_string();
    let sec_b58 = bs58::encode(seed).into_string();
    let json = format!(
        "{{\"publicKey\":\"{}\",\"secretKey\":\"{}\"}}",
        pk_b58, sec_b58
    );
    let path = "test_secret_key_obj32.json";
    fs::write(path, &json).map_err(|e| ToolError::Io(IoError::Io(e)))?;

    let text = read_secret_key_file(path)?;
    let sk = parse_signing_key_content(&text).expect("parse keypair json 32 from file");
    assert_eq!(sk.verifying_key().as_bytes(), &seed_pk_bytes(&seed));

    fs::remove_file(path).map_err(|e| ToolError::Io(IoError::Io(e)))?;
    Ok(())
}

// Read key as JSON array of 64 bytes and parse
#[test]
fn test_read_secret_key_file_and_parse_json_array_64() -> Result<()> {
    let seed = build_seed32();
    let kp = keypair_bytes_from_seed(&seed);
    let json = format!(
        "[{}]",
        kp.iter()
            .map(|b| b.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
    let path = "test_secret_key_arr64.json";
    fs::write(path, &json).map_err(|e| ToolError::Io(IoError::Io(e)))?;

    let text = read_secret_key_file(path)?;
    let sk = parse_signing_key_content(&text).expect("parse array64 from file");
    assert_eq!(sk.verifying_key().as_bytes(), &seed_pk_bytes(&seed));

    fs::remove_file(path).map_err(|e| ToolError::Io(IoError::Io(e)))?;
    Ok(())
}

// Read key as JSON array of 32 bytes and parse
#[test]
fn test_read_secret_key_file_and_parse_json_array_32() -> Result<()> {
    let seed = build_seed32();
    let json = format!(
        "[{}]",
        seed.iter()
            .map(|b| b.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
    let path = "test_secret_key_arr32.json";
    fs::write(path, &json).map_err(|e| ToolError::Io(IoError::Io(e)))?;

    let text = read_secret_key_file(path)?;
    let sk = parse_signing_key_content(&text).expect("parse array32 from file");
    assert_eq!(sk.verifying_key().as_bytes(), &seed_pk_bytes(&seed));

    fs::remove_file(path).map_err(|e| ToolError::Io(IoError::Io(e)))?;
    Ok(())
}

// Read mnemonic from file and normalize whitespace
#[test]
fn test_read_mnemonic_file_normalize() -> Result<()> {
    let path = "test_mnemonic.txt";
    // Write mnemonic with varied whitespace and newlines
    let content = "word1  word2\nword3\tword4  ";
    fs::write(path, content).map_err(|e| ToolError::Io(IoError::Io(e)))?;
    // Should collapse whitespace into single spaces
    let normalized = read_mnemonic(path)?;
    assert_eq!(normalized, "word1 word2 word3 word4");
    fs::remove_file(path).map_err(|e| ToolError::Io(IoError::Io(e)))?;
    Ok(())
}

// Inline handling and read_text_source behavior

#[test]
fn test_read_input_transaction_invalid_path_errors() {
    // Non-existent path should be treated as an error (no inline fallback)
    let err = read_input_transaction(Some(&"no_such_file_123456.json".to_string())).unwrap_err();
    match err {
        ToolError::Io(IoError::IoWithPath { .. }) => {}
        other => panic!("Expected Io(IoWithPath) for nonexistent path, got {other:?}"),
    }
}

#[test]
fn test_read_input_transaction_json_from_file() -> Result<()> {
    // Minimal valid UiTransaction JSON
    let json = r#"{
        "signatures": [],
        "message": {
            "header": {
                "num_required_signatures": 1,
                "num_readonly_signed_accounts": 0,
                "num_readonly_unsigned_accounts": 1
            },
            "account_keys": [
                "11111111111111111111111111111111",
                "11111111111111111111111111111111"
            ],
            "recent_blockhash": "11111111111111111111111111111111",
            "instructions": []
        }
    }"#;
    let path = "test_ui_tx.json";
    fs::write(path, json).map_err(|e| ToolError::Io(IoError::Io(e)))?;
    let variant = read_input_transaction(Some(&path.to_string()))?;
    match variant {
        InputTransaction::Json(_) => {}
        other => panic!("Expected Json variant, got {other:?}"),
    }
    fs::remove_file(path).map_err(|e| ToolError::Io(IoError::Io(e)))?;
    Ok(())
}

// Regression tests: a nonexistent path that LOOKS like valid data
// must be treated as a path error (no inline fallback)

#[test]
fn test_read_input_transaction_nonexistent_path_looks_like_base58_should_error() {
    // String of '1' chars is valid Base58 but should not be treated as inline
    let fake_path = "11111111111111111111111111111111".to_string();
    let err = read_input_transaction(Some(&fake_path)).unwrap_err();
    match err {
        ToolError::Io(IoError::IoWithPath { .. }) => {}
        other => panic!("Expected Io(IoWithPath) for nonexistent path, got {other:?}"),
    }
}

#[test]
fn test_read_input_transaction_nonexistent_path_looks_like_base64_should_error() {
    // "QUJD" is valid Base64 ("ABC") but should not be treated as inline
    let fake_path = "QUJD".to_string();
    let err = read_input_transaction(Some(&fake_path)).unwrap_err();
    match err {
        ToolError::Io(IoError::IoWithPath { .. }) => {}
        other => panic!("Expected Io(IoWithPath) for nonexistent path, got {other:?}"),
    }
}

#[test]
fn test_read_input_transaction_nonexistent_path_looks_like_json_should_error() {
    // Minimal valid UiTransaction JSON passed as a "path" must not be treated as inline
    let fake_path = r#"{\"signatures\":[],\"message\":{\"header\":{\"num_required_signatures\":1,\"num_readonly_signed_accounts\":0,\"num_readonly_unsigned_accounts\":1},\"account_keys\":[\"11111111111111111111111111111111\",\"11111111111111111111111111111111\"],\"recent_blockhash\":\"11111111111111111111111111111111\",\"instructions\":[]}}"#.to_string();
    let err = read_input_transaction(Some(&fake_path)).unwrap_err();
    match err {
        ToolError::Io(IoError::IoWithPath { .. }) => {}
        other => panic!("Expected Io(IoWithPath) for nonexistent path, got {other:?}"),
    }
}

#[test]
fn test_read_text_source_inline_ok() -> Result<()> {
    let s = read_text_source(Some("hello"), None, true)?;
    assert_eq!(s, "hello");
    Ok(())
}

#[test]
fn test_read_text_source_file_ok() -> Result<()> {
    let path = "test_rts.txt";
    fs::write(path, "abc").map_err(|e| ToolError::Io(IoError::Io(e)))?;
    let s = read_text_source(None, Some(path), true)?;
    assert_eq!(s, "abc");
    fs::remove_file(path).map_err(|e| ToolError::Io(IoError::Io(e)))?;
    Ok(())
}

#[test]
fn test_read_text_source_both_should_error() {
    let err = read_text_source(Some("a"), Some("b"), true).unwrap_err();
    match err {
        ToolError::InvalidInput(msg) => assert!(msg.contains("either inline value or --from-file")),
        other => panic!("Expected InvalidInput, got {other:?}"),
    }
}

#[test]
fn test_read_text_source_none_should_error() {
    let err = read_text_source(None, None, true).unwrap_err();
    match err {
        ToolError::InvalidInput(msg) => assert!(msg.contains("missing input")),
        other => panic!("Expected InvalidInput, got {other:?}"),
    }
}

#[test]
fn test_read_text_source_stdin_disallowed() {
    let err = read_text_source(None, Some("-"), false).unwrap_err();
    match err {
        ToolError::InvalidInput(msg) => assert!(msg.contains("stdin is disabled")),
        other => panic!("Expected InvalidInput, got {other:?}"),
    }
}

// New helpers semantics: message preserves bytes, signature/pubkey are trimmed

#[test]
fn test_read_message_preserves_newline_from_file() -> Result<()> {
    let path = "test_read_message_bytes.txt";
    std::fs::write(path, "hello\n").map_err(|e| ToolError::Io(IoError::Io(e)))?;
    let s = read_message(None, Some(path))?;
    assert_eq!(s, "hello\n");
    std::fs::remove_file(path).map_err(|e| ToolError::Io(IoError::Io(e)))?;
    Ok(())
}

#[test]
fn test_read_signature_trims_from_file() -> Result<()> {
    let path = "test_read_signature_trim.txt";
    std::fs::write(path, "SIG\n").map_err(|e| ToolError::Io(IoError::Io(e)))?;
    let s = read_signature(None, Some(path))?;
    assert_eq!(s, "SIG");
    std::fs::remove_file(path).map_err(|e| ToolError::Io(IoError::Io(e)))?;
    Ok(())
}

#[test]
fn test_read_pubkey_trims_from_file() -> Result<()> {
    let path = "test_read_pubkey_trim.txt";
    std::fs::write(path, "PK \n").map_err(|e| ToolError::Io(IoError::Io(e)))?;
    let s = read_pubkey(None, Some(path))?;
    assert_eq!(s, "PK");
    std::fs::remove_file(path).map_err(|e| ToolError::Io(IoError::Io(e)))?;
    Ok(())
}

#[test]
fn test_read_message_inline_preserves() -> Result<()> {
    let s = read_message(Some("abc\n"), None)?;
    assert_eq!(s, "abc\n");
    Ok(())
}

#[test]
fn test_read_signature_inline_trims() -> Result<()> {
    let s = read_signature(Some("sig \n"), None)?;
    assert_eq!(s, "sig");
    Ok(())
}

#[test]
fn test_read_pubkey_inline_trims() -> Result<()> {
    let s = read_pubkey(Some("pk \n"), None)?;
    assert_eq!(s, "pk");
    Ok(())
}

// New helpers: error matrix and path propagation

#[test]
fn test_read_message_both_should_error() {
    let err = read_message(Some("a"), Some("b")).unwrap_err();
    match err {
        ToolError::InvalidInput(msg) => assert!(msg.contains("either inline value or --from-file")),
        other => panic!("Expected InvalidInput, got {other:?}"),
    }
}

#[test]
fn test_read_message_none_should_error() {
    let err = read_message(None, None).unwrap_err();
    match err {
        ToolError::InvalidInput(msg) => assert!(msg.contains("missing input")),
        other => panic!("Expected InvalidInput, got {other:?}"),
    }
}

#[test]
fn test_read_signature_both_should_error() {
    let err = read_signature(Some("a"), Some("b")).unwrap_err();
    match err {
        ToolError::InvalidInput(msg) => assert!(msg.contains("either inline value or --from-file")),
        other => panic!("Expected InvalidInput, got {other:?}"),
    }
}

#[test]
fn test_read_signature_none_should_error() {
    let err = read_signature(None, None).unwrap_err();
    match err {
        ToolError::InvalidInput(msg) => assert!(msg.contains("missing input")),
        other => panic!("Expected InvalidInput, got {other:?}"),
    }
}

#[test]
fn test_read_pubkey_both_should_error() {
    let err = read_pubkey(Some("a"), Some("b")).unwrap_err();
    match err {
        ToolError::InvalidInput(msg) => assert!(msg.contains("either inline value or --from-file")),
        other => panic!("Expected InvalidInput, got {other:?}"),
    }
}

#[test]
fn test_read_pubkey_none_should_error() {
    let err = read_pubkey(None, None).unwrap_err();
    match err {
        ToolError::InvalidInput(msg) => assert!(msg.contains("missing input")),
        other => panic!("Expected InvalidInput, got {other:?}"),
    }
}

#[test]
fn test_read_message_nonexistent_file_has_path_context() {
    let err = read_message(None, Some("no_such_file_abc.txt")).unwrap_err();
    match err {
        ToolError::Io(IoError::IoWithPath { path: Some(p), .. }) => {
            assert!(p.ends_with("no_such_file_abc.txt"))
        }
        other => panic!("Expected IoWithPath, got {other:?}"),
    }
}

#[test]
fn test_read_signature_nonexistent_file_has_path_context() {
    let err = read_signature(None, Some("no_sig_file_abc.txt")).unwrap_err();
    match err {
        ToolError::Io(IoError::IoWithPath { path: Some(p), .. }) => {
            assert!(p.ends_with("no_sig_file_abc.txt"))
        }
        other => panic!("Expected IoWithPath, got {other:?}"),
    }
}

#[test]
fn test_read_pubkey_nonexistent_file_has_path_context() {
    let err = read_pubkey(None, Some("no_pk_file_abc.txt")).unwrap_err();
    match err {
        ToolError::Io(IoError::IoWithPath { path: Some(p), .. }) => {
            assert!(p.ends_with("no_pk_file_abc.txt"))
        }
        other => panic!("Expected IoWithPath, got {other:?}"),
    }
}

// Adapter: write_signed_transaction(JSON) writes via write_output (0644) and overwrites
#[test]
fn test_write_output_to_file_adapter() -> Result<()> {
    // Minimal valid UiTransaction via helper
    let ui = test_utils::generate_ui_transaction(
        1,
        vec![
            "11111111111111111111111111111111",
            "11111111111111111111111111111111",
        ],
        "11111111111111111111111111111111",
        0,
        vec![],
        "",
    );

    let path = "test_write_adapter.json";

    // Convert UiTransaction -> domain Transaction for adapter API
    let tx = solana_tools_lite::models::transaction::Transaction::try_from(ui)
        .expect("ui -> tx");

    // First write (create) in JSON mode
    write_signed_transaction(&tx, OutputFormat::Json { pretty: false }, Some(path), false)?;
    let content1 = std::fs::read_to_string(path).map_err(|e| ToolError::Io(IoError::Io(e)))?;
    let ui_again = solana_tools_lite::models::input_transaction::UiTransaction::from(&tx);
    assert_eq!(content1, serde_json::to_string(&ui_again).unwrap());

    // Second write (overwrite) with different content
    let ui2 = test_utils::generate_ui_transaction(
        2,
        vec![
            "11111111111111111111111111111111",
            "11111111111111111111111111111111",
        ],
        "11111111111111111111111111111111",
        0,
        vec![],
        "",
    );
    // Overwrite with a different UiTransaction
    let tx2 = solana_tools_lite::models::transaction::Transaction::try_from(ui2)
        .expect("ui2 -> tx");
    write_signed_transaction(&tx2, OutputFormat::Json { pretty: false }, Some(path), true)?;

    let content2 = std::fs::read_to_string(path).map_err(|e| ToolError::Io(IoError::Io(e)))?;
    let ui2_again = solana_tools_lite::models::input_transaction::UiTransaction::from(&tx2);
    assert_eq!(content2, serde_json::to_string(&ui2_again).unwrap());

    println!("--- {:?}", ui2_again);

    std::fs::remove_file(path).map_err(|e| ToolError::Io(IoError::Io(e)))?;
    Ok(())
}
