use crate::errors::ToolError;
use crate::handlers::sign_message;
use crate::utils::pretty_print_json;
use crate::layers::io;

/// Execute the sign flow:
/// - `message`: optional message to sign (as provided by CLI layer)
/// - `message_file`: optional path to message file (stdin "-" handled in IO layer)
/// - `secret_key_path`: path to secret key file (stdin "-" is rejected in IO layer)
/// - `json`: if true, pretty-print JSON result; otherwise print only the Base58 signature
pub fn execute(
    message: Option<&str>,
    message_file: Option<&str>,
    secret_key_path: &str,
    json: bool,
) -> Result<(), ToolError> {
    let message_content = match (message, message_file) {
        (Some(msg), _) => msg.to_string(),
        (None, Some(file_path)) => io::read_input(Some(file_path))?,
        (None, None) => return Err(ToolError::InvalidInput("No message or message file provided".into()))
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

    Ok(())
}

//TODO: 13aug 🔴 add feature of saving to file full json regardless of `json` bool