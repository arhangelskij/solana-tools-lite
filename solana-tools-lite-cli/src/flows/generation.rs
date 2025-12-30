use crate::constants::DEFAULT_WALLET_FILENAME;
use crate::flows::presenter::Presentable;
use solana_tools_lite::adapters::io_adapter as io;
use solana_tools_lite::handlers::generate;
use solana_tools_lite::models::results::GenResult;
use std::path::{Path, PathBuf};
use crate::shell::error::CliError;
use serde_json;

/// High-level generation flow: orchestrates handler call, file saving, and presentation.
///
/// Parameters
/// * `mnemonic_path`  – read mnemonic from file or stdin ("-"); when `None`, a new mnemonic is generated
/// * `passphrase_path` – read BIP‑39 passphrase from file or stdin ("-"); when `None`, uses empty passphrase
/// * `json`           – print result as JSON (`--json`)
/// * `show_secret`    – print private part to stdout (`--unsafe-show-secret`)
/// * `out_path`       – target path (file or directory) to save the full wallet JSON
/// * `force`          – override the wallet file if it exists
pub fn execute(
    mnemonic_path: Option<&str>,
    passphrase_path: Option<&str>,
    json: bool,
    show_secret: bool,
    out_path: Option<&str>,
    force: bool,
) -> Result<(), CliError> {
    let result = generate::handle(mnemonic_path, passphrase_path)?;
    let saved_path = save_to_file(&result, out_path, force)?;

    let print_stderr = out_path.is_some();
    print_result(&result, json, show_secret, print_stderr, &saved_path)?;

    Ok(())
}

fn save_to_file(
    result: &GenResult,
    out_path: Option<&str>,
    force: bool,
) -> Result<PathBuf, CliError> {
    // Resolve final target path (directory -> append wallet.json; None -> wallet.json)
    let target = io::resolve_final_path_with_default(out_path, DEFAULT_WALLET_FILENAME);

    // Always save full wallet to file
    let payload = serde_json::to_string_pretty(result)
        .map_err(|e| CliError::PresentationEncode(e.to_string()))?;
    io::write_secret_file(&target, &payload, force)?;

    Ok(target)
}

/// Print output of a generation
fn print_result(
    result: &GenResult,
    json: bool,
    show_secret: bool,
    print_stderr: bool,
    saved_path: &Path,
) -> Result<(), CliError> {
    // Delegate printing to Presentable
    result.present(json, show_secret, print_stderr)?;

    // Always inform where the wallet was saved
    eprintln!("Saved: {}", saved_path.display());
    Ok(())
}
