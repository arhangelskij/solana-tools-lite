use crate::adapters::io_adapter::read_mnemonic;
use crate::crypto::bip39;
use crate::errors::{GenError, Result};
use crate::models::results::GenResult;
use crate::utils::{hex_encode};
use ed25519_dalek::SigningKey;
use std::convert::TryInto;

pub fn execute(
    mnemonic_path: Option<&String>,
    passphrase: Option<&String>
) -> Result<GenResult> {
    // Resolve mnemonic: read from file/stdin if provided, otherwise generate a new one.
    let mnemonic = if let Some(p) = mnemonic_path.map(|s| s.as_str()) {
        let m = read_mnemonic(p)?; // file or "-" (stdin), with whitespace normalization
        bip39::validate_mnemonic(&m)?;
        m
    } else {
        bip39::generate_mnemonic()?
    };

    let default_passphrase = String::new();
    let passphrase = passphrase.unwrap_or(&default_passphrase);

    let seed = bip39::derive_seed(&mnemonic, &passphrase)?;
    let seed32: [u8; 32] = seed[..32]
        .try_into()
        .map_err(|_| GenError::InvalidSeedLength)?;
    let signing_key = SigningKey::from_bytes(&seed32);

    let pubkey_bytes = signing_key.verifying_key().to_bytes();
    let pubkey_base58 = bs58::encode(pubkey_bytes).into_string();

    let result = GenResult {
        mnemonic: mnemonic.clone(),
        public_key: pubkey_base58.clone(),
        secret_key: bs58::encode(signing_key.to_bytes()).into_string(),
        seed_hex: hex_encode(&seed),
    };

    Ok(result)
}
