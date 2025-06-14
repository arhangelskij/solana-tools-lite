use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use anyhow::{Result, bail};

/// Create key (first 32 bytes as seed)
pub fn keypair_from_seed(seed: &[u8]) -> Result<SigningKey> {
    if seed.len() < 32 {
        bail!("Seed must be at least 32 bytes");
    }
    Ok(SigningKey::from_bytes(&seed[..32].try_into()?))
}

pub fn sign_message(key: &SigningKey, message: &[u8]) -> Signature {
    key.sign(message)
}

pub fn verify_signature(pubkey: &VerifyingKey, message: &[u8], signature: &Signature) -> bool {
    pubkey.verify(message, signature).is_ok()
}