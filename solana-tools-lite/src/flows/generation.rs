use crate::adapters::io_adapter::{write_secret_file};
use crate::adapters::io_adapter as io;
use crate::constants::DEFAULT_WALLET_FILENAME;
use crate::errors::ToolError;
use crate::flows::presenter::Presentable;
use crate::handlers::generate;
use crate::models::results::GenResult;
use std::path::{Path, PathBuf};

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
    force: bool,
) -> Result<(), ToolError> {
    // Delegate passphrase resolution to the handler layer for consistency
    let result = generate::handle(mnemonic_path, passphrase_path)?;
    let saved_path = save_to_file(&result, out_path, force)?;

    print_result(&result, json, show_secret, &saved_path);

    Ok(())
}

fn save_to_file(
    result: &GenResult,
    out_path: Option<&str>,
    force: bool,
) -> Result<PathBuf, ToolError> {
    // Resolve final target path (directory -> append wallet.json; None -> wallet.json)
    let target = io::get_final_path_with_default(out_path, DEFAULT_WALLET_FILENAME);

    // Always save full wallet to file
    io::write_secret_file(&target, &result.to_full_json(), force)?;

    Ok(target)
}

/// Print output of a generation
fn print_result(result: &GenResult, json: bool, show_secret: bool, saved_path: &Path) {
    // Delegate printing to Presentable; secrets and JSON handling stay the same
    result.present(json, show_secret);
    // Always inform where the wallet was saved
    eprintln!("Saved: {}", saved_path.display());
}