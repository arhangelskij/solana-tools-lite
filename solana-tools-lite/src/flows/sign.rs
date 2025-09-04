use crate::adapters::io_adapter as io;
use crate::errors::{SignError, ToolError};
use crate::flows::presenter::Presentable;
use crate::handlers::sign_message;
use crate::models::results::SignResult;
use std::path::{Path, PathBuf};

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
    let saved_path = save_to_file(&result, output, force)?;

    // Print result similarly to generation flow, delegating to Presentable
    print_result(&result, json, saved_path.as_deref());

    Ok(())
}

// TODO: 🔴 think about trait `Saveable` or similar to group generic saving of results if its simple
fn save_to_file(
    result: &SignResult,
    out_path: Option<&str>,
    force: bool,
) -> Result<Option<PathBuf>, ToolError> {
    // Prepare pretty JSON
    let json_str =
        serde_json::to_string_pretty(&result).map_err(|e| SignError::JsonSerialize(e))?;

    // Write only when an explicit output path is provided
    let saved_path = match out_path {
        Some(path_str) => {
            let target = get_final_path(path_str);
            io::write_public_file(&target, &json_str, force)?;
            Some(target)
        }
        None => None,
    };

    Ok(saved_path)
}

//TODO: 28 aug 🟡 move into utils or something else

/// Resolve the final wallet path:
/// - if `output_path_str` points to a directory, append `wallet.json`
/// - otherwise treat it as a file path
fn get_final_path(output_path_str: &str) -> PathBuf {
    let p = Path::new(output_path_str);
    if p.is_dir() {
        p.join("sign.json") //TODO: 🟡
    } else {
        p.to_path_buf()
    }
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
    Ok(())
}
