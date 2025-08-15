use crate::crypto::ed25519;
use crate::errors::Result;
use crate::models::results::VerifyResult;

pub fn handle(message: &str, signature_b58: &str, pubkey_b58: &str) -> Result<VerifyResult> {
    let result = ed25519::verify_signature_raw(message, signature_b58, pubkey_b58);
    
    Ok(VerifyResult {
        message: message.to_string(),
        pubkey: pubkey_b58.to_string(),
        signature: signature_b58.to_string(),
        valid: result.is_ok(),
        error: result.err().map(|e| e.to_string()),
    })
}
