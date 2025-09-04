use crate::adapters::io_adapter as io;
use crate::errors::ToolError;
use crate::flows::presenter::Presentable;
use crate::handlers::sign_message;
use crate::models::results::SignResult;
use std::path::Path;

/// Execute the sign flow:
/// - `message`: optional message to sign (as provided by CLI layer)
/// - `message_file`: optional path to message file (stdin "-" handled in IO layer)
/// - `secret_key_path`: path to secret key file (stdin "-" is rejected in IO layer)
/// - `json`: if true, pretty-print JSON result; otherwise print only the Base58 signature
pub fn execute(
    message: Option<&str>,
    message_file: Option<&str>,
    secret_key_path: &str,
    output: Option<&str>,
    force: bool,
    json: bool,
) -> Result<(), ToolError> {
    // Resolve message from inline or file/stdin via adapter helper
    let message_content = io::read_message(message, message_file)?;

    // Domain handler: reads key from file (via adapter), signs, returns SignResult
    let result = sign_message::handle(&message_content, secret_key_path)?;

    // Persist full JSON artifact to file only if requested (independent of `json`)
    let saved_path = io::save_pretty_json(&result, output, force, "sign.json")?;

    // Print result similarly to generation flow, delegating to Presentable
    print_result(&result, json, saved_path.as_deref());

    Ok(())
}

/// Print output of a signing flow
fn print_result(result: &SignResult, json: bool, saved_path: Option<&Path>) {
    match saved_path {
        // When saving to a file, keep stdout clean; show signature and saved path on stderr
        Some(path) => {
            eprintln!("{}", result.signature_base58);
            eprintln!("Saved: {}", path.display());
        }
        None => {
            // Delegate to Presentable for stdout formatting
            result.present(json, false);
        }
    }
}
