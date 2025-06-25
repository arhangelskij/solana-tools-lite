//! Integration tests for base58 encoding/decoding
//! These ensure that our use of the `bs58` crate works as expected
//! and demonstrate example usage.#[cfg(test)]

mod integration_tests {
    use bs58;

    #[test]
    fn test_encode_decode_roundtrip() {
        // Basic string data
        let text_data = b"hello world";
        let encoded = bs58::encode(text_data).into_vec();
        let decoded = bs58::decode(&encoded).into_vec().unwrap();
        assert_eq!(decoded, text_data);

        // Binary data with edge values
        let binary_data = vec![0u8, 255u8, 128u8, 42u8];
        let encoded = bs58::encode(&binary_data).into_vec();
        let decoded = bs58::decode(&encoded).into_vec().unwrap();
        assert_eq!(decoded, binary_data);

        // Leading zeros preservation
        let zero_data = vec![0u8, 0u8, 1u8, 2u8];
        let encoded = bs58::encode(&zero_data).into_vec();
        let decoded = bs58::decode(&encoded).into_vec().unwrap();
        assert_eq!(decoded, zero_data);
    }

    #[test]
    fn test_edge_cases() {
        // Empty data
        let empty = b"";
        let encoded = bs58::encode(empty).into_vec();
        let decoded = bs58::decode(&encoded).into_vec().unwrap();
        assert_eq!(decoded, empty);

        // Single byte
        let single = b"A";
        let encoded = bs58::encode(single).into_vec();
        let decoded = bs58::decode(&encoded).into_vec().unwrap();
        assert_eq!(decoded, single);
    }

    #[test]
    fn test_invalid_input_handling() {
        // Invalid base58 characters should return error
        let invalid = "!!!@@@###";
        assert!(bs58::decode(invalid).into_vec().is_err());
    }
}