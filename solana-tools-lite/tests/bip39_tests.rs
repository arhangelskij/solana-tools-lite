use std::error::Error;

// Assume the bip39 API is exposed at crate root as `bip39`
use solana_tools_lite::crypto::bip39::{derive_seed, generate_mnemonic, validate_mnemonic};

#[test]
fn test_generate_and_validate_mnemonic() -> Result<(), Box<dyn Error>> {
    let m = generate_mnemonic()?;
    let word_count = m.split_whitespace().count();
    assert!(word_count == 12 || word_count == 24, "unexpected word count: {}", word_count);
    validate_mnemonic(&m)?; // should be valid
    Ok(())
}

#[test]
fn test_validate_mnemonic_invalid_checksum() {
    // Same words but invalid checksum (last word not adjusted)
    let invalid = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon";
    let res = validate_mnemonic(invalid);
    assert!(res.is_err(), "expected checksum error for invalid mnemonic");
}

#[test]
fn test_derive_seed_vector1_trezor() -> Result<(), Box<dyn Error>> {
    // BIP39 test vector 1 (english), passphrase "TREZOR"
    // https://github.com/trezor/python-mnemonic/blob/master/vectors.json
    let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
    let expected_hex = "c55257c360c07c72029aebc1b53c05ed0362ada38ead3e3e9efa3708e53495531f09a6987599d18264c1e1c92f2cf141630c7a3c4ab7c81b2f001698e7463b04";
    let expected = hex_to_arr64(expected_hex);

    validate_mnemonic(mnemonic)?; // should be valid according to vector
    let seed = derive_seed(mnemonic, "TREZOR")?;
    assert_eq!(seed, expected, "seed must match BIP39 test vector 1 (TREZOR)");
    Ok(())
}

fn hex_to_arr64(s: &str) -> [u8; 64] {
    let bytes = (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect::<Vec<u8>>();
    let mut arr = [0u8; 64];
    arr.copy_from_slice(&bytes);
    arr
}