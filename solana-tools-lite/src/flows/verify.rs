use crate::errors::ToolError;
use crate::utils::pretty_print_json;
use crate::handlers::verify; // expects handlers::verify_message::execute(...)

/// Verify flow: calls domain handler and prints result.
/// Returns Ok(()) on valid signature; returns an error to trigger non-zero exit on invalid.
pub fn execute(
    message: &str,
    signature_b58: &str,
    pubkey_b58: &str,
    json: bool,
) -> Result<(), ToolError> {
    let result = verify::handle(message, signature_b58, pubkey_b58)?;

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