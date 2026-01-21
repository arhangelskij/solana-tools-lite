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

            let action = parse_light_instruction(program_id, &instr.data);

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

/// Parse Light Protocol instruction based on program ID and data.
/// 
/// Handles both 1-byte discriminators (Compressed Token Program) and
/// 8-byte discriminators (Light System, Account Compression, etc.).
fn parse_light_instruction(program_id: &PubkeyBase58, data: &[u8]) -> Action {
    let program_id_str = program_id.to_string();
    
    match program_id_str.as_str() {
        // ====================================================================
        // COMPRESSED TOKEN PROGRAM - 1-BYTE DISCRIMINATORS
        // ====================================================================
        constants::COMPRESSED_TOKEN_PROGRAM_ID => {
            if let Some(discriminator) = parsing::extract_discriminator_u8(data) {
                match discriminator {
                    constants::DISCRIMINATOR_CTOKEN_TRANSFER => Action::CTokenTransfer,
                    constants::DISCRIMINATOR_CTOKEN_APPROVE => Action::CTokenApprove,
                    constants::DISCRIMINATOR_CTOKEN_REVOKE => Action::CTokenRevoke,
                    constants::DISCRIMINATOR_CTOKEN_MINT_TO => Action::CTokenMintTo,
                    constants::DISCRIMINATOR_CTOKEN_BURN => Action::CTokenBurn,
                    constants::DISCRIMINATOR_CLOSE_TOKEN_ACCOUNT => Action::CloseTokenAccount,
                    constants::DISCRIMINATOR_CTOKEN_FREEZE_ACCOUNT => Action::CTokenFreezeAccount,
                    constants::DISCRIMINATOR_CTOKEN_THAW_ACCOUNT => Action::CTokenThawAccount,
                    constants::DISCRIMINATOR_CTOKEN_TRANSFER_CHECKED => Action::CTokenTransferChecked,
                    constants::DISCRIMINATOR_CTOKEN_MINT_TO_CHECKED => Action::CTokenMintToChecked,
                    constants::DISCRIMINATOR_CTOKEN_BURN_CHECKED => Action::CTokenBurnChecked,
                    constants::DISCRIMINATOR_CREATE_TOKEN_ACCOUNT => Action::CreateTokenAccount,
                    constants::DISCRIMINATOR_CREATE_ASSOCIATED_TOKEN_ACCOUNT => Action::CreateAssociatedTokenAccount,
                    constants::DISCRIMINATOR_TRANSFER2 => Action::Transfer2,
                    constants::DISCRIMINATOR_CREATE_ASSOCIATED_TOKEN_ACCOUNT_IDEMPOTENT => Action::CreateAssociatedTokenAccountIdempotent,
                    constants::DISCRIMINATOR_MINT_ACTION => Action::MintAction,
                    constants::DISCRIMINATOR_CLAIM => Action::Claim,
                    constants::DISCRIMINATOR_WITHDRAW_FUNDING_POOL => Action::WithdrawFundingPool,
                    _ => {
                        // Fallback for Anchor/Token Interface instructions (8-byte discriminators)
                        if let Some(disc_8) = parsing::extract_discriminator_u64(data) {
                            match disc_8 {
                                // Token Interface instructions
                                constants::DISCRIMINATOR_TOKEN_INTERFACE_MINT_TO => Action::TokenInterfaceMintTo,
                                constants::DISCRIMINATOR_TOKEN_INTERFACE_TRANSFER => Action::TokenInterfaceTransfer,
                                constants::DISCRIMINATOR_BATCH_COMPRESS => Action::BatchCompress,
                                constants::DISCRIMINATOR_TOKEN_INTERFACE_APPROVE => Action::TokenInterfaceApprove,
                                constants::DISCRIMINATOR_TOKEN_INTERFACE_REVOKE => Action::TokenInterfaceRevoke,
                                constants::DISCRIMINATOR_TOKEN_INTERFACE_FREEZE => Action::TokenInterfaceFreeze,
                                constants::DISCRIMINATOR_TOKEN_INTERFACE_THAW => Action::TokenInterfaceThaw,
                                // Anchor Freeze/Thaw
                                [248, 198, 158, 145, 225, 117, 135, 200] => Action::Freeze,
                                [90, 147, 75, 178, 85, 88, 4, 137] => Action::Thaw,
                                _ => Action::UnknownEightByte { discriminator: disc_8 },
                            }
                        } else {
                            Action::Unknown { discriminator }
                        }
                    }
                }
            } else {
                Action::Unknown { discriminator: 0 }
            }
        }
        
        // ====================================================================
        // LIGHT SYSTEM PROGRAM - 8-BYTE DISCRIMINATORS
        // ====================================================================
        constants::LIGHT_SYSTEM_PROGRAM_ID => {
            if let Some(discriminator) = parsing::extract_discriminator_u64(data) {
                match discriminator {
                    constants::DISCRIMINATOR_INVOKE => Action::Invoke,
                    constants::DISCRIMINATOR_INVOKE_CPI => Action::InvokeCpi,
                    constants::DISCRIMINATOR_INVOKE_CPI_WITH_READ_ONLY => Action::InvokeCpiWithReadOnly,
                    constants::DISCRIMINATOR_INVOKE_CPI_WITH_ACCOUNT_INFO => Action::InvokeCpiWithAccountInfo,
                    _ => Action::UnknownEightByte { discriminator },
                }
            } else {
                Action::Unknown { discriminator: 0 }
            }
        }
        
        // ====================================================================
        // ACCOUNT COMPRESSION PROGRAM - 8-BYTE DISCRIMINATORS
        // ====================================================================
        constants::ACCOUNT_COMPRESSION_PROGRAM_ID => {
            if let Some(discriminator) = parsing::extract_discriminator_u64(data) {
                match discriminator {
                    constants::DISCRIMINATOR_INSERT_INTO_QUEUES => Action::InsertIntoQueues,
                    _ => Action::UnknownEightByte { discriminator },
                }
            } else {
                Action::Unknown { discriminator: 0 }
            }
        }
        
        // ====================================================================
        // LIGHT REGISTRY PROGRAM - 8-BYTE DISCRIMINATORS
        // ====================================================================
        constants::LIGHT_REGISTRY_ID => {
            if let Some(discriminator) = parsing::extract_discriminator_u64(data) {
                match discriminator {
                    constants::DISCRIMINATOR_CREATE_CONFIG_COUNTER => Action::CreateConfigCounter,
                    constants::DISCRIMINATOR_CREATE_COMPRESSIBLE_CONFIG => Action::CreateCompressibleConfig,
                    _ => Action::UnknownEightByte { discriminator },
                }
            } else {
                Action::Unknown { discriminator: 0 }
            }
        }
        
        // ====================================================================
        // SPL NOOP PROGRAM - No discriminators
        // ====================================================================
        constants::SPL_NOOP_PROGRAM_ID => {
            Action::Unknown { discriminator: 0 }
        }
        
        _ => Action::Unknown { discriminator: 0 },
    }
}
