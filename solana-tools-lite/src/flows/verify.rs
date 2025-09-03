use crate::errors::ToolError;
use crate::handlers::verify;
use crate::adapters::io_adapter::{read_message, read_pubkey, read_signature};
use crate::utils::pretty_print_json;

/// Verify flow: calls domain handler and prints result.
/// Returns Ok(()) on valid signature; returns an error to trigger non-zero exit on invalid.
pub fn execute(
    message: Option<&str>,
    message_file: Option<&str>,
    signature: Option<&str>,
    signature_file: Option<&str>,
    pubkey: Option<&str>,
    pubkey_file: Option<&str>,
    json: bool //TODO: 26aug 🔴 add new fields
) -> Result<(), ToolError> {
    // Resolve inputs using IO helpers
    let msg = read_message(message, message_file)?;
    let sig = read_signature(signature, signature_file)?;
    let pk = read_pubkey(pubkey, pubkey_file)?;

    let result = crate::handlers::verify::handle(&msg, &sig, &pk)?;

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

    if result.valid {
        Ok(())
    } else {
        Err(ToolError::InvalidInput(result.error.unwrap_or("signature verification failed".to_owned())))
    }
}
