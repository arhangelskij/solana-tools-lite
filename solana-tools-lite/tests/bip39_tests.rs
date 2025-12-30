use std::error::Error;

// Assume the bip39 API is exposed at crate root as `bip39`
use solana_tools_lite::crypto::bip39::{
    Bip39Config, derive_seed_from_mnemonic, generate_mnemonic_with, parse_mnemonic, validate_mnemonic,
};
#[test]
fn test_generate_and_validate_mnemonic() -> Result<(), Box<dyn Error>> {
    let m = generate_mnemonic_with(Bip39Config::default())?;
    let phrase = m.phrase();
    let word_count = phrase.split_whitespace().count();
    assert!(
        word_count == 12 || word_count == 24,
        "unexpected word count: {}",
        word_count
    );
    validate_mnemonic(&phrase)?; // should be valid
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

    let normalized = parse_mnemonic(mnemonic)?; // validates and normalizes
    let seed = derive_seed_from_mnemonic(&normalized, "TREZOR");
    assert_eq!(
        seed.as_bytes(),
        &expected,
        "seed must match BIP39 test vector 1 (TREZOR)"
    );
    Ok(())
}

#[test]
fn test_derive_seed_24_words_passphrase() -> Result<(), Box<dyn Error>> {
    // 24-word mnemonic with passphrase from Trezor vectors
    let mnemonic = "fish seed gold brand approve senior level choose deer snow gun address denial pottery science unknown dry library unfair wing spin slender choice achieve";
    let passphrase = "TREZOR";
    let expected_hex = "a4bf39c476dbcf269b454b4693d8c183b81d376cf7562286bafbb0049654553707791f2d651850a847d9b6904ad30988f74ed59bf23d01b88cebe28e05459aa3";
    
    let normalized = parse_mnemonic(mnemonic)?;
    let seed = derive_seed_from_mnemonic(&normalized, passphrase);
    
    let expected = hex_to_arr64(expected_hex);
    assert_eq!(seed.as_bytes(), &expected, "seed must match BIP39 24-word vector");
    
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
