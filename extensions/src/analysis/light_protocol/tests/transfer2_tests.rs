/// Integration tests for Transfer2 instruction parsing.
/// 
/// These tests verify that Transfer2 instructions are correctly parsed
/// to extract in_lamports and out_lamports values.
/// 
/// Tests verify that Transfer2 instructions are correctly parsed
/// to extract in_lamports, out_lamports, and aggregated amounts.

use crate::analysis::light_protocol::parse_light_instruction;
use crate::analysis::light_protocol::models::LightProtocolAction;
use crate::analysis::light_protocol::constants;
use solana_tools_lite::models::pubkey_base58::PubkeyBase58;

/// Helper function to create a PubkeyBase58 for the Compressed Token Program
fn compressed_token_program() -> PubkeyBase58 {
    // This is the actual Compressed Token Program ID
    PubkeyBase58::try_from(constants::COMPRESSED_TOKEN_PROGRAM_ID).unwrap()
}

#[test]
fn test_parse_transfer2_no_lamports() {
    // Test Transfer2 with None for both lamports fields
    let mut data = Vec::new();
    
    // Discriminator
    data.push(constants::DISCRIMINATOR_TRANSFER2);
    
    // compressions: None
    data.push(0x00);
    
    // in_token_data: empty vector
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
    
    // out_token_data: empty vector
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
    
    // in_lamports: None
    data.push(0x00);
    
    // out_lamports: None
    data.push(0x00);
    
    let program_id = compressed_token_program();
    let action = parse_light_instruction(&program_id, &data);
    
    match action {
        LightProtocolAction::Transfer2 { in_lamports, out_lamports, amount } => {
            assert_eq!(in_lamports, None, "in_lamports should be None");
            assert_eq!(out_lamports, None, "out_lamports should be None");
            assert_eq!(amount, None, "amount should be None");
        }
        _ => panic!("Expected Transfer2 action, got {:?}", action),
    }
}

#[test]
fn test_parse_transfer2_malformed_truncated() {
    // Test Transfer2 with truncated data (should not panic)
    let mut data = Vec::new();
    
    // Discriminator
    data.push(constants::DISCRIMINATOR_TRANSFER2);
    
    // compressions: None
    data.push(0x00);
    
    // in_token_data: empty vector
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
    
    // out_token_data: empty vector
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
    
    // Truncated - missing lamports fields
    
    let program_id = compressed_token_program();
    let action = parse_light_instruction(&program_id, &data);
    
    // Should return Transfer2 with None values, not panic
    match action {
        LightProtocolAction::Transfer2 { in_lamports, out_lamports, amount } => {
            assert_eq!(in_lamports, None, "in_lamports should be None on parse failure");
            assert_eq!(out_lamports, None, "out_lamports should be None on parse failure");
            assert_eq!(amount, None, "amount should be None");
        }
        _ => panic!("Expected Transfer2 action, got {:?}", action),
    }
}

#[test]
fn test_parse_transfer2_complex() {
    // Test Transfer2 with non-empty vectors and lamports
    let mut data = Vec::new();
    
    // Discriminator
    data.push(constants::DISCRIMINATOR_TRANSFER2);
    
    // 7 fixed bytes (bools, u8s, u16)
    data.extend_from_slice(&[0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x00]);
    
    // cpi_context: Some
    data.push(0x01); // Some discriminator
    data.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]); // programIndex
    data.push(0x00); // AccountContext: None
    
    // compressions: Some(Vec)
    data.push(0x01); // Some discriminator
    data.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]); // Vec length 1
    // Compression struct (31 bytes)
    data.push(0x00); // mode
    data.extend_from_slice(&100u64.to_le_bytes()); // amount
    data.extend_from_slice(&[0u8; 22]); // the rest (4*5 + 1 + 1)
    
    // proof: None
    data.push(0x00);
    
    // in_token_data: Vec length 1
    data.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]);
    // MultiInputTokenDataWithContext:
    data.extend_from_slice(&200u64.to_le_bytes()); // amount
    data.push(0x00); // hasDelegate: false
    data.push(0x00); // option discriminator: None
    data.extend_from_slice(&[0u8; 4+4+1]); // tokenIdx + poolIdx + bump
    
    // out_token_data: Vec length 1
    data.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]);
    // MultiTokenTransferOutputData (21 bytes):
    data.extend_from_slice(&300u64.to_le_bytes()); // amount
    data.extend_from_slice(&[0u8; 13]); // recipient + tokenIdx + poolIdx + bump
    
    // in_lamports: Some(Vec[1000])
    data.push(0x01); // Some
    data.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]); // len 1
    data.extend_from_slice(&1000u64.to_le_bytes()); // val
    
    // out_lamports: Some(Vec[2000])
    data.push(0x01); // Some
    data.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]); // len 1
    data.extend_from_slice(&2000u64.to_le_bytes()); // val
    
    let program_id = compressed_token_program();
    let action = parse_light_instruction(&program_id, &data);
    
    match action {
        LightProtocolAction::Transfer2 { in_lamports, out_lamports, amount } => {
            assert_eq!(in_lamports, Some(1000));
            assert_eq!(out_lamports, Some(2000));
            assert_eq!(amount, Some(600)); // 100 + 200 + 300
        }
        _ => panic!("Expected Transfer2 action, got {:?}", action),
    }
}

#[test]
fn test_parse_batch_compress_complex() {
    let mut data = Vec::new();
    
    // 8-byte discriminator
    data.extend_from_slice(&constants::DISCRIMINATOR_BATCH_COMPRESS);
    
    // pubkeys: Vec[32 bytes]
    data.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]); // len 1
    data.extend_from_slice(&[0u8; 32]); // 32 bytes of 0s
    
    // amounts: Some(Vec[400, 500])
    data.push(0x01); // Some
    data.extend_from_slice(&[0x02, 0x00, 0x00, 0x00]); // len 2
    data.extend_from_slice(&400u64.to_le_bytes());
    data.extend_from_slice(&500u64.to_le_bytes());
    
    // lamports: None
    data.push(0x00);
    
    // amount: Some(900)
    data.push(0x01);
    data.extend_from_slice(&900u64.to_le_bytes());
    
    let program_id = PubkeyBase58::try_from(constants::COMPRESSED_TOKEN_PROGRAM_ID).unwrap();
    let action = parse_light_instruction(&program_id, &data);
    
    match action {
        LightProtocolAction::BatchCompress { amount } => {
            assert_eq!(amount, Some(900)); // Prioritized or summed
        }
        _ => panic!("Expected BatchCompress action, got {:?}", action),
    }
}
