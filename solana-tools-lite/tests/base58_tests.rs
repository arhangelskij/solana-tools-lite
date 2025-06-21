use solana_tools_lite::crypto::base58;

#[cfg(test)]
mod tests {
    use super::*;

    /// Basic test: encode and decode "hello world" string
    #[test]
    fn test_base58_encode_decode() {
        let data = b"hello world";
        let encoded = base58::encode(data);
        let decoded = base58::decode(&encoded).unwrap();
        assert_eq!(decoded, data);
    }

    /// Checks that decoding an invalid base58 string returns an error
    #[test]
    fn test_base58_invalid_decode() {
        let invalid = "!!!@@@###";
        assert!(base58::decode(invalid).is_err());
    }
    
    /// Ensures encoding and decoding of empty data is handled correctly
    #[test]
    fn test_empty_data() {
        let data = b"";
        let encoded = base58::encode(data);
        let decoded = base58::decode(&encoded).unwrap();
        assert_eq!(decoded, data);
    }

    /// Tests encoding and decoding of a single byte
    #[test]
    fn test_single_byte() {
        let data = b"A";
        let encoded = base58::encode(data);
        let decoded = base58::decode(&encoded).unwrap();
        assert_eq!(decoded, data);
    }

    /// Checks base58 handling for arbitrary binary data (including non-ASCII)
    #[test]
    fn test_binary_data() {
        let data = vec![0u8, 255u8, 128u8, 42u8];
        let encoded = base58::encode(&data);
        let decoded = base58::decode(&encoded).unwrap();
        assert_eq!(decoded, data);
    }

    /// Verifies that leading zeros are preserved after encode/decode roundtrip
    #[test]
    fn test_leading_zeros() {
        let data = vec![0u8, 0u8, 1u8, 2u8];
        let encoded = base58::encode(&data);
        let decoded = base58::decode(&encoded).unwrap();
        assert_eq!(decoded, data);
    }
}