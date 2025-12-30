use solana_tools_lite::handlers::sign_message;
use solana_tools_lite::crypto::signing::SigningKey;
use solana_tools_lite_cli::flows::{base58, generation, sign, verify};
use solana_tools_lite_cli::models::cmds::Base58Action;
use std::fs;
use tempfile::TempDir;

const TEST_MNEMONIC: &str =
    "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

fn write_secret_key_file(dir: &TempDir, seed_byte: u8) -> (SigningKey, String) {
    let key = SigningKey::from_bytes(&[seed_byte; 32]);
    let path = dir.path().join(format!("key_{seed_byte}.json"));
    let content = serde_json::to_string(&key.to_bytes()).unwrap();
    
    fs::write(&path, content).unwrap();
    (key, path.to_string_lossy().to_string())
}

fn write_text_file(dir: &TempDir, name: &str, contents: &str) -> String {
    let path = dir.path().join(name);
    
    fs::write(&path, contents).unwrap();
    path.to_string_lossy().to_string()
}

#[test]
fn generation_flow_from_mnemonic_and_passphrase_files() {
    let dir = TempDir::new().unwrap();
    let mnemonic_path = write_text_file(&dir, "mnemonic.txt", TEST_MNEMONIC);
    let passphrase_path = write_text_file(&dir, "passphrase.txt", "hidden-pass");

    generation::execute(
        Some(&mnemonic_path),
        Some(&passphrase_path),
        false,
        false,
        Some(dir.path().to_str().unwrap()),
        false,
    )
    .expect("generation flow should succeed");

    let wallet_path = dir.path().join("wallet.json");
    let saved = fs::read_to_string(&wallet_path).expect("wallet file must exist");
    assert!(
        saved.contains("mnemonic"),
        "wallet file should contain mnemonic field"
    );
}

#[test]
fn sign_flow_reads_message_file_and_saves_json() {
    let dir = TempDir::new().unwrap();
    let message_path = write_text_file(&dir, "message.txt", "sign-me");
    let (_, key_path) = write_secret_key_file(&dir, 7);
    let output_path = dir.path().join("sign.json");

    sign::execute(
        None,
        Some(&message_path),
        &key_path,
        Some(output_path.to_str().unwrap()),
        false,
        true,
    )
    .expect("sign flow should succeed");

    let saved = fs::read_to_string(&output_path).expect("sign flow should write output json");
    assert!(
        saved.contains("signature_base58"),
        "output json should include signature"
    );
}

#[test]
fn verify_flow_valid_signature_creates_report() {
    let dir = TempDir::new().unwrap();
    let (key, _key_path) = write_secret_key_file(&dir, 9);
    let message = "hello world";
    let sig = sign_message::handle(message, &key).unwrap();
    let sig_b58 = sig.signature_base58;
    let pubkey_b58 = sig.public_key;
    let output_path = dir.path().join("verify.json");

    verify::execute(
        Some(message),
        None,
        Some(&sig_b58),
        None,
        Some(&pubkey_b58),
        None,
        Some(output_path.to_str().unwrap()),
        false,
        true,
    )
    .expect("verify flow should succeed");

    let saved = fs::read_to_string(&output_path).expect("verify json should exist");
    assert!(
        saved.contains("\"valid\": true"),
        "verify output should mark signature as valid"
    );
}

#[test]
fn verify_flow_invalid_signature_errors() {
    let dir = TempDir::new().unwrap();
    let (key, _) = write_secret_key_file(&dir, 11);
    let valid = sign_message::handle("msg1", &key).unwrap();
    let err = verify::execute(
        Some("msg2"),
        None,
        Some(&valid.signature_base58),
        None,
        Some(&valid.public_key),
        None,
        None,
        false,
        false,
    )
    .expect_err("verification must fail for mismatched message");
    assert!(
        format!("{err}").contains("VerifyFailed"),
        "error should propagate verification failure"
    );
}

#[test]
fn base58_flow_encode_and_invalid_decode() {
    base58::execute(
        &Base58Action::Encode {
            input: "hello world".into(),
        },
        false,
    )
    .expect("encode flow should succeed");

    let err = base58::execute(
        &Base58Action::Decode {
            input: "0OIl+/=".into(), // invalid alphabet
        },
        false,
    )
    .expect_err("decode flow should fail on invalid Base58");
    
    let err_text = err.to_string();
    assert!(
        err_text.contains("invalid character"),
        "unexpected error text: {err_text}"
    );
}
