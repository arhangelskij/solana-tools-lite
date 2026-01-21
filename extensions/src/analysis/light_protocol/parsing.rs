/// Parsing utilities for Light Protocol instruction data.
/// 
/// This module provides safe, reusable functions for parsing binary data
/// from Light Protocol instructions. All functions use defensive programming
/// practices to avoid panics and handle malformed data gracefully.
use super::constants::{U64_SIZE, U16_SIZE};

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
