use crate::extensions::traits::ProtocolAnalyzer;
use crate::models::analysis::TxAnalysis;
use crate::models::extensions::{AnalysisExtensionAction, LightProtocolAction, PrivacyImpact};
use crate::models::message::Message;
use crate::models::pubkey_base58::PubkeyBase58;

pub mod constants;

/// Analyzer for Light Protocol (ZK Compression).
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

            if instr.data.len() < constants::DISCRIMINATOR_SIZE {
                analysis.warnings.push(crate::models::analysis::AnalysisWarning::MalformedInstruction);
                continue;
            }

            let discriminator: [u8; constants::DISCRIMINATOR_SIZE] = instr.data
                [0..constants::DISCRIMINATOR_SIZE]
                .try_into()
                .unwrap_or([0u8; constants::DISCRIMINATOR_SIZE]);

            let action = match discriminator {
                constants::DISCRIMINATOR_CREATE_MINT => LightProtocolAction::CreateMint,
                constants::DISCRIMINATOR_MINT_TO => LightProtocolAction::MintTo,
                constants::DISCRIMINATOR_TRANSFER => LightProtocolAction::Transfer,
                constants::DISCRIMINATOR_COMPRESS_SOL => {
                    let lamports = if instr.data.len()
                        >= constants::DISCRIMINATOR_SIZE + constants::U64_SIZE
                    {
                        Some(u64::from_le_bytes(
                            instr.data[constants::DISCRIMINATOR_SIZE
                                ..constants::DISCRIMINATOR_SIZE + constants::U64_SIZE]
                                .try_into()
                                .unwrap_or([0u8; 8]),
                        ))
                    } else {
                        None
                    };
                    LightProtocolAction::CompressSol { lamports }
                }
                constants::DISCRIMINATOR_COMPRESS_TOKEN => {
                    let amount = if instr.data.len()
                        >= constants::DISCRIMINATOR_SIZE + constants::U64_SIZE
                    {
                        Some(u64::from_le_bytes(
                            instr.data[constants::DISCRIMINATOR_SIZE
                                ..constants::DISCRIMINATOR_SIZE + constants::U64_SIZE]
                                .try_into()
                                .unwrap_or([0u8; 8]),
                        ))
                    } else {
                        None
                    };
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
