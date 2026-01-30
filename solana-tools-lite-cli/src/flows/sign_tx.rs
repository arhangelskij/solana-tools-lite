use serde::Serialize;
use serde_json;
use solana_tools_lite::adapters::io_adapter::{
    read_and_parse_secret_key, read_input_transaction, read_lookup_tables, write_signed_transaction,
};
use solana_tools_lite::handlers::analysis::{analyze_input_transaction, build_signing_summary};
use solana_tools_lite::handlers::sign_tx::handle as handle_sign_transaction;
use solana_tools_lite::models::analysis::{SigningSummary, TxAnalysis};
use solana_tools_lite::serde::fmt::OutputFormat;
use solana_tools_lite::models::{PubkeyBase58, Transaction};

use crate::flows::presenter::{Presentable, AnalysisPresenter};
use crate::models::cmds::OutFmt;
use crate::shell::error::CliError;

/// Sign-transaction flow: thin orchestrator around the handler.
///
/// Parameters
/// - `input`: optional path to input file (when `None`, handler may read from stdin)
/// - `keypair_path`: path to keypair file (stdin disabled for secrets in adapter)
/// - `output`: optional output path (stdout when `None` or `Some("-")` via adapter)
/// - `pretty_json`: pretty JSON when output format is JSON
/// - `out_override`: force output format (json|base64|base58); otherwise mirrors input format
/// - `summary_json`: emit a machine-readable summary to stdout (requires `output` for the signed tx)
pub fn execute(
    input: Option<&str>,
    keypair_path: &str,
    output: Option<&str>,
    pretty_json: bool,
    out_override: Option<OutFmt>,
    force: bool,
    lookup_tables_path: Option<&str>,
    assume_yes: bool,
    max_fee: Option<u64>,
    summary_json: bool,
) -> Result<(), CliError> {
    if summary_json && output.map(|o| o == "-").unwrap_or(true) {
        return Err(CliError::SummaryRequiresOutput);
    }
    // 1) Read input transaction (file/stdin) via adapter
    let input_tx = read_input_transaction(input)?;

    // 2) Resolve default output format from input type (mirrors input format)
    let default_format = input_tx.default_output_format(pretty_json);

    // 3) Read + parse signing key
    let signing_key = read_and_parse_secret_key(keypair_path)?;

    let signing_pubkey = PubkeyBase58::from(signing_key.verifying_key().to_bytes());

    // 4) Optional: expand v0 accounts with lookup tables
    let tables = lookup_tables_path.map(read_lookup_tables).transpose()?;

    // 5) Analyze unsigned transaction via analyze_input_transaction
    let analysis = analyze_input_transaction(&input_tx, &signing_pubkey, tables.as_ref())?;
    let analysis_presenter = AnalysisPresenter {
        analysis: Some(&analysis),
        summary_payload: None,
    };
    
    analysis_presenter.present(false, false, true)?;

    // 6) Enforce fee limit for CI/pipeline safety
    if let Some(limit) = max_fee {
        if analysis.total_fee_lamports > limit as u128 {
            return Err(CliError::FeeLimitExceeded {
                fee_lamports: analysis.total_fee_lamports,
                max_lamports: limit,
            });
        }
    }

    // 7) Interactive confirm unless --yes
    if !assume_yes && !confirm_stdin()? {
        return Err(CliError::UserRejected);
    }

    // 8) Sign the tx
    let result = handle_sign_transaction(input_tx, &signing_key)?;

    // 9) Choose output format (override or mirror input)
    let chosen_format = match out_override {
        Some(OutFmt::Json) => OutputFormat::Json {
            pretty: pretty_json,
        },
        Some(OutFmt::Base64) => OutputFormat::Base64,
        Some(OutFmt::Base58) => OutputFormat::Base58,
        None => default_format,
    };

    // Optional JSON summary (prepared before writing the tx)
    let summary_payload =
        prepare_summary_payload(summary_json, &result.signed_tx, &analysis, output)?;

    // 10) Write out via adapter (file or stdout), respecting force for files
    write_signed_transaction(&result.signed_tx, chosen_format, output, force)?;

    if let Some(payload) = summary_payload.as_deref() {
        let summary_presenter = AnalysisPresenter {
            analysis: None,
            summary_payload: Some(payload),
        };

        summary_presenter.present(true, false, false)?;
    }

    Ok(())
}

#[derive(Serialize)]
struct CliSigningSummary<'a> {
    #[serde(flatten)]
    core_summary: &'a SigningSummary,
    output_path: Option<&'a str>,
}

fn prepare_summary_payload(
    summary_json: bool,
    tx: &Transaction,
    analysis: &TxAnalysis,
    output: Option<&str>,
) -> Result<Option<String>, CliError> {
    if !summary_json {
        return Ok(None);
    }

    let summary = build_signing_summary(tx, analysis)?;
    let wrapper = CliSigningSummary {
        core_summary: &summary,
        output_path: output,
    };

    let payload = serde_json::to_string_pretty(&wrapper)
        .map_err(|e| CliError::SummaryEncode(e.to_string()))?;
    Ok(Some(payload))
}

fn confirm_stdin() -> Result<bool, CliError> {
    use std::io::{self, Write};
    
    eprint!("Sign this transaction? [y/N] ");

    // ensure prompt is visible before reading input
    io::stderr().flush().ok();
    
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .map_err(|e| CliError::StdinRead(e.to_string()))?;
    let trimmed = line.trim().to_ascii_lowercase();

    Ok(trimmed == "y" || trimmed == "yes")
}
