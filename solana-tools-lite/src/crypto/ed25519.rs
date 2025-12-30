use crate::constants::crypto::{PUBKEY_LEN, SIG_LEN};
use crate::errors::{KeypairError, VerifyError};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

/// Create key (first 32 bytes as seed)
pub fn keypair_from_seed(seed: &[u8]) -> Result<SigningKey, KeypairError> {
    if seed.len() < 32 {
        return Err(KeypairError::SeedTooShort(seed.len()));
    }

    let slice: [u8; 32] = seed[..32]
        .try_into()
        .map_err(|_| KeypairError::SeedSlice("expected 32-byte slice"))?;

    Ok(SigningKey::from_bytes(&slice))
}

/// Deterministic Ed25519 signature helper (dalek's `sign` is infallible).
pub fn sign_message(key: &SigningKey, message: &[u8]) -> Signature {
    key.sign(message)
}
/// Verify a signature against raw message bytes using a verifying key.
pub fn verify_signature(pubkey: &VerifyingKey, message: &[u8], signature: &Signature) -> bool {
    pubkey.verify(message, signature).is_ok()
}

/// Verify a Base58 signature against a Base58 public key and message string.
pub fn verify_signature_raw(
    message: &str,
    signature_b58: &str,
    pubkey_b58: &str,
) -> Result<(), VerifyError> {
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

    let signature = signature_from_bytes(&sig_array);
    let pubkey = ed25519_dalek::VerifyingKey::from_bytes(&pubkey_array)
        .map_err(|_| VerifyError::InvalidPubkeyFormat)?;

    pubkey
        .verify(message.as_bytes(), &signature)
        .map_err(|_| VerifyError::VerificationFailed)?;

    Ok(())
}

/// Wrapper for creating a Signature from bytes (so that you don't use dalek directly)
pub fn signature_from_bytes(bytes: &[u8; 64]) -> Signature {
    Signature::from_bytes(bytes)
}
