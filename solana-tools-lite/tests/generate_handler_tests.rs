use solana_tools_lite::crypto::mnemonic::{derive_seed_from_mnemonic, parse_mnemonic};
use solana_tools_lite::handlers::generate;
use solana_tools_lite::utils::hex_encode;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn temp_path(tag: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("solana_tools_lite_{tag}_{nanos}.txt"))
}

#[test]
fn generate_from_mnemonic_and_passphrase_file() {
    let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
    let passphrase = "TREZOR";

    let mnemonic_path = temp_path("mnemonic");
    let passphrase_path = temp_path("passphrase");
    fs::write(&mnemonic_path, format!("{mnemonic}\n")).expect("write mnemonic file");
    fs::write(&passphrase_path, format!("{passphrase}\n")).expect("write passphrase file");

    let result = generate::handle(
        Some(mnemonic_path.to_string_lossy().as_ref()),
        Some(passphrase_path.to_string_lossy().as_ref()),
    )
    .expect("handle");

    let normalized = parse_mnemonic(mnemonic).expect("parse mnemonic");
    let expected_seed = derive_seed_from_mnemonic(&normalized, passphrase);
    let expected_seed_hex = hex_encode(expected_seed.as_bytes());

    assert_eq!(result.mnemonic, mnemonic);
    assert_eq!(result.seed_hex, expected_seed_hex);
    assert!(!result.public_key.is_empty());
    assert!(!result.secret_key.is_empty());

    let _ = fs::remove_file(mnemonic_path);
    let _ = fs::remove_file(passphrase_path);
}
