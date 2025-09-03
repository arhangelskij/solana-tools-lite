use std::path::{Path, PathBuf};
use crate::errors::ToolError;
use crate::adapters::io_adapter::write_secret_file;
use crate::models::results::GenResult;
use crate::flows::presenter::Presentable;
use crate::constants::DEFAULT_WALLET_FILENAME;
use crate::handlers::generate;

/// High-level generation flow: orchestrates handler call, file saving, and presentation.
///
/// Parameters
/// * `mnemonic_path`  – read mnemonic from file or stdin ("-"); when `None`, a new mnemonic is generated
/// * `passphrase_path` – read BIP‑39 passphrase from file or stdin ("-"); when `None`, uses empty passphrase
/// * `json`           – pretty JSON requested (`--json-pretty`)
/// * `show_secret`    – print private part to stdout (`--show-secret`)
/// * `out_path`       – target path (file or directory) to save the full wallet JSON
/// * `force`          – override the wallet file if it exists
pub fn execute(
    mnemonic_path: Option<&str>,
    passphrase_path: Option<&str>,
    json: bool,
    show_secret: bool,
    out_path: Option<&str>,
    force: bool
) -> Result<(), ToolError> {
    let result = generate::handle(mnemonic_path, passphrase_path)?;
    let saved_path = save_to_file(&result, out_path, force)?;
    
    print_result(&result, json, show_secret, &saved_path)?;

    Ok(())
}

fn save_to_file(result: &GenResult, out_path: Option<&str>, force: bool) -> Result<PathBuf, ToolError> {
    // Resolve final target path (directory -> append wallet.json; None -> wallet.json)
    let target: PathBuf = get_final_path(out_path.unwrap_or("wallet.json"));

    // Always save full wallet to file
    write_secret_file(&target, &result.to_full_json(), force)?;

    Ok(target)
}


//TODO: 28 aug 🟡 move into utils or something else

/// Resolve the final wallet path:
/// - if `output_path_str` points to a directory, append `wallet.json`
/// - otherwise treat it as a file path
fn get_final_path(output_path_str: &str) -> PathBuf {
    let p = Path::new(output_path_str);
    
    if p.is_dir() {
        p.join(DEFAULT_WALLET_FILENAME)
    } else {
        p.to_path_buf()
    }
}

/// Print output of a generation
fn print_result(result: &GenResult, json: bool, show_secret: bool, saved_path: &Path) -> Result<(), ToolError> {
    // Delegate printing to Presentable; secrets and JSON handling stay the same
    result.present(json, show_secret);
    // Always inform where the wallet was saved
    eprintln!("Saved: {}", saved_path.display());

    Ok(())
}