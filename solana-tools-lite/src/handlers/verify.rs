use crate::models::results::VerifyResult;
use crate::utils::pretty_print_json;
use crate::crypto::ed25519;

pub fn handle_verify(message: &str, signature_b58: &str, pubkey_b58: &str, json: bool) -> i32 {
    let result = match ed25519::verify_signature_raw(message, signature_b58, pubkey_b58) {
        Ok(()) => VerifyResult {
            message: message.to_string(),
            pubkey: pubkey_b58.to_string(),
            signature: signature_b58.to_string(),
            valid: true,
            error: None
        },
        Err(e) => VerifyResult {
            message: message.to_string(),
            pubkey: pubkey_b58.to_string(),
            signature: signature_b58.to_string(),
            valid: false,
            error: Some(e.to_string())
        },
    };

    if json {
        pretty_print_json(&result);
    } else if result.valid {
        println!("[✓] Signature is valid");
    } else {
        eprintln!(
            "[✗] Signature is invalid: {}",
            result.error.as_deref().unwrap_or("unknown error")
        );
    }

    if result.valid { 0 } else { 1 }
}