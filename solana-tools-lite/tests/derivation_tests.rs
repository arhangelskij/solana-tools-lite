use solana_tools_lite::crypto::derive::{derive_key_from_seed, DerivationPath, SOLANA_DERIVATION_PATH};
use solana_tools_lite::crypto::mnemonic::{derive_seed_from_mnemonic, parse_mnemonic};

#[test]
fn parse_valid_solana_path() {
    let path = DerivationPath::parse(SOLANA_DERIVATION_PATH).expect("valid solana path");
    assert_eq!(path.indexes.len(), 4);
}

#[test]
fn reject_non_hardened() {
    let res = DerivationPath::parse("m/44/501/0/0");
    assert!(res.is_err(), "non-hardened segments must fail");
}

#[test]
fn reject_bad_prefix() {
    let res = DerivationPath::parse("n/44'/501'");
    assert!(res.is_err(), "path must start with m/");
}

#[test]
fn derivation_is_deterministic() {
    let mnemonic =
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
    let normalized = parse_mnemonic(mnemonic).expect("parse mnemonic");
    let seed = derive_seed_from_mnemonic(&normalized, "TREZOR");
    let path = DerivationPath::parse(SOLANA_DERIVATION_PATH).expect("parse path");

    let (key_a, chain_a) = derive_key_from_seed(&seed, &path).expect("derive a");
    let (key_b, chain_b) = derive_key_from_seed(&seed, &path).expect("derive b");

    assert_eq!(key_a, key_b);
    assert_eq!(chain_a, chain_b);
}

#[test]
fn derivation_path_display_roundtrip() {
    let path = DerivationPath::parse("m/44'/501'/0'/0'").expect("parse path");
    assert_eq!(path.to_string(), "m/44'/501'/0'/0'");
}
