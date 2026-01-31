use solana_tools_lite::extensions::traits::ProtocolAnalyzer;
use solana_tools_lite::models::analysis::{TxAnalysis, AnalysisWarning};
use solana_tools_lite::extensions::analysis::{PrivacyImpact, AnalysisExtensionAction};
use solana_tools_lite::models::message::Message;
use solana_tools_lite::models::pubkey_base58::PubkeyBase58;
use solana_tools_lite::ToolError;
use std::sync::Arc;

pub mod constants;
pub mod parsing;
pub mod models;
pub mod decoder;

pub use models::LightProtocolAction as Action;

#[cfg(test)]
mod tests;

/// Analyzer for Light Protocol (ZK Compression).
/// 
/// This analyzer detects and classifies Light Protocol instructions in Solana transactions.
/// It supports all major Light Protocol operations including compression, decompression,
/// transfers, and state management.
/// 
/// # Privacy Classification
/// 
/// The analyzer classifies operations into different privacy impact categories:
/// - `StorageCompression`: Operations that compress/decompress assets or manage state
/// - `Confidential`: Operations that involve private transfers or minting
/// - `None`: Unknown or unclassified operations
pub struct LightProtocol;

impl ProtocolAnalyzer for LightProtocol {
    fn name(&self) -> &'static str {
        "Light Protocol"
    }

    // Light Protocol programs
    fn supported_programs(&self) -> Result<&'static [PubkeyBase58], ToolError> {
        constants::supported_programs()
    }

    fn program_description(&self, program_id: &PubkeyBase58) -> Option<&'static str> {
        let program_id_str = program_id.to_string();
        match program_id_str.as_str() {
            constants::LIGHT_SYSTEM_PROGRAM_ID => Some("Light System Program - Core ZK compression operations"),
            constants::ACCOUNT_COMPRESSION_PROGRAM_ID => Some("Account Compression Program - Merkle tree state management"),
            constants::COMPRESSED_TOKEN_PROGRAM_ID => Some("Compressed Token Program - SPL token operations"),
            constants::LIGHT_REGISTRY_ID => Some("Light Registry - Compressible config management"),
            constants::SPL_NOOP_PROGRAM_ID => Some("SPL Noop Program - Proof verification"),
            _ => None,
        }
    }

    fn analyze(
        &self,
        message: &Message,
        account_list: &[PubkeyBase58],
        signer: &PubkeyBase58,
        analysis: &mut TxAnalysis,
    ) {
        let programs = match self.supported_programs() {
            Ok(programs) => programs,
            Err(e) => {
                eprintln!("[CRITICAL] Light Protocol: Failed to initialize program IDs: {}", e);
                return;
            }
        };
        
        for instr in message.instructions() {
            let program_id = match account_list.get(instr.program_id_index as usize) {
                Some(pk) => pk,
                None => continue,
            };

            if !programs.contains(program_id) {
                continue;
            }

            // Validate instruction has minimum required data length
            if instr.data.is_empty() {
                analysis.warnings.push(AnalysisWarning::MalformedInstruction);
                continue;
            }

            let action = parsing::parse_light_instruction(program_id, &instr.data);

            // Record SOL transfers if action provides them
            match &action {
                Action::Invoke { lamports: Some(l), from_index, to_index } |
                Action::InvokeCpi { lamports: Some(l), from_index, to_index } |
                Action::InvokeCpiWithReadOnly { lamports: Some(l), from_index, to_index } |
                Action::InvokeCpiWithAccountInfo { lamports: Some(l), from_index, to_index } => {
                    let from = match from_index {
                        Some(idx) => account_list.get(*idx as usize).map(|pk| pk.to_string()).unwrap_or_else(|| "Unknown".to_string()),
                        None => "Compressed State".to_string(),
                    };
                    let to = match to_index {
                        Some(idx) => account_list.get(*idx as usize).map(|pk| pk.to_string()).unwrap_or_else(|| "Unknown".to_string()),
                        None => "Compressed State".to_string(),
                    };
                    
                    analysis.transfers.push(solana_tools_lite::models::analysis::TransferView {
                        from: from.clone(),
                        to,
                        lamports: *l,
                        from_is_signer: match from_index {
                            Some(idx) => account_list.get(*idx as usize).map(|pk| pk == signer).unwrap_or(false),
                            None => false,
                        },
                    });

                    // Track total SOL sent by signer if applicable
                    if let Some(idx) = from_index {
                        if account_list.get(*idx as usize).map(|pk| pk == signer).unwrap_or(false) {
                            analysis.total_sol_send_by_signer += *l as u128;
                        }
                    }
                }
                _ => {}
            }

            // Count privacy impact regardless of signer involvement
            // Privacy level should reflect the transaction's actual operations, not the signer's role
            match action.privacy_impact() {
                PrivacyImpact::Confidential => analysis.confidential_ops_count += 1,
                PrivacyImpact::StorageCompression => analysis.storage_ops_count += 1,
                PrivacyImpact::Hybrid => {
                    analysis.confidential_ops_count += 1;
                    analysis.storage_ops_count += 1;
                }
                _ => {}
            }

            analysis
                .extension_actions
                .push(AnalysisExtensionAction::new(Arc::new(action)));
        }
    }

    /// Enrich the analysis with custom protocol notices (e.g. ZK Compression warnings).
    fn enrich_notice(&self, analysis: &mut TxAnalysis) {
        let mut notice = String::new();
        
        notice.push_str("!!! ZK COMPRESSION NOTICE !!!\n");
        notice.push_str("This transaction uses ZK Compression (Light Protocol).\n");
        notice.push_str(
            "- Compressed assets are NOT always visible in standard explorers (SolanaFM, Solscan, etc.)\n",
        );
        notice.push_str("- You need a specialized indexer or explorer (e.g. Photon) to view state.\n");
        
        if analysis.confidential_ops_count > 0 {
            notice.push_str("- Valid proofs are required for these instructions.\n");
            notice.push_str("Please verify the integrity of the proof data source.\n");
        }

        notice.push_str("\n");
        notice.push_str(&format!(
            "Note: Network fee ({}) is always public",
            solana_tools_lite::utils::format_sol(analysis.base_fee_lamports)
        ));

        analysis.extension_notices.push(notice);
    }
}
