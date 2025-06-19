use crate::utils::hex_encode;
use crate::crypto::bip39;
use anyhow::Context;

use ed25519_dalek::{SigningKey};

pub fn handle_gen(mnemonic: Option<String>, passphrase: Option<String>, explain: bool) -> anyhow::Result<()> {
    let mnemonic = match mnemonic {
        Some(m) => {
            bip39::validate_mnemonic(&m).context("mnemonic phrase provided by user is invalid")?;
            m
        },
        None => bip39::generate_mnemonic().context("failed to generate mnemonic")?,
    };

    let passphrase = passphrase.unwrap_or_default();

    let seed = bip39::derive_seed(&mnemonic, &passphrase).context("failed to derive seed from mnemonic")?;
    let signing_key = SigningKey::from_bytes(&seed[..32].try_into().context("seed slice is not 32 bytes when deriving signing key")?);

    if explain { //TODO: use info
        println!("🔐 Mnemonic:\n{}\n", mnemonic);
        println!("📥 Passphrase: '{}'", passphrase);
        println!("🔄 PBKDF2-SHA512 Seed:\n{}\n", hex_encode(&seed));
    }

    println!("📤 Public Key (Base58): {:?}", signing_key);
    Ok(())
}