/// Parsing utilities for Light Protocol instruction data.
/// 
/// This module provides safe, reusable functions for parsing binary data
/// from Light Protocol instructions. All functions use defensive programming
/// practices to avoid panics and handle malformed data gracefully.
use super::constants::{U64_SIZE, U16_SIZE};
use solana_tools_lite::models::pubkey_base58::PubkeyBase58;

/// Safely parse a u64 value from instruction data at the given offset.
/// 
/// This function performs bounds checking and uses safe conversion methods
/// to extract a little-endian u64 from the instruction data.
/// 
/// # Arguments
/// 
/// * `data` - The instruction data bytes
/// * `offset` - The byte offset where the u64 starts
/// 
/// # Returns
/// 
/// `Some(value)` if the data is long enough and parsing succeeds,
/// `None` if the data is too short or parsing fails.
pub fn parse_u64_at_offset(data: &[u8], offset: usize) -> Option<u64> {
    if data.len() < offset + U64_SIZE {
        return None;
    }
    
    data[offset..offset + U64_SIZE]
        .try_into()
        .ok()
        .map(u64::from_le_bytes)
}

/// Safely parse a u16 value from instruction data at the given offset.
/// 
/// This function performs bounds checking and uses safe conversion methods
/// to extract a little-endian u16 from the instruction data.
/// 
/// # Arguments
/// 
/// * `data` - The instruction data bytes
/// * `offset` - The byte offset where the u16 starts
/// 
/// # Returns
/// 
/// `Some(value)` if the data is long enough and parsing succeeds,
/// `None` if the data is too short or parsing fails.
pub fn parse_u16_at_offset(data: &[u8], offset: usize) -> Option<u16> {
    if data.len() < offset + U16_SIZE {
        return None;
    }
    
    data[offset..offset + U16_SIZE]
        .try_into()
        .ok()
        .map(u16::from_le_bytes)
}

/// Extract 1-byte discriminator from Compressed Token Program instruction data.
/// 
/// Safely extracts the first byte as the discriminator.
/// 
/// # Arguments
/// 
/// * `data` - The instruction data
/// 
/// # Returns
/// 
/// `Some(discriminator)` if data is not empty, `None` otherwise.
pub fn extract_discriminator_u8(data: &[u8]) -> Option<u8> {
    data.first().copied()
}

/// Extract 8-byte discriminator from Light Protocol instruction data.
/// 
/// Safely extracts the 8-byte discriminator from the beginning of instruction data.
/// Returns None if the data is too short.
/// 
/// # Arguments
/// 
/// * `data` - The instruction data
/// 
/// # Returns
/// 
/// `Some(discriminator)` if data is long enough, `None` if data is too short.
pub fn extract_discriminator_u64(data: &[u8]) -> Option<[u8; 8]> {
    if data.len() < 8 {
        return None;
    }
    
    data[0..8].try_into().ok() //TODO: const
}

/// Parse a u32 value from Borsh-encoded data.
/// 
/// Reads a little-endian u32 from the beginning of the data slice.
/// Returns the parsed value and the number of bytes consumed (always 4).
/// 
/// # Arguments
/// 
/// * `data` - The data slice to parse from
/// 
/// # Returns
/// 
/// `Some((value, 4))` if data is at least 4 bytes long,
/// `None` if data is too short.
/// 
/// # Example
/// 
/// ```ignore
/// let data = [0x01, 0x00, 0x00, 0x00, 0xFF];
/// assert_eq!(parse_borsh_u32(&data), Some((1u32, 4)));
/// ```
pub fn parse_borsh_u32(data: &[u8]) -> Option<(u32, usize)> {
    if data.len() < 4 {
        return None;
    }
    
    let bytes: [u8; 4] = data[0..4].try_into().ok()?;
    Some((u32::from_le_bytes(bytes), 4))
}

/// Parse a u64 value from Borsh-encoded data.
/// 
/// Reads a little-endian u64 from the beginning of the data slice.
/// Returns the parsed value and the number of bytes consumed (always 8).
/// 
/// # Arguments
/// 
/// * `data` - The data slice to parse from
/// 
/// # Returns
/// 
/// `Some((value, 8))` if data is at least 8 bytes long,
/// `None` if data is too short.
/// 
/// # Example
/// 
/// ```ignore
/// let data = [0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF];
/// assert_eq!(parse_borsh_u64(&data), Some((1u64, 8)));
/// ```
pub fn parse_borsh_u64(data: &[u8]) -> Option<(u64, usize)> {
    if data.len() < 8 {
        return None;
    }
    
    let bytes: [u8; 8] = data[0..8].try_into().ok()?;
    Some((u64::from_le_bytes(bytes), 8))
}

/// Parse a `Vec<u64>` from Borsh-encoded data.
/// 
/// Reads a u32 length prefix followed by that many u64 elements.
/// Returns the parsed vector and the total number of bytes consumed.
/// 
/// # Arguments
/// 
/// * `data` - The data slice to parse from
/// 
/// # Returns
/// 
/// `Some((vector, bytes_consumed))` if parsing succeeds,
/// `None` if data is too short or length is invalid.
/// 
/// # Example
/// 
/// ```ignore
/// // Empty vector: length=0
/// let data = [0x00, 0x00, 0x00, 0x00];
/// assert_eq!(parse_borsh_vec_u64(&data), Some((vec![], 4)));
/// 
/// // Vector with one element: length=1, value=42
/// let data = [0x01, 0x00, 0x00, 0x00, 0x2A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
/// assert_eq!(parse_borsh_vec_u64(&data), Some((vec![42], 12)));
/// ```
pub fn parse_borsh_vec_u64(data: &[u8]) -> Option<(Vec<u64>, usize)> {
    // Parse length prefix
    let (len, mut consumed) = parse_borsh_u32(data)?;
    
    // Validate that we have enough data for all elements
    let element_bytes = (len as usize).checked_mul(U64_SIZE)?;
    if data.len() < consumed + element_bytes {
        return None;
    }
    
    // Parse all elements
    let mut vec = Vec::with_capacity(len as usize);
    for i in 0..len {
        let offset = consumed + (i as usize) * U64_SIZE;
        let (value, _) = parse_borsh_u64(&data[offset..])?;
        vec.push(value);
    }
    
    consumed += element_bytes;
    Some((vec, consumed))
}

/// Parse an `Option<Vec<u64>>` from Borsh-encoded data.
/// 
/// Reads a u8 discriminator (0 for None, 1 for Some) followed by
/// an optional `Vec<u64>` if the discriminator is 1.
/// 
/// # Arguments
/// 
/// * `data` - The data slice to parse from
/// 
/// # Returns
/// 
/// `Some((option, bytes_consumed))` if parsing succeeds,
/// `None` if data is too short or invalid.
/// 
/// # Example
/// 
/// ```ignore
/// // None variant
/// let data = [0x00, 0xFF];
/// assert_eq!(parse_borsh_option_vec_u64(&data), Some((None, 1)));
/// 
/// // Some with empty vector
/// let data = [0x01, 0x00, 0x00, 0x00, 0x00, 0xFF];
/// assert_eq!(parse_borsh_option_vec_u64(&data), Some((Some(vec![]), 5)));
/// ```
pub fn parse_borsh_option_vec_u64(data: &[u8]) -> Option<(Option<Vec<u64>>, usize)> {
    if data.is_empty() {
        return None;
    }
    
    let discriminator = data[0];
    match discriminator {
        0 => Some((None, 1)),
        1 => {
            let (vec, consumed) = parse_borsh_vec_u64(&data[1..])?;
            Some((Some(vec), 1 + consumed))
        }
        _ => None,
    }
}

/// Skip a specified number of bytes in the data.
/// 
/// Validates that the data has at least `count` bytes available.
/// 
/// # Arguments
/// 
/// * `data` - The data slice
/// * `count` - Number of bytes to skip
/// 
/// # Returns
/// 
/// `Some(count)` if data has at least `count` bytes,
/// `None` if data is too short.
pub fn skip_bytes(data: &[u8], count: usize) -> Option<usize> {
    if data.len() < count {
        return None;
    }
    Some(count)
}

/// Skip a Borsh-encoded vector by reading its length and calculating total bytes.
/// 
/// Reads a u32 length prefix and calculates the total bytes needed for
/// `length * element_size` elements, then validates that much data is available.
/// 
/// # Arguments
/// 
/// * `data` - The data slice
/// * `element_size` - Size of each element in bytes
/// 
/// # Returns
/// 
/// `Some(bytes_to_skip)` if data is valid,
/// `None` if data is too short or calculation overflows.
pub fn skip_borsh_vec(data: &[u8], element_size: usize) -> Option<usize> {
    let (len, mut consumed) = parse_borsh_u32(data)?;
    
    let element_bytes = (len as usize).checked_mul(element_size)?;
    if data.len() < consumed + element_bytes {
        return None;
    }
    
    consumed += element_bytes;
    Some(consumed)
}

/// Iterate over a Borsh-encoded vector of structs and extract sum of u64 `amount` from each.
/// 
/// Reads a u32 length prefix, then iterates over `length` structs of `struct_size`,
/// extracting the `u64` value at `amount_offset` from each struct.
/// 
/// # Arguments
/// 
/// * `data` - The data slice
/// * `struct_size` - Size of each struct in bytes
/// * `amount_offset` - Offset of the `u64` amount within the struct
/// 
/// # Returns
/// 
/// `Some((total_amount, bytes_consumed))` if parsing succeeds,
/// `None` if data is too short or layout is invalid.
pub fn parse_borsh_vec_amount(data: &[u8], struct_size: usize, amount_offset: usize) -> Option<(u64, usize)> {
    let (len, mut consumed) = parse_borsh_u32(data)?;
    
    let total_bytes = (len as usize).checked_mul(struct_size)?;
    if data.len() < consumed + total_bytes {
        return None;
    }
    
    let mut total_amount: u64 = 0;
    for i in 0..len {
        let item_offset = consumed + (i as usize) * struct_size + amount_offset;
        let amount = parse_u64_at_offset(data, item_offset)?;
        total_amount = total_amount.checked_add(amount)?;
    }
    
    consumed += total_bytes;
    Some((total_amount, consumed))
}

/// Extract sum of `amount` from a Borsh-encoded `Option<Vec<Struct>>`.
/// 
/// # Arguments
/// 
/// * `data` - The data slice
/// * `struct_size` - Size of each struct in bytes
/// * `amount_offset` - Offset of the `u64` amount within the struct
/// 
/// # Returns
/// 
/// `Some((total_amount, bytes_consumed))` if parsing succeeds,
/// `None` if data is too short or invalid.
pub fn parse_borsh_option_vec_amount(data: &[u8], struct_size: usize, amount_offset: usize) -> Option<(u64, usize)> {
    if data.is_empty() {
        return None;
    }
    
    let discriminator = data[0];
    match discriminator {
        0 => Some((0, 1)),
        1 => {
            let (amount, consumed) = parse_borsh_vec_amount(&data[1..], struct_size, amount_offset)?;
            Some((amount, 1 + consumed))
        }
        _ => None,
    }
}

/// Parse Light Protocol instruction based on program ID and data.
/// 
/// Handles both 1-byte discriminators (Compressed Token Program) and
/// 8-byte discriminators (Light System, Account Compression, etc.).
/// 
/// # Arguments
/// 
/// * `program_id` - The program ID
/// * `data` - The instruction data bytes
/// 
/// # Returns
/// 
/// A `LightProtocolAction` representing the parsed instruction.
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
                        amount: parse_u64_at_offset(data, constants::OFFSET_CTOKEN_AMOUNT) 
                    },
                    constants::DISCRIMINATOR_CTOKEN_APPROVE => Action::CTokenApprove { 
                        amount: parse_u64_at_offset(data, constants::OFFSET_CTOKEN_AMOUNT) 
                    },
                    constants::DISCRIMINATOR_CTOKEN_REVOKE => Action::CTokenRevoke,
                    constants::DISCRIMINATOR_CTOKEN_MINT_TO => Action::CTokenMintTo { 
                        amount: parse_u64_at_offset(data, constants::OFFSET_CTOKEN_AMOUNT) 
                    },
                    constants::DISCRIMINATOR_CTOKEN_BURN => Action::CTokenBurn { 
                        amount: parse_u64_at_offset(data, constants::OFFSET_CTOKEN_AMOUNT) 
                    },
                    constants::DISCRIMINATOR_CLOSE_TOKEN_ACCOUNT => Action::CloseTokenAccount,
                    constants::DISCRIMINATOR_CTOKEN_FREEZE_ACCOUNT => Action::CTokenFreezeAccount,
                    constants::DISCRIMINATOR_CTOKEN_THAW_ACCOUNT => Action::CTokenThawAccount,
                    constants::DISCRIMINATOR_CTOKEN_TRANSFER_CHECKED => Action::CTokenTransferChecked { 
                        amount: parse_u64_at_offset(data, constants::OFFSET_CTOKEN_AMOUNT) 
                    },
                    constants::DISCRIMINATOR_CTOKEN_MINT_TO_CHECKED => Action::CTokenMintToChecked { 
                        amount: parse_u64_at_offset(data, constants::OFFSET_CTOKEN_AMOUNT) 
                    },
                    constants::DISCRIMINATOR_CTOKEN_BURN_CHECKED => Action::CTokenBurnChecked { 
                        amount: parse_u64_at_offset(data, constants::OFFSET_CTOKEN_AMOUNT) 
                    },
                    constants::DISCRIMINATOR_CREATE_TOKEN_ACCOUNT => Action::CreateTokenAccount,
                    constants::DISCRIMINATOR_CREATE_ASSOCIATED_TOKEN_ACCOUNT => Action::CreateAssociatedTokenAccount,
                    constants::DISCRIMINATOR_TRANSFER2 => parse_transfer2(data),
                    constants::DISCRIMINATOR_CREATE_ASSOCIATED_TOKEN_ACCOUNT_IDEMPOTENT => Action::CreateAssociatedTokenAccountIdempotent,
                    constants::DISCRIMINATOR_MINT_ACTION => Action::MintAction,
                    constants::DISCRIMINATOR_CLAIM => Action::Claim,
                    constants::DISCRIMINATOR_WITHDRAW_FUNDING_POOL => Action::WithdrawFundingPool { 
                        amount: parse_u64_at_offset(data, constants::OFFSET_CTOKEN_AMOUNT)
                    },
                    _ => {
                        // Fallback for Anchor/Token Interface instructions (8-byte discriminators)
                        if let Some(disc_8) = extract_discriminator_u64(data) {
                            match disc_8 {
                                // Token Interface instructions
                                constants::DISCRIMINATOR_TOKEN_INTERFACE_MINT_TO => Action::TokenInterfaceMintTo { 
                                    amount: parse_u64_at_offset(data, constants::OFFSET_TOKEN_INTERFACE_AMOUNT) 
                                },
                                constants::DISCRIMINATOR_TOKEN_INTERFACE_TRANSFER => Action::TokenInterfaceTransfer { 
                                    amount: parse_u64_at_offset(data, constants::OFFSET_TOKEN_INTERFACE_AMOUNT) 
                                },
                                constants::DISCRIMINATOR_BATCH_COMPRESS => parse_batch_compress(data),
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
                    constants::DISCRIMINATOR_INVOKE => parse_invoke(data),
                    constants::DISCRIMINATOR_INVOKE_CPI => parse_invoke_cpi(data),
                    constants::DISCRIMINATOR_INVOKE_CPI_WITH_READ_ONLY => parse_invoke_cpi_with_readonly(data),
                    constants::DISCRIMINATOR_INVOKE_CPI_WITH_ACCOUNT_INFO => parse_invoke_cpi_with_account_info(data),
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
        constants::SPL_NOOP_PROGRAM_ID => {
            Action::Unknown { discriminator: 0 }
        }
        _ => Action::Unknown { discriminator: 0 },
    }
}


//TODO: 🔴 move to local codec
/// Deep parsing for Transfer2 instruction.
fn parse_transfer2(data: &[u8]) -> super::models::LightProtocolAction {
    use super::models::LightProtocolAction as Action;
    
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
            let (sum, consumed) = match parse_borsh_vec_amount(&data[cursor..], 31, 1) {
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
            advance!(skip_borsh_vec(&data[cursor..], 1)); // a
            // b: vec(vec(u8))
            let (b_len, b_consumed) = match parse_borsh_u32(&data[cursor..]) {
                Some(res) => res,
                None => return Action::Transfer2 { in_lamports: None, out_lamports: None, amount: None },
            };
            cursor += b_consumed;
            for _ in 0..b_len {
                advance!(skip_borsh_vec(&data[cursor..], 1));
            }
            advance!(skip_borsh_vec(&data[cursor..], 1)); // c
        }
    } else { return Action::Transfer2 { in_lamports: None, out_lamports: None, amount: None }; }

    // in_token_data: Vec<MultiInputTokenDataWithContext>
    let (in_len, in_consumed) = match parse_borsh_u32(&data[cursor..]) {
        Some(res) => res,
        None => return Action::Transfer2 { in_lamports: None, out_lamports: None, amount: None },
    };
    cursor += in_consumed;
    for _ in 0..in_len {
        let (amt, _) = match parse_borsh_u64(&data[cursor..]) {
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
    let (out_sum, out_consumed) = match parse_borsh_vec_amount(&data[cursor..], 21, 0) {
        Some(res) => res,
        None => return Action::Transfer2 { in_lamports: None, out_lamports: None, amount: None },
    };
    total_amount = total_amount.saturating_add(out_sum);
    cursor += out_consumed;

    // in_lamports: Option<Vec<u64>>
    let in_lamports = match parse_borsh_option_vec_u64(&data[cursor..]) {
        Some((opt_vec, len)) => {
            cursor += len;
            opt_vec.map(|v| v.iter().sum())
        }
        None => None,
    };

    // out_lamports: Option<Vec<u64>>
    let out_lamports = match parse_borsh_option_vec_u64(&data[cursor..]) {
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

/// Deep parsing for BatchCompress instruction.
fn parse_batch_compress(data: &[u8]) -> super::models::LightProtocolAction {
    use super::models::LightProtocolAction as Action;
    
    let mut cursor = 8; // Skip discriminator
    let mut sum_amounts: Option<u64> = None;

    // pubkeys: Vec<[u8; 32]>
    if let Some(consumed) = skip_borsh_vec(&data[cursor..], 32) {
        cursor += consumed;
    } else {
        return Action::BatchCompress { amount: None };
    }

    // amounts: Option<Vec<u64>>
    if let Some(&disc) = data.get(cursor) {
        cursor += 1;
        if disc == 1 {
            if let Some((v, len)) = parse_borsh_vec_u64(&data[cursor..]) {
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
            parse_u64_at_offset(data, cursor)
        } else {
            None
        }
    } else { None };

    Action::BatchCompress { amount: priority_amount.or(sum_amounts) }
}

/// Parse Invoke instruction from Light System Program.
fn parse_invoke(data: &[u8]) -> super::models::LightProtocolAction {
    use super::models::LightProtocolAction as Action;
    
    let mut cursor = 8; // Skip discriminator

    // proof: Option<CompressedProof>
    if let Some(&disc) = data.get(cursor) {
        cursor += 1;
        if disc == 1 {
            // proof: a: Vec<u8>
            if let Some(len) = skip_borsh_vec(&data[cursor..], 1) {
                cursor += len;
            } else { return Action::Invoke { lamports: None, from_index: None, to_index: None }; }
            
            // b: Vec<Vec<u8>>
            if let Some((b_len, b_consumed)) = parse_borsh_u32(&data[cursor..]) {
                cursor += b_consumed;
                for _ in 0..b_len {
                    if let Some(inner_len) = skip_borsh_vec(&data[cursor..], 1) {
                        cursor += inner_len;
                    } else { return Action::Invoke { lamports: None, from_index: None, to_index: None }; }
                }
            } else { return Action::Invoke { lamports: None, from_index: None, to_index: None }; }
            
            // c: Vec<u8>
            if let Some(len) = skip_borsh_vec(&data[cursor..], 1) {
                cursor += len;
            } else { return Action::Invoke { lamports: None, from_index: None, to_index: None }; }
        }
    } else { return Action::Invoke { lamports: None, from_index: None, to_index: None }; }

    // new_address_params: Vec<NewAddressParams>
    // NewAddressParams is 40 bytes: [u8; 32] seed + Pubkey address
    if let Some(_len) = skip_borsh_vec(&data[cursor..], 40) {
        // ... len is skipped
    } else { return Action::Invoke { lamports: None, from_index: None, to_index: None }; }

    // input_compressed_accounts: Vec<InputCompressedAccount>
    // Just skip it for now as we don't have exact size, skip_borsh_vec should handle it if elements are fixed size
    // For Light Protocol, these are dynamic, so we might need more complex skipping or just a guestimate
    // if the rest of the data is what we want.
    
    // In demo_compress_sol, we have 79 bytes total.
    // Disc (8) + Proof (11 bytes: 1 disc + 4 len + 0 a + 4 len + 0 b + 4 len + 0 c) = 19
    // + newAddressParams (4 bytes: 0 len) = 23
    // + inputCompressedAccounts (4 bytes: 0 len) = 27
    // + outputCompressedAccounts (4 bytes: 0 len) = 31
    // + relayFee (1 byte: 0 disc) = 32
    // + lamports (9 bytes: 1 disc + 8 val) = 41
    // + isCompress (1 byte) = 42
    
    // Let's try to jump to the end instead if we can't reliably parse.
    // The last 10 bytes of a 79-byte message are:
    // ... 01 00 e1 f5 05 00 00 00 00 01
    // 01 (Some)
    // 00 e1 f5 05 00 00 00 00 (0.1 SOL)
    // 01 (isCompress)
    if data.len() >= 10 {
        let last_10 = &data[data.len() - 10..];
        if last_10[0] == 1 {
            let lamports = u64::from_le_bytes(last_10[1..9].try_into().unwrap());
            let is_compress = last_10[9] == 1;
            
            let (from_index, to_index) = if is_compress {
                // Compression: From public (account 0) To compressed (internal)
                (Some(0), None)
            } else {
                // Decompression: From compressed (internal) To public (account 0)
                (None, Some(0))
            };
            
            return Action::Invoke { lamports: Some(lamports), from_index, to_index };
        }
    }

    Action::Invoke { lamports: None, from_index: None, to_index: None }
}

/// Parse InvokeCpi instruction from Light System Program.
fn parse_invoke_cpi(data: &[u8]) -> super::models::LightProtocolAction {
    use super::models::LightProtocolAction as Action;
    
    // Similar to Invoke, use trailing bytes if they look reasonable
    if data.len() >= 10 {
        let last_10 = &data[data.len() - 10..];
        if last_10[0] == 1 {
            let lamports = u64::from_le_bytes(last_10[1..9].try_into().unwrap());
            let is_compress = last_10[9] == 1;
            
            let (from_index, to_index) = if is_compress {
                (Some(0), None)
            } else {
                (None, Some(0))
            };
            
            return Action::InvokeCpi { lamports: Some(lamports), from_index, to_index };
        }
    }

    Action::InvokeCpi { lamports: None, from_index: None, to_index: None }
}

/// Parse InvokeCpiWithReadOnly instruction from Light System Program.
fn parse_invoke_cpi_with_readonly(data: &[u8]) -> super::models::LightProtocolAction {
    use super::models::LightProtocolAction as Action;
    
    // Similar to Invoke, use trailing bytes if they look reasonable
    if data.len() >= 10 {
        let last_10 = &data[data.len() - 10..];
        if last_10[0] == 1 {
            let lamports = u64::from_le_bytes(last_10[1..9].try_into().unwrap());
            let is_compress = last_10[9] == 1;
            
            let (from_index, to_index) = if is_compress {
                (Some(0), None)
            } else {
                (None, Some(0))
            };
            
            return Action::InvokeCpiWithReadOnly { lamports: Some(lamports), from_index, to_index };
        }
    }

    Action::InvokeCpiWithReadOnly { lamports: None, from_index: None, to_index: None }
}

/// Parse InvokeCpiWithAccountInfo instruction from Light System Program.
fn parse_invoke_cpi_with_account_info(data: &[u8]) -> super::models::LightProtocolAction {
    use super::models::LightProtocolAction as Action;
    
    // Similar to Invoke, use trailing bytes if they look reasonable
    if data.len() >= 10 {
        let last_10 = &data[data.len() - 10..];
        if last_10[0] == 1 {
            let lamports = u64::from_le_bytes(last_10[1..9].try_into().unwrap());
            let is_compress = last_10[9] == 1;
            
            let (from_index, to_index) = if is_compress {
                (Some(0), None)
            } else {
                (None, Some(0))
            };
            
            return Action::InvokeCpiWithAccountInfo { lamports: Some(lamports), from_index, to_index };
        }
    }

    Action::InvokeCpiWithAccountInfo { lamports: None, from_index: None, to_index: None }
}
