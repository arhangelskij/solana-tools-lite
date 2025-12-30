use crate::flows::presenter::Presentable;
use solana_tools_lite::adapters::io_adapter as io;
use solana_tools_lite::handlers::sign_message;
use solana_tools_lite::models::results::SignResult;
use std::path::Path;
use crate::shell::error::CliError;

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
) -> Result<(), CliError> {
    // Resolve message from inline or file/stdin via adapter helper
    let message_content = io::read_message(message, message_file)?;

    // Read & parse signing key in the flow
    let signing_key = io::read_and_parse_secret_key(secret_key_path)?;

    // Sign message
    let result = sign_message::handle(&message_content, &signing_key)?;

    // Persist full JSON artifact to file only if requested (independent of `json`)
    let saved_path = io::save_pretty_json(&result, output, force, "sign.json")?;

    // Print result similarly to generation flow, delegating to Presentable
    print_result(&result, json, saved_path.as_deref())?;

    Ok(())
}

/// Print output of a signing flow
fn print_result(
    result: &SignResult,
    json: bool,
    saved_path: Option<&Path>,
) -> Result<(), CliError> {
    match saved_path {
        // When saving to a file, keep stdout clean; show signature and saved path on stderr
        Some(path) => {
            eprintln!("{}", result.signature_base58);
            eprintln!("Saved: {}", path.display());
        }
        None => {
            // Delegate to Presentable for stdout formatting
            result.present(json, false, false)?;
        }
    }
    Ok(())
}
