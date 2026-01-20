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
pub fn parse_u64_at_offset(data: &[u8], offset: usize) -> Option<u64> {
    if data.len() < offset + U64_SIZE {
        return None;
    }
    
    data[offset..offset + U64_SIZE]
        .try_into()
        .ok()
        .map(u64::from_le_bytes)
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
pub fn extract_discriminator(data: &[u8]) -> [u8; DISCRIMINATOR_SIZE] {
    if data.len() < DISCRIMINATOR_SIZE {
        return [0u8; DISCRIMINATOR_SIZE];
    }
    
    data[0..DISCRIMINATOR_SIZE]
        .try_into()
        .unwrap_or([0u8; DISCRIMINATOR_SIZE])
}
