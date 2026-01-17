use crate::extensions::traits::ProtocolAnalyzer;
use crate::models::analysis::TxAnalysis;
use crate::models::extensions::{AnalysisExtensionAction, LightProtocolAction, PrivacyImpact};
use crate::models::message::Message;
use crate::models::pubkey_base58::PubkeyBase58;

pub mod constants;
pub mod parsing;

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
/// 
/// # Examples
/// 
/// ```rust
/// use solana_tools_lite::extensions::light_protocol::LightProtocol;
/// use solana_tools_lite::extensions::ProtocolAnalyzer;
/// 
/// let analyzer = LightProtocol;
/// let is_light_tx = analyzer.detect(&message);
/// ```
pub struct LightProtocol;

impl ProtocolAnalyzer for LightProtocol {
    fn name(&self) -> &'static str {
        "Light Protocol"
    }

    // Light Protocol programs (verified Jan 2026)
    fn supported_programs(&self) -> &'static [PubkeyBase58] {
        constants::supported_programs()
    }

    fn analyze(
        &self,
        message: &Message,
        account_list: &[PubkeyBase58],
        signer: &PubkeyBase58,
        analysis: &mut TxAnalysis,
    ) {
        let programs = self.supported_programs();

        for instr in message.instructions() {
            let program_id = match account_list.get(instr.program_id_index as usize) {
                Some(pk) => pk,
                None => continue,
            };

            if !programs.contains(program_id) {
                continue;
            }

            // Validate instruction has minimum required data length
            if !parsing::validate_instruction_length(&instr.data, constants::DISCRIMINATOR_SIZE) {
                analysis.warnings.push(crate::models::analysis::AnalysisWarning::MalformedInstruction);
                continue;
            }

            let discriminator = parsing::extract_discriminator(&instr.data);

            let action = match discriminator {
                constants::DISCRIMINATOR_CREATE_MINT => LightProtocolAction::CreateMint,
                constants::DISCRIMINATOR_MINT_TO => LightProtocolAction::MintTo,
                constants::DISCRIMINATOR_TRANSFER => LightProtocolAction::Transfer,
                constants::DISCRIMINATOR_COMPRESS_SOL => {
                    let lamports = parsing::parse_amount_from_instruction(&instr.data);
                    LightProtocolAction::CompressSol { lamports }
                }
                constants::DISCRIMINATOR_COMPRESS_TOKEN => {
                    let amount = parsing::parse_amount_from_instruction(&instr.data);
                    LightProtocolAction::CompressToken { amount }
                }
                constants::DISCRIMINATOR_DECOMPRESS => LightProtocolAction::Decompress,
                constants::DISCRIMINATOR_STATE_UPDATE => LightProtocolAction::StateUpdate,
                constants::DISCRIMINATOR_CLOSE_ACCOUNT => LightProtocolAction::CloseAccount,
                _ => LightProtocolAction::Unknown { discriminator },
            };

            // Signer involvement check: only count if signer is an account in this instruction
            let signer_involved = instr.accounts.iter().any(|&idx| {
                account_list.get(idx as usize).map(|pk| pk == signer).unwrap_or(false)
            });

            if signer_involved {
                match action.privacy_impact() {
                    PrivacyImpact::Confidential => analysis.confidential_ops_count += 1,
                    PrivacyImpact::StorageCompression => analysis.storage_ops_count += 1,
                    _ => {}
                }
            }

            analysis
                .extension_actions
                .push(AnalysisExtensionAction::LightProtocol(action));
        }
    }

    fn enrich_notice(&self, analysis: &TxAnalysis) -> Option<String> {
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

        notice.push_str(&format!(
            "Note: Network fee ({}) is always public",
            crate::utils::format_sol(analysis.base_fee_lamports)
        ));

        Some(notice)
    }
}
