use std::path::Path;
use crate::errors::ToolError;
use crate::layers::io::write_output;
use crate::models::results::GenResult;
use crate::utils::pretty_print_json;

/// Present `GenResult` according to CLI flags and save full wallet.
///
/// * `result`       – data returned by the domain layer
/// * `json`         – pretty JSON requested (`--json-pretty`)
/// * `show_secret`  – print private part to stdout (`--show-secret`)
/// * `out_path`     – where to save the full wallet
pub fn execute(
    result: &GenResult,
    json: bool,
    show_secret: bool,
    out_path: Option<&str>,
    force: bool
) -> Result<(), ToolError> {
    save_to_file(result, out_path, force)?;
    print_result(result, json, show_secret)?;

    Ok(())
}

fn print_result(result: &GenResult, json: bool, show_secret: bool) -> Result<(), ToolError> {
    match (json, show_secret) {
        // Pretty JSON with secrets
        (true, true ) => pretty_print_json(result),
        // With secrets
        (false, true ) => println!("{}", result),
        // Public
        (true, false) => pretty_print_json(&result.as_public()),
        (false, false) => println!("{}", result.as_public())
        
    }
    Ok(())
}

fn save_to_file(result: &GenResult, out_path: Option<&str>, force: bool) -> Result<(), ToolError> {
    // Default file name when user didn't provide one
    let wallet_path = out_path.unwrap_or("wallet.json");

    // If file exists and not forced, return an error
    if Path::new(wallet_path).exists() && !force {
        return Err(ToolError::FileExists(wallet_path.to_string()));
    }

    // Always save full wallet to file
    write_output(Some(wallet_path), &result.to_full_json())?;

    Ok(())
}
