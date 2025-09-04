use crate::adapters::io_adapter as io;
use crate::adapters::io_adapter::{read_message, read_pubkey, read_signature};
use crate::errors::ToolError;
use crate::flows::presenter::Presentable;
use crate::handlers::verify;
use crate::models::results::VerifyResult;
use std::path::Path;

/// Verify flow: calls domain handler and prints result.
/// Returns Ok(()) on valid signature; returns an error to trigger non-zero exit on invalid.
pub fn execute(
    message: Option<&str>,
    message_file: Option<&str>,
    signature: Option<&str>,
    signature_file: Option<&str>,
    pubkey: Option<&str>,
    pubkey_file: Option<&str>,
    output: Option<&str>,
    force: bool,
    json: bool,
) -> Result<(), ToolError> {
    // Resolve inputs using IO helpers
    let msg = read_message(message, message_file)?;
    let sig = read_signature(signature, signature_file)?;
    let pk = read_pubkey(pubkey, pubkey_file)?;

    let result = verify::handle(&msg, &sig, &pk)?;

    // Persist full JSON artifact to file only if requested
    let saved_path = io::save_pretty_json(&result, output, force, "verification.json")?;

    // Print result: when saving, keep stdout clean and print status + Saved to stderr
    print_result(&result, json, saved_path.as_deref());

    //TODO: 🔴 for Verify return Ok(VerifyResult)?
    if result.valid {
        Ok(())
    } else {
        Err(ToolError::InvalidInput(
            result
                .error
                .unwrap_or("signature verification failed".to_owned()),
        ))
    }
}

fn print_result(result: &VerifyResult, json: bool, saved_path: Option<&Path>) {
    match saved_path {
        Some(path) => {
            if result.valid {
                eprintln!("[✓] Signature is valid");
            } else {
                let err = result
                    .error
                    .as_deref()
                    .unwrap_or("signature verification failed");
                eprintln!("[✗] Signature is invalid: {}", err);
            }
            eprintln!("Saved: {}", path.display());
        }
        None => {
            // Delegate printing to Presentable for stdout/stderr behavior
            result.present(json, false);
        }
    }
}
