use crate::errors::ToolError;
use crate::handlers::sign_message;
use crate::utils::pretty_print_json;

/// Execute the sign flow:
/// - `message`: message to sign (as provided by CLI layer)
/// - `secret_key_path`: path to secret key file (stdin "-" is rejected in IO layer)
/// - `json`: if true, pretty-print JSON result; otherwise print only the Base58 signature
pub fn execute(message: &str, secret_key_path: &str, json: bool) -> Result<(), ToolError> {
    // Domain handler: reads key from file (via adapter), signs, returns SignResult
    let result = sign_message::execute(message, secret_key_path)?;

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