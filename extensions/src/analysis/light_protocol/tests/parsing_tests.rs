use crate::analysis::light_protocol::parsing::*;

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
fn test_parse_u16_at_offset_success() {
    let data = [0, 0, 0x10, 0x27];
    let result = parse_u16_at_offset(&data, 2);
    assert_eq!(result, Some(10000));
}

#[test]
fn test_parse_u16_at_offset_too_short() {
    let data = [1, 2];
    let result = parse_u16_at_offset(&data, 1);
    assert_eq!(result, None);
}

#[test]
fn test_parse_u16_at_offset_out_of_bounds() {
    let data = [1, 2, 3];
    let result = parse_u16_at_offset(&data, 2);
    assert_eq!(result, None);
}

#[test]
fn test_extract_discriminator_u8_success() {
    let data = [3, 4, 5, 6];
    let result = extract_discriminator_u8(&data);
    assert_eq!(result, Some(3));
}

#[test]
fn test_extract_discriminator_u8_empty() {
    let data = [];
    let result = extract_discriminator_u8(&data);
    assert_eq!(result, None);
}

#[test]
fn test_extract_discriminator_u64_success() {
    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = extract_discriminator_u64(&data);
    assert_eq!(result, Some([1, 2, 3, 4, 5, 6, 7, 8]));
}

#[test]
fn test_extract_discriminator_u64_exact_length() {
    let data = [1, 2, 3, 4, 5, 6, 7, 8];
    let result = extract_discriminator_u64(&data);
    assert_eq!(result, Some([1, 2, 3, 4, 5, 6, 7, 8]));
}

#[test]
fn test_extract_discriminator_u64_too_short() {
    let data = [1, 2, 3];
    let result = extract_discriminator_u64(&data);
    assert_eq!(result, None);
}

#[test]
fn test_extract_discriminator_u64_empty() {
    let data = [];
    let result = extract_discriminator_u64(&data);
    assert_eq!(result, None);
}
