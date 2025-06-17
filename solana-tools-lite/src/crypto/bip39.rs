use hmac::Hmac;
use pbkdf2::pbkdf2;
use sha2::Sha512;
use bip39::{Mnemonic, Language};

/// Generate a random 12-word English BIP-39 mnemonic phrase.
pub fn generate_mnemonic() -> String {
    let mut rng = bip39::rand::thread_rng();
    let mnemonic = Mnemonic::generate_in_with(&mut rng, Language::English, 12).unwrap();
    
    mnemonic.to_string()
}

/// Derive a 64-byte seed from a BIP-39 mnemonic and passphrase.
pub fn derive_seed(mnemonic: &str, passphrase: &str) -> [u8; 64] {
    let salt = format!("mnemonic{}", passphrase);
    let mut out = [0u8; 64];
    
    pbkdf2::<Hmac<Sha512>>(mnemonic.as_bytes(), salt.as_bytes(), 2048, &mut out)
        .expect("PBKDF2 failed");

    out
}
