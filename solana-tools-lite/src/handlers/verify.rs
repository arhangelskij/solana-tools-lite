use crate::crypto::signing::verify_signature_raw;
use crate::errors::Result;
use crate::models::results::VerifyResult;

/// Verify a Base58 signature against a message and public key.
pub fn handle(message: &str, signature_b58: &str, pubkey_b58: &str) -> Result<VerifyResult> {
    verify_signature_raw(message, signature_b58, pubkey_b58)?;

    Ok(VerifyResult {
        message: message.to_string(),
        pubkey: pubkey_b58.to_string(),
        signature: signature_b58.to_string(),
        valid: true,
    })
}
