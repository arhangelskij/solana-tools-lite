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
