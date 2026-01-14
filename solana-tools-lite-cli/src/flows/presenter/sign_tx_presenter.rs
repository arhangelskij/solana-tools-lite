//! Presentation rules for transaction signing summaries.

use crate::flows::presenter::{emit_line, Presentable};
use crate::shell::error::CliError;
use crate::constants::LAMPORTS_PER_SOL;
use solana_tools_lite::constants::compute_budget;
use solana_tools_lite::models::analysis::{AnalysisWarning, TokenProgramKind, TxAnalysis};
use solana_tools_lite::models::extensions::{AnalysisExtensionAction, LightProtocolAction};

/// Bundles analysis and an optional JSON summary payload.
pub(crate) struct SignTxPresentation<'a> {
    pub(crate) analysis: Option<&'a TxAnalysis>,
    pub(crate) summary_payload: Option<&'a str>,
}

impl Presentable for SignTxPresentation<'_> {
    fn present(
        &self,
        json: bool,
        _show_secret: bool,
        to_stderr: bool,
    ) -> Result<(), CliError> {
        if let Some(analysis) = self.analysis {
            emit_summary(analysis);
        }
        
        if json {
            if let Some(payload) = self.summary_payload {
                emit_line(payload, to_stderr);
            }
        }
        Ok(())
    }
}

fn lamports_to_sol(lamports: u128) -> f64 {
    lamports as f64 / LAMPORTS_PER_SOL
}

fn emit_summary(analysis: &TxAnalysis) {
    for (i, t) in analysis.transfers.iter().enumerate() {
        eprintln!("Instruction #{}: System Program (Transfer)", i + 1);
        eprintln!(
            "  From:   {}{}",
            t.from,
            if t.from_is_signer { " (signer)" } else { "" }
        );
        eprintln!("  To:     {}", t.to);
        eprintln!("  Amount: {:.9} SOL", lamports_to_sol(t.lamports as u128));
        eprintln!("          ({} lamports)", t.lamports);
    }

    eprintln!("--------------------------------------------------");
    eprintln!("TRANSACTION SUMMARY");
    eprintln!(
        "Network Fee:    {:.9} SOL ({} lamports)",
        lamports_to_sol(analysis.base_fee_lamports),
        analysis.base_fee_lamports
    );
    
    if analysis.is_fee_payer {
        eprintln!("                !!! YOU ARE THE FEE PAYER !!!");
    }
    
    if let Some((pf, est)) = analysis.priority_fee_lamports {
        if est {
            eprintln!(
                "Priority Fee:   {:.9} SOL ({} lamports, estimated with default {} CU)",
                lamports_to_sol(pf),
                pf,
                compute_budget::DEFAULT_COMPUTE_UNIT_LIMIT
            );
        } else {
            eprintln!(
                "Priority Fee:   {:.9} SOL ({} lamports)",
                lamports_to_sol(pf),
                pf
            );
        }
    } else {
        eprintln!("Priority Fee:   0.000000000 SOL (0 lamports)");
    }
    
    if let Some(price) = analysis.compute_unit_price_micro {
        let limit = analysis
            .compute_unit_limit
            .unwrap_or(compute_budget::DEFAULT_COMPUTE_UNIT_LIMIT);
        eprintln!(
            "Compute Budget: price={} micro-lamports, limit={}",
            price, limit
        );
    }
    let total_cost = analysis.total_fee_lamports + analysis.total_sol_send_by_signer;
    
    if analysis.total_sol_send_by_signer > 0 {
        eprintln!(
            "YOU SEND:       {:.9} SOL ({} lamports)",
            lamports_to_sol(analysis.total_sol_send_by_signer),
            analysis.total_sol_send_by_signer
        );
    }
    
    if analysis.has_non_sol_assets {
        eprintln!("ASSETS:         SOL + Non-SOL Tokens involved");
    }
    
    eprintln!("MAX TOTAL COST: {:.9} SOL", lamports_to_sol(total_cost));
    
    let (label, desc) = analysis.privacy_level.display_info(
        analysis.confidential_ops_count,
        analysis.storage_ops_count
    );
    eprintln!("PRIVACY LEVEL:  {} ({})", label, desc);
    eprintln!("--------------------------------------------------");

    if analysis.confidential_ops_count > 0 || analysis.storage_ops_count > 0 {
        eprintln!("EXTENSION PROTOCOLS SUMMARY:");
        if analysis.confidential_ops_count > 0 {
            eprintln!("  - Private (Confidential) Operations: {}", analysis.confidential_ops_count);
        }
        if analysis.storage_ops_count > 0 {
            eprintln!("  - Storage/Bridge (Public->ZK) Operations: {}", analysis.storage_ops_count);
        }
        eprintln!("--------------------------------------------------");
    }

    // ZK Compression / Extension Notices
    if !analysis.extension_actions.is_empty() {
        emit_extension_notices(&analysis.extension_actions);
        eprintln!("--------------------------------------------------");
    }

    if !analysis.warnings.is_empty() {
        eprintln!("WARNINGS:");
        for w in &analysis.warnings {
            eprintln!("- {}", warning_to_message(w));
        }
        eprintln!("--------------------------------------------------");
    }
}

fn warning_to_message(warning: &AnalysisWarning) -> String {
    match warning {
        AnalysisWarning::LookupTablesNotProvided => {
            "Address table lookups present but --tables was not provided; looked-up accounts will be shown as raw indexes".to_string()
        }
        AnalysisWarning::LookupTableMissing(key) => {
            format!("Lookup table {} missing or incomplete; some accounts may be unresolved", key)
        }
        AnalysisWarning::TokenTransferDetected(kind) => {
            let label = match kind {
                TokenProgramKind::SplToken => "Token Program",
                TokenProgramKind::Token2022 => "Token-2022 Program",
            };
            format!(
                "{} transfer detected; amounts are shown as raw u64 (offline mode cannot infer decimals)",
                label
            )
        }
        AnalysisWarning::UnknownProgram { program_id } => {
            format!("Unknown program encountered: {}", program_id)
        }
        AnalysisWarning::SignerNotRequired => {
            "!!! SECURITY WARNING !!! Your signature is NOT REQUIRED for this transaction. This might be a phishing attempt if you were asked to sign it.".to_string()
        }
        AnalysisWarning::CpiLimit => {
            "Analysis limited to top-level instructions. CPI (Cross-Program Invocations) not analyzed.".to_string()
        }
        AnalysisWarning::ConfidentialTransferDetected => {
            "Confidential Transfer (Token-2022) detected. Transaction privacy level set to Hybrid/Confidential.".to_string()
        }
    }
}
fn emit_extension_notices(actions: &[AnalysisExtensionAction]) {
    let mut saw_light_protocol = false;
    
    eprintln!("EXTENSION PROTOCOLS DETECTED:");
    for action in actions {
        match action {
            AnalysisExtensionAction::LightProtocol(light_action) => {
                saw_light_protocol = true;
                eprintln!("- Light Protocol: {}", light_action.description());
            }
        }
    }

    if saw_light_protocol {
        eprintln!();
        eprintln!("!!! ZK COMPRESSION NOTICE !!!");
        eprintln!("This transaction uses ZK Compression (Light Protocol).");
        eprintln!("- Compressed assets are NOT visible in standard explorers (SolanaFM, Solscan, etc.)");
        eprintln!("- You need a specialized indexer or explorer (e.g. Photon) to view state.");
        eprintln!("- Valid proofs are required for these instructions.");
        eprintln!("Please verify the integrity of the proof data source.");
    }
}
