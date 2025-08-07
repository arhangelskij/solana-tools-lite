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
) -> Result<(), ToolError> {
    save_to_file(result, out_path)?;
    print_result(result, json, show_secret)?;

    Ok(())
}

fn print_result(result: &GenResult, json: bool, show_secret: bool) -> Result<(), ToolError> {
    if !show_secret {
        let public_gen_result = result.as_public();
        if json {
            // Pretty JSON only with public fields
            pretty_print_json(&public_gen_result);
        } else {
            println!("{}", public_gen_result);
        }
    } else {
        if json {
            // Pretty JSON with secrets
            pretty_print_json(&result);
        } else {
            println!("{}", result);
        }
    }

    Ok(())
}

fn save_to_file(result: &GenResult, out_path: Option<&str>) -> Result<(), ToolError> {
    // Default file name when user didn't provide one
    let wallet_path = out_path.unwrap_or("wallet.json");
    // Always save full wallet to file
    write_output(Some(wallet_path), &result.to_full_json())?;

    Ok(())
}
