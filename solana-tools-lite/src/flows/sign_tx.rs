use crate::errors::ToolError;
use crate::handlers;
use crate::models::cmds::OutFmt;

/// Sign-transaction flow: thin orchestrator around the handler.
///
/// Parameters
/// - `input`: optional path to input file (when `None`, handler may read from stdin)
/// - `keypair`: path to keypair file (stdin disabled for secrets in adapter)
/// - `output`: optional output path (stdout when `None` or `Some("-")` via adapter)
/// - `json_pretty`: pretty JSON when output format is JSON
/// - `out_override`: force output format (json|base64|base58); otherwise mirrors input format
pub fn execute(
    input: Option<&str>,
    keypair: &str,
    output: Option<&str>,
    json_pretty: bool,
    out_override: Option<OutFmt>,
) -> Result<(), ToolError> {
    handlers::sign_tx::handle_sign_transaction_file(
        input,
        keypair,
        output,
        json_pretty,
        out_override,
    )?;

    Ok(())
}

