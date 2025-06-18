use hmac::Hmac;
use pbkdf2::pbkdf2;
use sha2::Sha512;
use bip39::{Mnemonic, Language};
use thiserror::Error;

/// Errors that can arise when working with BIP‑39 helpers.
#[derive(Debug, Error)]
pub enum Bip39Error {
    #[error("failed to generate mnemonic: {0}")]
    Mnemonic(#[from] bip39::Error),
    #[error("PBKDF2 failed: {0}")]
    Pbkdf2(&'static str),
}

/// Generate a random 12-word English BIP-39 mnemonic phrase.
pub fn generate_mnemonic() -> Result<String, Bip39Error> {
    let mut rng = bip39::rand::thread_rng();
    let mnemonic = Mnemonic::generate_in_with(&mut rng, Language::English, 12)?;
    
    Ok(mnemonic.to_string())
}

/// Derive a 64-byte seed from a BIP-39 mnemonic and passphrase.
pub fn derive_seed(mnemonic: &str, passphrase: &str) -> Result<[u8; 64], Bip39Error> {
    let salt = format!("mnemonic{}", passphrase);
    let mut out = [0u8; 64];
    
    pbkdf2::<Hmac<Sha512>>(mnemonic.as_bytes(), salt.as_bytes(), 2048, &mut out)
        .map_err(|_| Bip39Error::Pbkdf2("iteration count is zero or output buffer is empty"))?;

    Ok(out)
}
