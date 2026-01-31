//! Presentation rules for transaction signing summaries.

use crate::flows::presenter::{emit_line, Presentable};
use crate::shell::error::CliError;
use solana_tools_lite::constants::compute_budget;
use solana_tools_lite::models::analysis::{AnalysisWarning, TokenProgramKind, TxAnalysis};
use solana_tools_lite::utils::format_sol;

/// Bundles analysis and an optional JSON summary payload.
pub struct AnalysisPresenter<'a> {
    pub analysis: Option<&'a TxAnalysis>,
    pub summary_payload: Option<&'a str>,
}

impl Presentable for AnalysisPresenter<'_> {
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

fn emit_summary(analysis: &TxAnalysis) {
    for (i, t) in analysis.transfers.iter().enumerate() {
        eprintln!("==================================================");
        eprintln!("Instruction #{}: System Program (Transfer)", i + 1);
        eprintln!(
            "  From:   {}{}",
            t.from,
            if t.from_is_signer { " (signer)" } else { "" }
        );
        eprintln!("  To:     {}", t.to);
        eprintln!("  Amount: {} ({} lamports)", format_sol(t.lamports as u128), t.lamports);
        // eprintln!("          ({} lamports)", t.lamports);
    }

    eprintln!("--------------------------------------------------");
    eprintln!("TRANSACTION SUMMARY");
    eprintln!("Non-SOL Assets: {}", if analysis.has_non_sol_assets { "Yes (SPL/Token-2022 detected)" } else { "No" });
    eprintln!(
        "Network Fee:    {} ({} lamports)",
        format_sol(analysis.base_fee_lamports),
        analysis.base_fee_lamports
    );
    
    if analysis.is_fee_payer {
        eprintln!("                !!! YOU ARE THE FEE PAYER !!!");
    }
    
    if let Some((pf, est)) = analysis.priority_fee_lamports {
        if est {
            eprintln!(
                "Priority Fee:   {} ({} lamports, estimated with default {} CU)",
                format_sol(pf),
                pf,
                compute_budget::DEFAULT_COMPUTE_UNIT_LIMIT
            );
        } else {
            eprintln!(
                "Priority Fee:   {} ({} lamports)",
                format_sol(pf),
                pf
            );
        }
    } else {
        eprintln!("Priority Fee:   {} (0 lamports)", format_sol(0));
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
            "YOU SEND:       {} ({} lamports)",
            format_sol(analysis.total_sol_send_by_signer as u128),
            analysis.total_sol_send_by_signer
        );
    }
    
    eprintln!("MAX TOTAL COST: {}", format_sol(total_cost));
    
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

    // Extension Protocol Actions
    if !analysis.extension_actions.is_empty() {
        eprintln!("EXTENSION PROTOCOLS DETECTED:");
        for action in &analysis.extension_actions {
            eprintln!("  - {}: {}", action.protocol_name(), action.description());
        }
        eprintln!("--------------------------------------------------");
    }

    // Protocol-specific Notices (Plugins)
    if !analysis.extension_notices.is_empty() {
        for notice in &analysis.extension_notices {
            eprintln!("{}", notice);
            eprintln!("--------------------------------------------------");
        }
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
        AnalysisWarning::LookupTableNotProvided => {
            "Address table lookups present but lookup table was not provided; some accounts may be unresolved".to_string()
        }
        AnalysisWarning::LookupTableMissing(key) => {
            format!("Lookup table {} missing or incomplete; some accounts may be unresolved", key)
        }
        AnalysisWarning::TokenTransferDetected(kind) => {
            let label = match kind {
                TokenProgramKind::SplToken => "Token Program",
                TokenProgramKind::Token2022 => "Token-2022 Program",
                TokenProgramKind::AssociatedToken => "Associated Token Program",
            };
            format!(
                "{} interaction detected. Detailed token transfer amounts are not displayed in offline mode.",
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
        AnalysisWarning::MalformedInstruction => {
            "One or more protocol instructions are malformed (too short or corrupted data)".to_string()
        }
    }
}
