use crate::adapters::io_adapter as io;
use crate::errors::{SignError, ToolError};
use crate::handlers::sign_message;
use crate::models::results::SignResult;
use crate::utils::pretty_print_json;
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
    let message_content = match (message, message_file) {
        (Some(msg), _) => msg.to_string(),
        (None, Some(file_path)) => io::read_input(Some(file_path))?,
        (None, None) => {
            return Err(ToolError::InvalidInput(
                "No message or message file provided".into(),
            ));
        }
    };

    // Domain handler: reads key from file (via adapter), signs, returns SignResult
    let result = sign_message::handle(&message_content, secret_key_path)?;

    if json {
        // Pretty JSON with message, signature_base58, public_key
        pretty_print_json(&result);
    } else {
        // Plain output: only the Base58 signature to stdout
        println!("{}", result.signature_base58);
    }

    // Persist full JSON artifact to file if requested (independent of `json`)
    save_to_file(&result, output, force);
    // if let Some(path) = output {

    // }

    Ok(())
}

// TODO: 🔴 think about trait `Saveable` or similar to group generic saving of results if its simple
fn save_to_file(
    result: &SignResult,
    out_path: Option<&str>,
    force: bool,
) -> Result<PathBuf, ToolError> {
    // Always write the full JSON result; use pretty JSON for readability
    let json_str =
        serde_json::to_string_pretty(&result).map_err(|e| SignError::JsonSerialize(e))?;

    io::write_output(out_path, &json_str)?;
    eprintln!("Saved: {}", out_path.unwrap_or("result.json"));

    // Resolve final target path (directory -> append wallet.json; None -> wallet.json)
    let target: PathBuf = get_final_path(out_path.unwrap_or("result.json"));

    Ok(target)
}

//TODO: 28 aug 🟡 move into utils or something else

/// Resolve the final wallet path:
/// - if `output_path_str` points to a directory, append `wallet.json`
/// - otherwise treat it as a file path
fn get_final_path(output_path_str: &str) -> PathBuf {
    let p = Path::new(output_path_str);
    if p.is_dir() {
        p.join("wallet.json")
    } else {
        p.to_path_buf()
    }
}
