use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use anyhow::{Result, bail};
use crate::handlers::verify::VerifyError;

const SIG_LEN: usize = 64;
const PUBKEY_LEN: usize = 32;

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
// TODO: check unused funcs
pub fn verify_signature(pubkey: &VerifyingKey, message: &[u8], signature: &Signature) -> bool {
    pubkey.verify(message, signature).is_ok()
}

pub fn verify_signature_raw(message: &str, signature_b58: &str, pubkey_b58: &str,) -> Result<(), VerifyError> {
    let sig_bytes = bs58::decode(signature_b58).into_vec()?;
    let pubkey_bytes = bs58::decode(pubkey_b58).into_vec()?;

    if sig_bytes.len() != SIG_LEN {
        return Err(VerifyError::InvalidSignatureLength(sig_bytes.len()));
    }
    if pubkey_bytes.len() != PUBKEY_LEN {
        return Err(VerifyError::InvalidPubkeyLength(pubkey_bytes.len()));
    }

    let sig_array: [u8; SIG_LEN] = sig_bytes
        .try_into()
        .map_err(|_| VerifyError::InvalidSignatureFormat)?;
    let pubkey_array: [u8; PUBKEY_LEN] = pubkey_bytes
        .try_into()
        .map_err(|_| VerifyError::InvalidPubkeyFormat)?;

    let signature = ed25519_dalek::Signature::from_bytes(&sig_array);
    let pubkey = ed25519_dalek::VerifyingKey::from_bytes(&pubkey_array)
        .map_err(|_| VerifyError::InvalidPubkeyFormat)?;

    pubkey
        .verify(message.as_bytes(), &signature)
        .map_err(|_| VerifyError::VerificationFailed)?;

    Ok(())
}