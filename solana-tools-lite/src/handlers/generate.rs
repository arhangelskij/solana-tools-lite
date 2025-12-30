use crate::adapters::io_adapter::{read_mnemonic, read_passphrase};
use crate::crypto::derive::{derive_key_from_seed, DerivationPath, SOLANA_DERIVATION_PATH};
use crate::crypto::mnemonic::{
    derive_seed_from_mnemonic, generate_mnemonic_with, parse_mnemonic, Bip39Config,
};
use crate::errors::Result;
use crate::models::results::GenResult;
use crate::utils::hex_encode;
use ed25519_dalek::SigningKey;

/// Generate or load a mnemonic, derive a keypair, and return a structured result.
pub fn handle(mnemonic_path: Option<&str>, passphrase_path: Option<&str>) -> Result<GenResult> {
    // Resolve mnemonic: read from file/stdin if provided, otherwise generate a new one.
    let mnemonic = if let Some(p) = mnemonic_path {
        let m = read_mnemonic(p)?; // file or "-" (stdin), with whitespace normalization
        parse_mnemonic(&m)?
    } else {
        generate_mnemonic_with(Bip39Config::default())?
    };

    // Resolve passphrase securely: read from file or stdin when provided; default to empty
    let passphrase_owned: Option<String> = match passphrase_path {
        Some(p) => Some(read_passphrase(p)?),
        None => None,
    };

    let passphrase: &str = passphrase_owned.as_deref().unwrap_or("");

    let seed = derive_seed_from_mnemonic(&mnemonic, passphrase);

    // Solana standard path: m/44'/501'/0'/0'
    let path = DerivationPath::parse(SOLANA_DERIVATION_PATH)?;
    let (key_bytes, _) = derive_key_from_seed(&seed, &path)?;

    let signing_key = SigningKey::from_bytes(&key_bytes);

    let pubkey_bytes = signing_key.verifying_key().to_bytes();
    let pubkey_base58 = bs58::encode(pubkey_bytes).into_string();

    let result = GenResult {
        mnemonic: mnemonic.phrase(),
        public_key: pubkey_base58.clone(),
        secret_key: bs58::encode(signing_key.to_bytes()).into_string(),
        seed_hex: hex_encode(seed.as_bytes()),
    };

    Ok(result)
}
