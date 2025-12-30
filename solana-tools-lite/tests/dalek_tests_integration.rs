use ed25519_dalek::Signature;
use std::convert::TryFrom;

// -------------------------------
// Section: ed25519 signature parsing
// -------------------------------

// Parse a valid 64-byte ed25519 signature
#[test]
fn test_signature_from_bytes() {
    let bytes = [1u8; 64];
    let sig = Signature::try_from(&bytes[..]).expect("valid signature bytes must parse");
    assert_eq!(sig.to_bytes(), bytes);
}

// Error on invalid signature length (not 64 bytes)
#[test]
fn test_signature_from_bytes_invalid_length() {
    let bytes = [1u8; 63];
    let result = Signature::try_from(bytes.as_slice());
    assert!(result.is_err());
}

// Zeroed signature bytes still produce a valid Signature struct
#[test]
fn test_empty_signature() {
    let bytes = [0u8; 64];
    let sig = Signature::try_from(&bytes[..])
        .expect("zeroed signature bytes should still parse as a struct");
    assert_eq!(sig.to_bytes(), bytes);
}
