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

            //TODO: 🟡 move parse_light_instruction into parser
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

/// Extract u64 amount from instruction data at specified offset.
fn extract_u64_amount(data: &[u8], offset: usize) -> Option<u64> {
    parsing::parse_u64_at_offset(data, offset)
}

// TODO: 🔴  move parse_light_instruction into parser

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
                    constants::DISCRIMINATOR_CTOKEN_TRANSFER => Action::CTokenTransfer { 
                        amount: extract_u64_amount(data, constants::OFFSET_CTOKEN_AMOUNT) 
                    },
                    constants::DISCRIMINATOR_CTOKEN_APPROVE => Action::CTokenApprove { 
                        amount: extract_u64_amount(data, constants::OFFSET_CTOKEN_AMOUNT) 
                    },
                    constants::DISCRIMINATOR_CTOKEN_REVOKE => Action::CTokenRevoke,
                    constants::DISCRIMINATOR_CTOKEN_MINT_TO => Action::CTokenMintTo { 
                        amount: extract_u64_amount(data, constants::OFFSET_CTOKEN_AMOUNT) 
                    },
                    constants::DISCRIMINATOR_CTOKEN_BURN => Action::CTokenBurn { 
                        amount: extract_u64_amount(data, constants::OFFSET_CTOKEN_AMOUNT) 
                    },
                    constants::DISCRIMINATOR_CLOSE_TOKEN_ACCOUNT => Action::CloseTokenAccount,
                    constants::DISCRIMINATOR_CTOKEN_FREEZE_ACCOUNT => Action::CTokenFreezeAccount,
                    constants::DISCRIMINATOR_CTOKEN_THAW_ACCOUNT => Action::CTokenThawAccount,
                    constants::DISCRIMINATOR_CTOKEN_TRANSFER_CHECKED => Action::CTokenTransferChecked { 
                        amount: extract_u64_amount(data, constants::OFFSET_CTOKEN_AMOUNT) 
                    },
                    constants::DISCRIMINATOR_CTOKEN_MINT_TO_CHECKED => Action::CTokenMintToChecked { 
                        amount: extract_u64_amount(data, constants::OFFSET_CTOKEN_AMOUNT) 
                    },
                    constants::DISCRIMINATOR_CTOKEN_BURN_CHECKED => Action::CTokenBurnChecked { 
                        amount: extract_u64_amount(data, constants::OFFSET_CTOKEN_AMOUNT) 
                    },
                    constants::DISCRIMINATOR_CREATE_TOKEN_ACCOUNT => Action::CreateTokenAccount,
                    constants::DISCRIMINATOR_CREATE_ASSOCIATED_TOKEN_ACCOUNT => Action::CreateAssociatedTokenAccount,
                    constants::DISCRIMINATOR_TRANSFER2 => parse_transfer2(data),
                    constants::DISCRIMINATOR_CREATE_ASSOCIATED_TOKEN_ACCOUNT_IDEMPOTENT => Action::CreateAssociatedTokenAccountIdempotent,
                    constants::DISCRIMINATOR_MINT_ACTION => Action::MintAction,
                    constants::DISCRIMINATOR_CLAIM => Action::Claim,
                    constants::DISCRIMINATOR_WITHDRAW_FUNDING_POOL => Action::WithdrawFundingPool { 
                        amount: extract_u64_amount(data, constants::OFFSET_CTOKEN_AMOUNT) // Assuming standard layout after 1-byte discriminator
                    },
                    _ => {
                        // Fallback for Anchor/Token Interface instructions (8-byte discriminators)
                        if let Some(disc_8) = parsing::extract_discriminator_u64(data) {
                            match disc_8 {
                                // Token Interface instructions
                                constants::DISCRIMINATOR_TOKEN_INTERFACE_MINT_TO => Action::TokenInterfaceMintTo { 
                                    amount: extract_u64_amount(data, constants::OFFSET_TOKEN_INTERFACE_AMOUNT) 
                                },
                                constants::DISCRIMINATOR_TOKEN_INTERFACE_TRANSFER => Action::TokenInterfaceTransfer { 
                                    amount: extract_u64_amount(data, constants::OFFSET_TOKEN_INTERFACE_AMOUNT) 
                                },
                                constants::DISCRIMINATOR_BATCH_COMPRESS => parse_batch_compress(data),
                                constants::DISCRIMINATOR_TOKEN_INTERFACE_APPROVE => Action::TokenInterfaceApprove,
                                constants::DISCRIMINATOR_TOKEN_INTERFACE_REVOKE => Action::TokenInterfaceRevoke,
                                constants::DISCRIMINATOR_TOKEN_INTERFACE_FREEZE => Action::TokenInterfaceFreeze,
                                constants::DISCRIMINATOR_TOKEN_INTERFACE_THAW => Action::TokenInterfaceThaw,
                                constants::DISCRIMINATOR_CREATE_TOKEN_POOL => Action::CreateTokenPool,
                                constants::DISCRIMINATOR_ADD_TOKEN_POOL => Action::AddTokenPool,
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
                    constants::DISCRIMINATOR_INVOKE => Action::Invoke { 
                        lamports: extract_u64_amount(data, constants::OFFSET_TOKEN_INTERFACE_AMOUNT) 
                    },
                    constants::DISCRIMINATOR_INVOKE_CPI => Action::InvokeCpi { 
                        lamports: extract_u64_amount(data, constants::OFFSET_TOKEN_INTERFACE_AMOUNT) 
                    },
                    constants::DISCRIMINATOR_INVOKE_CPI_WITH_READ_ONLY => Action::InvokeCpiWithReadOnly { 
                        lamports: extract_u64_amount(data, constants::OFFSET_TOKEN_INTERFACE_AMOUNT) 
                    },
                    constants::DISCRIMINATOR_INVOKE_CPI_WITH_ACCOUNT_INFO => Action::InvokeCpiWithAccountInfo { 
                        lamports: extract_u64_amount(data, constants::OFFSET_TOKEN_INTERFACE_AMOUNT) 
                    },
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


// TODO: 23jan 🔴 move into local extension – codec(decoder)
/// Deep parsing for Transfer2 instruction.
fn parse_transfer2(data: &[u8]) -> Action {
    let mut cursor = 1; // Skip discriminator
    let mut total_amount: u64 = 0;

    // Fixed fields (7 bytes)
    // with_transaction_hash: bool, with_lamports_change_account_merkle_tree_index: bool,
    // lamports_change_account_merkle_tree_index: u8, lamports_change_account_owner_index: u8,
    // output_queue: u8, max_top_up: u16
    if data.len() < cursor + 7 {
        return Action::Transfer2 { in_lamports: None, out_lamports: None, amount: None };
    }
    cursor += 7;

    // Helper to safely advance cursor
    macro_rules! advance {
        ($opt:expr) => {
            match $opt {
                Some(consumed) => cursor += consumed,
                None => return Action::Transfer2 { in_lamports: None, out_lamports: None, amount: None },
            }
        };
    }

    // cpi_context: Option<CompressedCpiContext>
    if let Some(&disc) = data.get(cursor) {
        cursor += 1;
        if disc == 1 {
            // Some(CompressedCpiContext)
            cursor += 4; // programIndex
            if let Some(&acc_disc) = data.get(cursor) {
                cursor += 1;
                if acc_disc == 1 {
                    cursor += 8; // AccountContext (2 * u32)
                }
            } else { return Action::Transfer2 { in_lamports: None, out_lamports: None, amount: None }; }
        }
    } else { return Action::Transfer2 { in_lamports: None, out_lamports: None, amount: None }; }

    // compressions: Option<Vec<Compression>>
    if let Some(&disc) = data.get(cursor) {
        cursor += 1;
        if disc == 1 {
            let (sum, consumed) = match parsing::parse_borsh_vec_amount(&data[cursor..], 31, 1) {
                Some(res) => res,
                None => return Action::Transfer2 { in_lamports: None, out_lamports: None, amount: None },
            };
            total_amount = total_amount.saturating_add(sum);
            cursor += consumed;
        }
    } else { return Action::Transfer2 { in_lamports: None, out_lamports: None, amount: None }; }

    // proof: Option<CompressedProof>
    if let Some(&disc) = data.get(cursor) {
        cursor += 1;
        if disc == 1 {
            // skip proof: vec(u8), vec(vec(u8)), vec(u8)
            advance!(parsing::skip_borsh_vec(&data[cursor..], 1)); // a
            // b: vec(vec(u8))
            let (b_len, b_consumed) = match parsing::parse_borsh_u32(&data[cursor..]) {
                Some(res) => res,
                None => return Action::Transfer2 { in_lamports: None, out_lamports: None, amount: None },
            };
            cursor += b_consumed;
            for _ in 0..b_len {
                advance!(parsing::skip_borsh_vec(&data[cursor..], 1));
            }
            advance!(parsing::skip_borsh_vec(&data[cursor..], 1)); // c
        }
    } else { return Action::Transfer2 { in_lamports: None, out_lamports: None, amount: None }; }

    // in_token_data: Vec<MultiInputTokenDataWithContext>
    let (in_len, in_consumed) = match parsing::parse_borsh_u32(&data[cursor..]) {
        Some(res) => res,
        None => return Action::Transfer2 { in_lamports: None, out_lamports: None, amount: None },
    };
    cursor += in_consumed;
    for _ in 0..in_len {
        let (amt, _) = match parsing::parse_borsh_u64(&data[cursor..]) {
            Some(a) => a,
            None => return Action::Transfer2 { in_lamports: None, out_lamports: None, amount: None },
        };
        total_amount = total_amount.saturating_add(amt);
        cursor += 8; // amount
        if let Some(&_has_delegate) = data.get(cursor) {
            cursor += 1;
            if let Some(&opt_disc) = data.get(cursor) {
                cursor += 1;
                if opt_disc == 1 { cursor += 4; }
            } else { return Action::Transfer2 { in_lamports: None, out_lamports: None, amount: None }; }
        } else { return Action::Transfer2 { in_lamports: None, out_lamports: None, amount: None }; }
        cursor += 4 + 4 + 1; // tokenIdx + poolIdx + bump
    }

    // out_token_data: Vec<MultiTokenTransferOutputData>
    let (out_sum, out_consumed) = match parsing::parse_borsh_vec_amount(&data[cursor..], 21, 0) {
        Some(res) => res,
        None => return Action::Transfer2 { in_lamports: None, out_lamports: None, amount: None },
    };
    total_amount = total_amount.saturating_add(out_sum);
    cursor += out_consumed;

    // in_lamports: Option<Vec<u64>>
    let in_lamports = match parsing::parse_borsh_option_vec_u64(&data[cursor..]) {
        Some((opt_vec, len)) => {
            cursor += len;
            opt_vec.map(|v| v.iter().sum())
        }
        None => None,
    };

    // out_lamports: Option<Vec<u64>>
    let out_lamports = match parsing::parse_borsh_option_vec_u64(&data[cursor..]) {
        Some((opt_vec, _)) => {
            opt_vec.map(|v| v.iter().sum())
        }
        None => None,
    };

    Action::Transfer2 {
        in_lamports,
        out_lamports,
        amount: Some(total_amount),
    }
}

// TODO: 23jan 🔴 move into local extension – codec(decoder)
/// Deep parsing for BatchCompress instruction.
fn parse_batch_compress(data: &[u8]) -> Action {
    let mut cursor = 8; // Skip discriminator
    let mut sum_amounts: Option<u64> = None;

    // pubkeys: Vec<[u8; 32]>
    if let Some(consumed) = parsing::skip_borsh_vec(&data[cursor..], 32) {
        cursor += consumed;
    } else {
        return Action::BatchCompress { amount: None };
    }

    // amounts: Option<Vec<u64>>
    if let Some(&disc) = data.get(cursor) {
        cursor += 1;
        if disc == 1 {
            if let Some((v, len)) = parsing::parse_borsh_vec_u64(&data[cursor..]) {
                sum_amounts = Some(v.iter().sum());
                cursor += len;
            }
        }
    } else { return Action::BatchCompress { amount: None }; }

    // lamports: Option<u64>
    if let Some(&disc) = data.get(cursor) {
        cursor += 1;
        if disc == 1 {
            cursor += 8;
        }
    } else { return Action::BatchCompress { amount: sum_amounts }; }

    // amount: Option<u64>
    let priority_amount = if let Some(&disc) = data.get(cursor) {
        cursor += 1;
        if disc == 1 {
            parsing::parse_u64_at_offset(data, cursor)
        } else {
            None
        }
    } else { None };

    Action::BatchCompress { amount: priority_amount.or(sum_amounts) }
}
