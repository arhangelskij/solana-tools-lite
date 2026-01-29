/// Parsing utilities for Light Protocol instruction data.
use super::decoder::{
    decode_u64_at_offset, extract_discriminator_u8, extract_discriminator_u64, decode_transfer2, decode_token_interface_mint_to, decode_batch_compress,
    decode_invoke, decode_invoke_cpi, decode_invoke_cpi_with_readonly, decode_invoke_cpi_with_account_info,
};
use solana_tools_lite::models::pubkey_base58::PubkeyBase58;

/// Parse Light Protocol instruction based on program ID and data.
pub fn parse_light_instruction(program_id: &PubkeyBase58, data: &[u8]) -> super::models::LightProtocolAction {
    use super::models::LightProtocolAction as Action;
    use super::constants;
    
    let program_id_str = program_id.to_string();
    match program_id_str.as_str() {
        // ====================================================================
        // COMPRESSED TOKEN PROGRAM - 1-BYTE DISCRIMINATORS
        // ====================================================================
        constants::COMPRESSED_TOKEN_PROGRAM_ID => {
            if let Some(discriminator) = extract_discriminator_u8(data) {
                match discriminator {
                    constants::DISCRIMINATOR_CTOKEN_TRANSFER => Action::CTokenTransfer { 
                        amount: decode_u64_at_offset(data, constants::OFFSET_CTOKEN_AMOUNT) 
                    },
                    constants::DISCRIMINATOR_CTOKEN_APPROVE => Action::CTokenApprove { 
                        amount: decode_u64_at_offset(data, constants::OFFSET_CTOKEN_AMOUNT) 
                    },
                    constants::DISCRIMINATOR_CTOKEN_REVOKE => Action::CTokenRevoke,
                    constants::DISCRIMINATOR_CTOKEN_MINT_TO => Action::CTokenMintTo { 
                        amount: decode_u64_at_offset(data, constants::OFFSET_CTOKEN_AMOUNT) 
                    },
                    constants::DISCRIMINATOR_CTOKEN_BURN => Action::CTokenBurn { 
                        amount: decode_u64_at_offset(data, constants::OFFSET_CTOKEN_AMOUNT) 
                    },
                    constants::DISCRIMINATOR_CLOSE_TOKEN_ACCOUNT => Action::CloseTokenAccount,
                    constants::DISCRIMINATOR_CTOKEN_FREEZE_ACCOUNT => Action::CTokenFreezeAccount,
                    constants::DISCRIMINATOR_CTOKEN_THAW_ACCOUNT => Action::CTokenThawAccount,
                    constants::DISCRIMINATOR_CTOKEN_TRANSFER_CHECKED => Action::CTokenTransferChecked { 
                        amount: decode_u64_at_offset(data, constants::OFFSET_CTOKEN_AMOUNT) 
                    },
                    constants::DISCRIMINATOR_CTOKEN_MINT_TO_CHECKED => Action::CTokenMintToChecked { 
                        amount: decode_u64_at_offset(data, constants::OFFSET_CTOKEN_AMOUNT) 
                    },
                    constants::DISCRIMINATOR_CTOKEN_BURN_CHECKED => Action::CTokenBurnChecked { 
                        amount: decode_u64_at_offset(data, constants::OFFSET_CTOKEN_AMOUNT) 
                    },
                    constants::DISCRIMINATOR_CREATE_TOKEN_ACCOUNT => Action::CreateTokenAccount,
                    constants::DISCRIMINATOR_CREATE_ASSOCIATED_TOKEN_ACCOUNT => Action::CreateAssociatedTokenAccount,
                    constants::DISCRIMINATOR_TRANSFER2 => decode_transfer2(data),
                    constants::DISCRIMINATOR_CREATE_ASSOCIATED_TOKEN_ACCOUNT_IDEMPOTENT => Action::CreateAssociatedTokenAccountIdempotent,
                    constants::DISCRIMINATOR_MINT_ACTION => Action::MintAction,
                    constants::DISCRIMINATOR_CLAIM => Action::Claim,
                    constants::DISCRIMINATOR_WITHDRAW_FUNDING_POOL => Action::WithdrawFundingPool { 
                        amount: decode_u64_at_offset(data, constants::OFFSET_CTOKEN_AMOUNT)
                    },
                    _ => {
                        if let Some(disc_8) = extract_discriminator_u64(data) {
                            match disc_8 {
                                constants::DISCRIMINATOR_TOKEN_INTERFACE_MINT_TO => decode_token_interface_mint_to(data),
                                constants::DISCRIMINATOR_TOKEN_INTERFACE_TRANSFER => Action::TokenInterfaceTransfer { amount: None },
                                constants::DISCRIMINATOR_BATCH_COMPRESS => decode_batch_compress(data),
                                constants::DISCRIMINATOR_TOKEN_INTERFACE_APPROVE => Action::TokenInterfaceApprove,
                                constants::DISCRIMINATOR_TOKEN_INTERFACE_REVOKE => Action::TokenInterfaceRevoke,
                                constants::DISCRIMINATOR_TOKEN_INTERFACE_FREEZE => Action::TokenInterfaceFreeze,
                                constants::DISCRIMINATOR_CTOKEN_THAW => Action::CTokenThaw,
                                constants::DISCRIMINATOR_CREATE_TOKEN_POOL => Action::CreateTokenPool,
                                constants::DISCRIMINATOR_ADD_TOKEN_POOL => Action::AddTokenPool,
                                constants::DISCRIMINATOR_CTOKEN_FREEZE => Action::CTokenFreeze,
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
            if let Some(discriminator) = extract_discriminator_u64(data) {
                match discriminator {
                    constants::DISCRIMINATOR_INVOKE => decode_invoke(data),
                    constants::DISCRIMINATOR_INVOKE_CPI => decode_invoke_cpi(data),
                    constants::DISCRIMINATOR_INVOKE_CPI_WITH_READ_ONLY => decode_invoke_cpi_with_readonly(data),
                    constants::DISCRIMINATOR_INVOKE_CPI_WITH_ACCOUNT_INFO => decode_invoke_cpi_with_account_info(data),
                    constants::DISCRIMINATOR_INIT_CPI_CONTEXT_ACCOUNT_INSTRUCTION => Action::InitCpiContextAccount,
                    constants::DISCRIMINATOR_RE_INIT_CPI_CONTEXT_ACCOUNT_INSTRUCTION => Action::ReInitCpiContextAccount,
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
            if let Some(discriminator) = extract_discriminator_u64(data) {
                match discriminator {
                    constants::DISCRIMINATOR_INSERT_INTO_QUEUES => Action::InsertIntoQueues,
                    constants::DISCRIMINATOR_INITIALIZE_COMPRESSION_CONFIG => Action::InitializeCompressionConfig,
                    constants::DISCRIMINATOR_UPDATE_COMPRESSION_CONFIG => Action::UpdateCompressionConfig,
                    constants::DISCRIMINATOR_DECOMPRESS_ACCOUNTS_IDEMPOTENT => Action::DecompressAccountsIdempotent,
                    constants::DISCRIMINATOR_COMPRESS_ACCOUNTS_IDEMPOTENT => Action::CompressAccountsIdempotent,
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
            if let Some(discriminator) = extract_discriminator_u64(data) {
                match discriminator {
                    constants::DISCRIMINATOR_CREATE_CONFIG_COUNTER => Action::CreateConfigCounter,
                    constants::DISCRIMINATOR_CREATE_COMPRESSIBLE_CONFIG => Action::CreateCompressibleConfig,
                    constants::DISCRIMINATOR_REGISTRY_CLAIM => Action::RegistryClaim,
                    constants::DISCRIMINATOR_COMPRESS_AND_CLOSE => Action::CompressAndClose,
                    constants::DISCRIMINATOR_REGISTER_FORESTER => Action::RegisterForester,
                    constants::DISCRIMINATOR_REGISTER_FORESTER_EPOCH => Action::RegisterForesterEpoch,
                    constants::DISCRIMINATOR_FINALIZE_REGISTRATION => Action::FinalizeRegistration,
                    constants::DISCRIMINATOR_REPORT_WORK => Action::ReportWork,
                    _ => Action::UnknownEightByte { discriminator },
                }
            } else {
                Action::Unknown { discriminator: 0 }
            }
        }
        
        // ====================================================================
        // SPL NOOP PROGRAM - No discriminators
        // ====================================================================
        constants::SPL_NOOP_PROGRAM_ID => Action::Unknown { discriminator: 0 },
        _ => Action::Unknown { discriminator: 0 },
    }
}
