use serde_json;
use solana_tools_lite::adapters::io_adapter::{
    read_input_transaction, read_lookup_tables,
};
use solana_tools_lite::handlers::analysis::{analyze_transaction, build_signing_summary};
use solana_tools_lite::models::{InputTransaction, PubkeyBase58, Transaction};
use std::convert::TryFrom;

use crate::flows::presenter::{Presentable, AnalysisPresenter};
use crate::shell::error::CliError;

/// Analyze-transaction flow: thin orchestrator around the analysis handler.
///
/// Parameters
/// - `input`: optional path to input file (when `None`, handler may read from stdin)
/// - `pubkey`: optional public key to analyze as (Base58); if not provided, uses first signer
/// - `lookup_tables_path`: optional path to lookup tables file
/// - `summary_json`: emit analysis summary as JSON to stdout
pub fn execute(
    input: Option<&str>,
    pubkey: Option<&str>,
    lookup_tables_path: Option<&str>,
    summary_json: bool,
) -> Result<(), CliError> {
    // 1) Read input transaction (file/stdin) via adapter
    let input_tx: InputTransaction = read_input_transaction(input)?;

    // 2) Convert to Transaction
    let tx: Transaction = Transaction::try_from(input_tx)
        .map_err(|e| CliError::Core(solana_tools_lite::ToolError::TransactionParse(e)))?;

    let message = &tx.message;

    // 3) Determine the public key to analyze as
    let analyze_pubkey = if let Some(pk_str) = pubkey {
        PubkeyBase58::try_from(pk_str)
            .map_err(|e| CliError::Core(solana_tools_lite::ToolError::InvalidInput(
                format!("Invalid pubkey: {}", e)
            )))?
    } else {
        // Use first signer from message header
        let account_keys = message.account_keys();

        account_keys
            .get(0)
            .cloned()
            .ok_or_else(|| CliError::Core(solana_tools_lite::ToolError::InvalidInput(
                "No accounts in message".to_string()
            )))?
    };

    // 4) Read lookup tables if provided
    let tables = lookup_tables_path.map(read_lookup_tables).transpose()?;

    // 5) Analyze the transaction
    let analysis = analyze_transaction(message, &analyze_pubkey, tables.as_ref());

    // 6) Present analysis summary to stderr
    let analysis_presenter = AnalysisPresenter {
        analysis: Some(&analysis),
        summary_payload: None,
    };

    analysis_presenter.present(false, false, true)?;

    // 7) Optionally emit JSON summary to stdout
    if summary_json {
        let summary = build_signing_summary(&tx, &analysis)?;

        let payload = serde_json::to_string_pretty(&summary)
            .map_err(|e| CliError::SummaryEncode(e.to_string()))?;

        println!("{}", payload);
    }

    Ok(())
}
