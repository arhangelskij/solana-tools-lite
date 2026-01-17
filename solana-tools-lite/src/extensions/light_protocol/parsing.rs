/// Parsing utilities for Light Protocol instruction data.
/// 
/// This module provides safe, reusable functions for parsing binary data
/// from Light Protocol instructions. All functions use defensive programming
/// practices to avoid panics and handle malformed data gracefully.
use super::constants::{DISCRIMINATOR_SIZE, U64_SIZE};

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
/// 
/// # Examples
/// 
/// ```rust
/// use solana_tools_lite::extensions::light_protocol::parsing::parse_u64_at_offset;
/// 
/// let data = [0, 0, 0, 0, 0, 0, 0, 0, 0x10, 0x27, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
/// let value = parse_u64_at_offset(&data, 8);
/// assert_eq!(value, Some(10000));
/// ```
pub fn parse_u64_at_offset(data: &[u8], offset: usize) -> Option<u64> {
    if data.len() < offset + U64_SIZE {
        return None;
    }
    
    data[offset..offset + U64_SIZE]
        .try_into()
        .ok()
        .map(u64::from_le_bytes)
}

/// Parse a u64 amount/lamports value from Light Protocol instruction data.
/// 
/// This is a convenience function that parses a u64 value immediately after
/// the 8-byte discriminator in Light Protocol instructions.
/// 
/// # Arguments
/// 
/// * `data` - The complete instruction data including discriminator
/// 
/// # Returns
/// 
/// `Some(amount)` if the instruction contains valid amount data,
/// `None` if the instruction is too short or parsing fails.
/// 
/// # Examples
/// 
/// ```rust
/// use solana_tools_lite::extensions::light_protocol::parsing::parse_amount_from_instruction;
/// 
/// // Instruction with discriminator + amount
/// let mut data = vec![1, 2, 3, 4, 5, 6, 7, 8]; // discriminator
/// data.extend_from_slice(&1000u64.to_le_bytes()); // amount
/// 
/// let amount = parse_amount_from_instruction(&data);
/// assert_eq!(amount, Some(1000));
/// ```
pub fn parse_amount_from_instruction(data: &[u8]) -> Option<u64> {
    parse_u64_at_offset(data, DISCRIMINATOR_SIZE)
}

/// Validate that instruction data has the minimum required length.
/// 
/// Light Protocol instructions must have at least an 8-byte discriminator.
/// This function provides a consistent way to validate instruction length.
/// 
/// # Arguments
/// 
/// * `data` - The instruction data to validate
/// * `min_length` - The minimum required length in bytes
/// 
/// # Returns
/// 
/// `true` if the data is long enough, `false` otherwise.
/// 
/// # Examples
/// 
/// ```rust
/// use solana_tools_lite::extensions::light_protocol::parsing::validate_instruction_length;
/// use solana_tools_lite::extensions::light_protocol::constants::DISCRIMINATOR_SIZE;
/// 
/// let data = [1, 2, 3, 4, 5, 6, 7, 8];
/// assert!(validate_instruction_length(&data, DISCRIMINATOR_SIZE));
/// 
/// let short_data = [1, 2, 3];
/// assert!(!validate_instruction_length(&short_data, DISCRIMINATOR_SIZE));
/// ```
pub fn validate_instruction_length(data: &[u8], min_length: usize) -> bool {
    data.len() >= min_length
}

/// Extract discriminator from Light Protocol instruction data.
/// 
/// Safely extracts the 8-byte discriminator from the beginning of instruction data.
/// Returns a default discriminator if the data is too short.
/// 
/// # Arguments
/// 
/// * `data` - The instruction data
/// 
/// # Returns
/// 
/// An 8-byte array containing the discriminator, or zeros if data is too short.
/// 
/// # Examples
/// 
/// ```rust
/// use solana_tools_lite::extensions::light_protocol::parsing::extract_discriminator;
/// 
/// let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
/// let discriminator = extract_discriminator(&data);
/// assert_eq!(discriminator, [1, 2, 3, 4, 5, 6, 7, 8]);
/// 
/// let short_data = [1, 2, 3];
/// let discriminator = extract_discriminator(&short_data);
/// assert_eq!(discriminator, [0, 0, 0, 0, 0, 0, 0, 0]);
/// ```
pub fn extract_discriminator(data: &[u8]) -> [u8; DISCRIMINATOR_SIZE] {
    if data.len() < DISCRIMINATOR_SIZE {
        return [0u8; DISCRIMINATOR_SIZE];
    }
    
    data[0..DISCRIMINATOR_SIZE]
        .try_into()
        .unwrap_or([0u8; DISCRIMINATOR_SIZE])
}