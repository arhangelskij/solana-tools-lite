
/// Unit tests for Borsh parsing utilities.
/// 
/// These tests verify that all parsing functions correctly handle valid data,
/// edge cases, and malformed input without panicking.

use crate::analysis::light_protocol::decoder;

// ============================================================================
// Tests for parse_borsh_u32
// ============================================================================

#[test]
fn test_parse_borsh_u32_valid() {
    // Test parsing a valid u32 value
    let data = [0x01, 0x00, 0x00, 0x00, 0xFF];
    let result = decoder::decode_borsh_u32(&data);
    assert_eq!(result, Some((1u32, 4)));
}

#[test]
fn test_parse_borsh_u32_max_value() {
    // Test parsing maximum u32 value
    let data = [0xFF, 0xFF, 0xFF, 0xFF, 0x00];
    let result = decoder::decode_borsh_u32(&data);
    assert_eq!(result, Some((u32::MAX, 4)));
}

#[test]
fn test_parse_borsh_u32_zero() {
    // Test parsing zero
    let data = [0x00, 0x00, 0x00, 0x00];
    let result = decoder::decode_borsh_u32(&data);
    assert_eq!(result, Some((0u32, 4)));
}

#[test]
fn test_parse_borsh_u32_insufficient_data() {
    // Test with data shorter than 4 bytes
    let data = [0x01, 0x00, 0x00];
    let result = decoder::decode_borsh_u32(&data);
    assert_eq!(result, None);
}

#[test]
fn test_parse_borsh_u32_empty_data() {
    // Test with empty data
    let data: [u8; 0] = [];
    let result = decoder::decode_borsh_u32(&data);
    assert_eq!(result, None);
}

#[test]
fn test_parse_borsh_u32_little_endian() {
    // Test little-endian encoding: 0x04030201 = 0x01020304 in little-endian
    let data = [0x04, 0x03, 0x02, 0x01];
    let result = decoder::decode_borsh_u32(&data);
    assert_eq!(result, Some((0x01020304u32, 4)));
}

// ============================================================================
// Tests for parse_borsh_u64
// ============================================================================

#[test]
fn test_parse_borsh_u64_valid() {
    // Test parsing a valid u64 value
    let data = [0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF];
    let result = decoder::decode_borsh_u64(&data);
    assert_eq!(result, Some((1u64, 8)));
}

#[test]
fn test_parse_borsh_u64_max_value() {
    // Test parsing maximum u64 value
    let data = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    let result = decoder::decode_borsh_u64(&data);
    assert_eq!(result, Some((u64::MAX, 8)));
}

#[test]
fn test_parse_borsh_u64_zero() {
    // Test parsing zero
    let data = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let result = decoder::decode_borsh_u64(&data);
    assert_eq!(result, Some((0u64, 8)));
}

#[test]
fn test_parse_borsh_u64_insufficient_data() {
    // Test with data shorter than 8 bytes
    let data = [0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let result = decoder::decode_borsh_u64(&data);
    assert_eq!(result, None);
}

#[test]
fn test_parse_borsh_u64_empty_data() {
    // Test with empty data
    let data: [u8; 0] = [];
    let result = decoder::decode_borsh_u64(&data);
    assert_eq!(result, None);
}

#[test]
fn test_parse_borsh_u64_little_endian() {
    // Test little-endian encoding
    let data = [0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01];
    let result = decoder::decode_borsh_u64(&data);
    assert_eq!(result, Some((0x0102030405060708u64, 8)));
}

// ============================================================================
// Tests for parse_borsh_vec_u64
// ============================================================================

#[test]
fn test_parse_borsh_vec_u64_empty() {
    // Test parsing empty vector (length = 0)
    let data = [0x00, 0x00, 0x00, 0x00, 0xFF];
    let result = decoder::decode_borsh_vec_u64(&data);
    assert_eq!(result, Some((vec![], 4)));
}

#[test]
fn test_parse_borsh_vec_u64_single_element() {
    // Test parsing vector with one u64 element
    // Length: 1 (4 bytes) + Value: 42 (8 bytes) = 12 bytes total
    let data = [
        0x01, 0x00, 0x00, 0x00,                         // length = 1
        0x2A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // value = 42
        0xFF,
    ];
    let result = decoder::decode_borsh_vec_u64(&data);
    assert_eq!(result, Some((vec![42u64], 12)));
}

#[test]
fn test_parse_borsh_vec_u64_multiple_elements() {
    // Test parsing vector with multiple u64 elements
    let data = [
        0x03, 0x00, 0x00, 0x00,                         // length = 3
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // value = 1
        0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // value = 2
        0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // value = 3
    ];
    let result = decoder::decode_borsh_vec_u64(&data);
    assert_eq!(result, Some((vec![1u64, 2u64, 3u64], 28)));
}

#[test]
fn test_parse_borsh_vec_u64_insufficient_data_for_length() {
    // Test with data too short for length prefix
    let data = [0x01, 0x00, 0x00];
    let result = decoder::decode_borsh_vec_u64(&data);
    assert_eq!(result, None);
}

#[test]
fn test_parse_borsh_vec_u64_insufficient_data_for_elements() {
    // Test with length prefix but insufficient data for elements
    let data = [
        0x02, 0x00, 0x00, 0x00,                         // length = 2
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // first element
        // missing second element
    ];
    let result = decoder::decode_borsh_vec_u64(&data);
    assert_eq!(result, None);
}

#[test]
fn test_parse_borsh_vec_u64_large_length() {
    // Test with very large length that would overflow
    let data = [0xFF, 0xFF, 0xFF, 0xFF]; // length = u32::MAX
    let result = decoder::decode_borsh_vec_u64(&data);
    assert_eq!(result, None); // Should fail due to insufficient data
}

// ============================================================================
// Tests for parse_borsh_option_vec_u64
// ============================================================================

#[test]
fn test_parse_borsh_option_vec_u64_none() {
    // Test parsing None variant (discriminator = 0)
    let data = [0x00, 0xFF];
    let result = decoder::decode_borsh_option_vec_u64(&data);
    assert_eq!(result, Some((None, 1)));
}

#[test]
fn test_parse_borsh_option_vec_u64_some_empty() {
    // Test parsing Some with empty vector
    let data = [
        0x01,                           // discriminator = 1 (Some)
        0x00, 0x00, 0x00, 0x00,         // length = 0
        0xFF,
    ];
    let result = decoder::decode_borsh_option_vec_u64(&data);
    assert_eq!(result, Some((Some(vec![]), 5)));
}

#[test]
fn test_parse_borsh_option_vec_u64_some_with_data() {
    // Test parsing Some with vector containing elements
    let data = [
        0x01,                                           // discriminator = 1 (Some)
        0x02, 0x00, 0x00, 0x00,                         // length = 2
        0x0A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // value = 10
        0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // value = 20
    ];
    let result = decoder::decode_borsh_option_vec_u64(&data);
    assert_eq!(result, Some((Some(vec![10u64, 20u64]), 21)));
}

#[test]
fn test_parse_borsh_option_vec_u64_insufficient_data_discriminator() {
    // Test with empty data
    let data: [u8; 0] = [];
    let result = decoder::decode_borsh_option_vec_u64(&data);
    assert_eq!(result, None);
}

#[test]
fn test_parse_borsh_option_vec_u64_insufficient_data_some() {
    // Test with Some discriminator but insufficient data for vector
    let data = [0x01, 0x01, 0x00]; // discriminator + partial length
    let result = decoder::decode_borsh_option_vec_u64(&data);
    assert_eq!(result, None);
}

#[test]
fn test_parse_borsh_option_vec_u64_invalid_discriminator() {
    // Test with invalid discriminator (not 0 or 1)
    let data = [0x02, 0xFF];
    let result = decoder::decode_borsh_option_vec_u64(&data);
    assert_eq!(result, None);
}

// ============================================================================
// Tests for skip_bytes
// ============================================================================

#[test]
fn test_skip_bytes_valid() {
    let data = [0x01, 0x02, 0x03, 0x04, 0x05];
    let result = decoder::skip_bytes(&data, 3);
    assert_eq!(result, Some(3));
}

#[test]
fn test_skip_bytes_exact_length() {
    let data = [0x01, 0x02, 0x03];
    let result = decoder::skip_bytes(&data, 3);
    assert_eq!(result, Some(3));
}

#[test]
fn test_skip_bytes_insufficient_data() {
    let data = [0x01, 0x02];
    let result = decoder::skip_bytes(&data, 3);
    assert_eq!(result, None);
}

#[test]
fn test_skip_bytes_zero() {
    let data = [0x01, 0x02, 0x03];
    let result = decoder::skip_bytes(&data, 0);
    assert_eq!(result, Some(0));
}

#[test]
fn test_skip_bytes_empty_data() {
    let data: [u8; 0] = [];
    let result = decoder::skip_bytes(&data, 1);
    assert_eq!(result, None);
}

// ============================================================================
// Tests for skip_borsh_vec
// ============================================================================

#[test]
fn test_skip_borsh_vec_empty() {
    // Empty vector: length = 0, element_size = 8
    let data = [0x00, 0x00, 0x00, 0x00, 0xFF];
    let result = decoder::skip_borsh_vec(&data, 8);
    assert_eq!(result, Some(4)); // Only length prefix
}

#[test]
fn test_skip_borsh_vec_with_elements() {
    // Vector with 2 elements of size 8 bytes each
    let data = [
        0x02, 0x00, 0x00, 0x00,                         // length = 2
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // element 1
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // element 2
        0xFF,
    ];
    let result = decoder::skip_borsh_vec(&data, 8);
    assert_eq!(result, Some(20)); // 4 (length) + 16 (2 * 8)
}

#[test]
fn test_skip_borsh_vec_insufficient_data() {
    // Vector claims 2 elements but data is too short
    let data = [
        0x02, 0x00, 0x00, 0x00,                         // length = 2
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // element 1 only
    ];
    let result = decoder::skip_borsh_vec(&data, 8);
    assert_eq!(result, None);
}

#[test]
fn test_skip_borsh_vec_insufficient_data_for_length() {
    // Data too short for length prefix
    let data = [0x01, 0x00, 0x00];
    let result = decoder::skip_borsh_vec(&data, 8);
    assert_eq!(result, None);
}

#[test]
fn test_skip_borsh_vec_large_element_size() {
    // Vector with large element size
    let data = [
        0x01, 0x00, 0x00, 0x00,                         // length = 1
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 8 bytes
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 8 more bytes
        0xFF,
    ];
    let result = decoder::skip_borsh_vec(&data, 16); // element_size = 16
    assert_eq!(result, Some(20)); // 4 (length) + 16 (1 * 16)
}


// ============================================================================
// Tests for parse_light_instruction - Invoke
// ============================================================================

#[test]
fn test_parse_invoke_compress_sol() {
    use crate::analysis::light_protocol::parsing::parse_light_instruction;
    use solana_tools_lite::models::pubkey_base58::PubkeyBase58;
    use crate::analysis::light_protocol::models::LightProtocolAction;
    
    // Real data from demo_compress_sol.b64
    // Discriminator: 1a10a90715caf219 (Invoke)
    // Action: 43 (Compress SOL)
    // Amount: 00e1f505 (little-endian) = 100_000_000 lamports = 0.1 SOL
    let data: &[u8] = &[
        0x1a, 0x10, 0xa9, 0x07, 0x15, 0xca, 0xf2, 0x19, // discriminator
        0x43, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // action + padding
        0x01, 0x00, 0x00, 0x00, // proof length
        0x36, 0x41, 0x33, 0xbf, 0x5f, 0xab, 0x28, 0x6d, 0x4a, 0x72, 0x93, 0xfc, 0x3c, 0x2b, 0x59, 0x5d,
        0x9f, 0xa1, 0x16, 0x4d, 0x24, 0x8c, 0xf6, 0xe2, 0x88, 0xdd, 0x5a, 0x1f, 0x7b, 0xdb, 0x09, 0x41,
        0x00, 0xe1, 0xf5, 0x05, 0x00, 0x00, 0x00, 0x00, // amount: 100_000_000 lamports
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x01, 0x00, 0xe1, 0xf5, 0x05, 0x00, 0x00, 0x00, 0x00, 0x01,
    ];
    
    let program_id = PubkeyBase58::try_from("SySTEM1eSU2p4BGQfQpimFEWWSC1XDFeun3Nqzz3rT7").unwrap();
    let action = parse_light_instruction(&program_id, data);
    
    match action {
        LightProtocolAction::Invoke { lamports, from_index, to_index } => {
            // 0.1 SOL = 100_000_000 lamports
            assert_eq!(lamports, Some(100_000_000), "Should parse 0.1 SOL (100_000_000 lamports)");
            assert_eq!(from_index, Some(0));
            assert_eq!(to_index, None);
        }
        _ => panic!("Expected Invoke action, got {:?}", action),
    }
}

#[test]
fn test_parse_invoke_cpi_variants() {
    use crate::analysis::light_protocol::parsing::parse_light_instruction;
    use solana_tools_lite::models::pubkey_base58::PubkeyBase58;
    use crate::analysis::light_protocol::models::LightProtocolAction;
    use crate::analysis::light_protocol::constants;

    let program_id = PubkeyBase58::try_from(constants::LIGHT_SYSTEM_PROGRAM_ID).unwrap();

    // Helper to create test data with trailing bytes
    let create_data = |disc: [u8; 8], lamports: u64, is_compress: bool| {
        let mut data = vec![0u8; 50]; // Mock some data
        data[0..8].copy_from_slice(&disc);
        // Add trailing bytes: [Option disc, u64 lamports, bool is_compress]
        let mut trailing = vec![1u8]; // Some
        trailing.extend_from_slice(&lamports.to_le_bytes());
        trailing.push(if is_compress { 1 } else { 0 });
        data.extend_from_slice(&trailing);
        data
    };

    // 1. InvokeCpi
    let data_cpi = create_data(constants::DISCRIMINATOR_INVOKE_CPI, 500_000, true);
    let action_cpi = parse_light_instruction(&program_id, &data_cpi);
    match action_cpi {
        LightProtocolAction::InvokeCpi { lamports, from_index, to_index } => {
            assert_eq!(lamports, Some(500_000));
            assert_eq!(from_index, Some(0));
            assert_eq!(to_index, None);
        }
        _ => panic!("Expected InvokeCpi, got {:?}", action_cpi),
    }

    // 2. InvokeCpiWithReadOnly
    let data_ro = create_data(constants::DISCRIMINATOR_INVOKE_CPI_WITH_READ_ONLY, 1_000_000, false);
    let action_ro = parse_light_instruction(&program_id, &data_ro);
    match action_ro {
        LightProtocolAction::InvokeCpiWithReadOnly { lamports, from_index, to_index } => {
            assert_eq!(lamports, Some(1_000_000));
            assert_eq!(from_index, None);
            assert_eq!(to_index, Some(0));
        }
        _ => panic!("Expected InvokeCpiWithReadOnly, got {:?}", action_ro),
    }

    // 3. InvokeCpiWithAccountInfo
    let data_ai = create_data(constants::DISCRIMINATOR_INVOKE_CPI_WITH_ACCOUNT_INFO, 2_000_000, true);
    let action_ai = parse_light_instruction(&program_id, &data_ai);
    match action_ai {
        LightProtocolAction::InvokeCpiWithAccountInfo { lamports, from_index, to_index } => {
            assert_eq!(lamports, Some(2_000_000));
            assert_eq!(from_index, Some(0));
            assert_eq!(to_index, None);
        }
        _ => panic!("Expected InvokeCpiWithAccountInfo, got {:?}", action_ai),
    }
}
