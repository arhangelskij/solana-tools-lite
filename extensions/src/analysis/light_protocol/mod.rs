use solana_tools_lite::extensions::traits::ProtocolAnalyzer;
use solana_tools_lite::models::analysis::{TxAnalysis, AnalysisWarning};
use solana_tools_lite::models::extensions::{PrivacyImpact, AnalysisExtensionAction};
use solana_tools_lite::models::message::Message;
use solana_tools_lite::models::pubkey_base58::PubkeyBase58;
use solana_tools_lite::ToolError;
use std::sync::Arc;

pub mod constants;
pub mod parsing;
pub mod errors;
pub mod models;

pub use models::LightProtocolAction;

#[cfg(test)]
mod tests;

use models::LightProtocolAction as Action;
use constants::DISCRIMINATOR_SIZE;

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
                // Configuration error - should never happen with valid program ID constants
                // Indicates build-time misconfiguration or data corruption
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
            if instr.data.len() < DISCRIMINATOR_SIZE {
                analysis.warnings.push(AnalysisWarning::MalformedInstruction);
                continue;
            }

            let discriminator = parsing::extract_discriminator(&instr.data);

            let action = match discriminator {
                constants::DISCRIMINATOR_CREATE_MINT => Action::CreateMint,
                constants::DISCRIMINATOR_MINT_TO => Action::MintTo,
                constants::DISCRIMINATOR_TRANSFER => Action::Transfer,
                constants::DISCRIMINATOR_COMPRESS_SOL => {
                    let lamports = parsing::parse_u64_at_offset(&instr.data, DISCRIMINATOR_SIZE);
                    Action::CompressSol { lamports }
                }
                constants::DISCRIMINATOR_COMPRESS_TOKEN => {
                    let amount = parsing::parse_u64_at_offset(&instr.data, DISCRIMINATOR_SIZE);
                    Action::CompressToken { amount }
                }
                constants::DISCRIMINATOR_DECOMPRESS => Action::Decompress,
                constants::DISCRIMINATOR_STATE_UPDATE => Action::StateUpdate,
                constants::DISCRIMINATOR_CLOSE_ACCOUNT => Action::CloseAccount,
                _ => Action::Unknown { discriminator },
            };

            // Signer involvement check: only count if signer is an account in this instruction
            let signer_involved = instr.accounts.iter().any(|&idx| {
                account_list.get(idx as usize).map(|pk| pk == signer).unwrap_or(false)
            });

            if signer_involved {
                match action.privacy_impact() {
                    PrivacyImpact::Confidential => analysis.confidential_ops_count += 1,
                    PrivacyImpact::StorageCompression => analysis.storage_ops_count += 1,
                    PrivacyImpact::Hybrid => {
                        analysis.confidential_ops_count += 1;
                        analysis.storage_ops_count += 1;
                    }
                    _ => {}
                }
            }

            analysis
                .extension_actions
                .push(AnalysisExtensionAction::new(Arc::new(action)));
        }
    }

    fn enrich_notice(&self, analysis: &mut TxAnalysis) {
        let mut notice = String::new();
        
        notice.push_str("!!! ZK COMPRESSION NOTICE !!!\n");
        notice.push_str("This transaction uses ZK Compression (Light Protocol).\n");
        notice.push_str(
            "- Compressed assets are NOT visible in standard explorers (SolanaFM, Solscan, etc.)\n",
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
