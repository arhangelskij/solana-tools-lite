use crate::adapters::io_adapter::{read_and_parse_secret_key, read_input_transaction, write_signed_transaction};
use crate::errors::ToolError;
use crate::handlers::sign_tx::handle_sign_transaction;
use crate::models::cmds::OutFmt;
use crate::models::input_transaction::InputTransaction;
use crate::serde::fmt::OutputFormat;

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
    keypair: &str, //TODO: mb add `path` to name?
    output: Option<&str>,
    json_pretty: bool,
    out_override: Option<OutFmt>,
    force: bool
) -> Result<(), ToolError> {
    // 1) Read input transaction (file/stdin) via adapter
    let input_tx: InputTransaction = read_input_transaction(input)?;

    //TODO: think about to move inside intput_tx this logic
    // 2) Resolve default output format from input type
    let default_format = match &input_tx {
        InputTransaction::Json(_) => OutputFormat::Json { pretty: json_pretty },
        InputTransaction::Base64(_) => OutputFormat::Base64,
        InputTransaction::Base58(_) => OutputFormat::Base58,
    };

    // 3) Read + parse signing key
    let signing_key = read_and_parse_secret_key(keypair)?;

    // 4) Domain signing via pure handler (returns raw tx in result)
    let result = handle_sign_transaction(input_tx, &signing_key)?;

    // 5) Choose output format (override or mirror input)
    let chosen_format = match out_override {
        Some(OutFmt::Json) => OutputFormat::Json { pretty: json_pretty },
        Some(OutFmt::Base64) => OutputFormat::Base64,
        Some(OutFmt::Base58) => OutputFormat::Base58,
        None => default_format,
    };
//TODO: 6/09 write 🔴🔴
    // 6) Write out via adapter (file or stdout), respecting force for files
    write_signed_transaction(&result.signed_tx, chosen_format, output, force)?;

    Ok(())
}
