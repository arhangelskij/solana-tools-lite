use crate::extensions::light_protocol::parsing::*;
use crate::extensions::light_protocol::constants::DISCRIMINATOR_SIZE;

#[test]
fn test_parse_u64_at_offset_success() {
    let data = [0, 0, 0, 0, 0, 0, 0, 0, 0x10, 0x27, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let result = parse_u64_at_offset(&data, 8);
    assert_eq!(result, Some(10000));
}

#[test]
fn test_parse_u64_at_offset_too_short() {
    let data = [1, 2, 3, 4, 5];
    let result = parse_u64_at_offset(&data, 0);
    assert_eq!(result, None);
}

#[test]
fn test_parse_u64_at_offset_out_of_bounds() {
    let data = [1, 2, 3, 4, 5, 6, 7, 8];
    let result = parse_u64_at_offset(&data, 5);
    assert_eq!(result, None);
}

#[test]
fn test_parse_amount_from_instruction_success() {
    let mut data = vec![1, 2, 3, 4, 5, 6, 7, 8]; // discriminator
    data.extend_from_slice(&1000u64.to_le_bytes()); // amount
    
    let result = parse_amount_from_instruction(&data);
    assert_eq!(result, Some(1000));
}

#[test]
fn test_parse_amount_from_instruction_too_short() {
    let data = [1, 2, 3, 4, 5, 6, 7, 8]; // only discriminator
    let result = parse_amount_from_instruction(&data);
    assert_eq!(result, None);
}

#[test]
fn test_validate_instruction_length() {
    assert!(validate_instruction_length(&[1, 2, 3, 4, 5, 6, 7, 8], 8));
    assert!(validate_instruction_length(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 8));
    assert!(!validate_instruction_length(&[1, 2, 3], 8));
    assert!(!validate_instruction_length(&[], 1));
}

#[test]
fn test_extract_discriminator_success() {
    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = extract_discriminator(&data);
    assert_eq!(result, [1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_extract_discriminator_exact_length() {
    let data = [1, 2, 3, 4, 5, 6, 7, 8];
    let result = extract_discriminator(&data);
    assert_eq!(result, [1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_extract_discriminator_too_short() {
    let data = [1, 2, 3];
    let result = extract_discriminator(&data);
    assert_eq!(result, [0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_extract_discriminator_empty() {
    let data = [];
    let result = extract_discriminator(&data);
    assert_eq!(result, [0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_parse_u64_at_offset_zero_value() {
    let data = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let result = parse_u64_at_offset(&data, 8);
    assert_eq!(result, Some(0));
}

#[test]
fn test_parse_u64_at_offset_max_value() {
    let mut data = vec![0; 8];
    data.extend_from_slice(&u64::MAX.to_le_bytes());
    let result = parse_u64_at_offset(&data, 8);
    assert_eq!(result, Some(u64::MAX));
}

#[test]
fn test_validate_instruction_length_edge_cases() {
    // Exact minimum length
    assert!(validate_instruction_length(&[1; DISCRIMINATOR_SIZE], DISCRIMINATOR_SIZE));
    
    // One byte short
    assert!(!validate_instruction_length(&[1; DISCRIMINATOR_SIZE - 1], DISCRIMINATOR_SIZE));
    
    // Zero length requirement
    assert!(validate_instruction_length(&[], 0));
    assert!(validate_instruction_length(&[1], 0));
}