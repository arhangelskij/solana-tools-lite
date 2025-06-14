use crate::utils::hex_encode;
use crate::crypto::bip39;

use ed25519_dalek::{SigningKey};

pub fn handle_gen(mnemonic: Option<String>, passphrase: Option<String>, explain: bool) -> anyhow::Result<()> {
    let mnemonic = mnemonic.unwrap_or_else(|| bip39::generate_mnemonic());
    let passphrase = passphrase.unwrap_or_default();

    let seed = bip39::derive_seed(&mnemonic, &passphrase);
    let signing_key = SigningKey::from_bytes(&seed[..32].try_into()?);

    

    if explain { //TODO: use info
        println!("🔐 Mnemonic:\n{}\n", mnemonic);
        println!("📥 Passphrase: '{}'", passphrase);
        println!("🔄 PBKDF2-SHA512 Seed:\n{}\n", hex_encode(&seed));
    }

    println!("📤 Public Key (Base58): {:?}", signing_key);
    Ok(())
}