use solana_tools_lite::crypto::ed25519;
use solana_tools_lite::handlers::verify;

#[cfg(test)]
mod tests {
    use super::*;
    use bs58;

    /// A valid signature for the given message and public key should verify successfully.
    #[test]
    fn test_valid_signature_verification() {
        let seed = [42u8; 64];
        let key = ed25519::keypair_from_seed(&seed).unwrap();
        let pubkey = key.verifying_key();
        let msg = "test-message";
        let sig = ed25519::sign_message(&key, msg.as_bytes());

        let sig_b58 = bs58::encode(sig.to_bytes()).into_string();
        let pubkey_b58 = bs58::encode(pubkey.to_bytes()).into_string();

        let result = verify::handle_verify(msg, &sig_b58, &pubkey_b58);
        assert!(result.is_ok());
    }

    /// An invalid signature or public key must cause verification to fail.
    #[test]
    fn test_invalid_signature_should_fail() {
        let pubkey = [1u8; 32]; // garbage pubkey
        let sig = [2u8; 64]; // garbage signature

        let pubkey_b58 = bs58::encode(pubkey).into_string();
        let sig_b58 = bs58::encode(sig).into_string();

        let result = verify::handle_verify("fake", &sig_b58, &pubkey_b58);
        assert!(result.is_err());
    }

    /// A signature generated for a different message should not verify for the original message.
    #[test]
    fn test_signature_mismatch_should_fail() {
        let seed = [1u8; 64];
        let key = ed25519::keypair_from_seed(&seed).unwrap();
        let pubkey_b58 = bs58::encode(key.verifying_key().to_bytes()).into_string();

        let sig = ed25519::sign_message(&key, b"other-message");
        let sig_b58 = bs58::encode(sig.to_bytes()).into_string();

        let result = verify::handle_verify("original-message", &sig_b58, &pubkey_b58);
        assert!(result.is_err());
    }

    /// Empty message should still verify when the signature matches.
    #[test]
    fn test_empty_message_valid_signature() {
        let seed = [99u8; 64];
        let key = ed25519::keypair_from_seed(&seed).unwrap();
        let sig = ed25519::sign_message(&key, b"");
        let sig_b58 = bs58::encode(sig.to_bytes()).into_string();
        let pubkey_b58 = bs58::encode(key.verifying_key().to_bytes()).into_string();

        let result = verify::handle_verify("", &sig_b58, &pubkey_b58);
        assert!(result.is_ok());
    }

    /// Very long message (~10 KB) should verify when the signature matches.
    #[test]
    fn test_very_long_message_valid_signature() {
        let seed = [100u8; 64];
        let key = ed25519::keypair_from_seed(&seed).unwrap();
        let long_msg = "A".repeat(10 * 1024); // 10 KB
        let sig = ed25519::sign_message(&key, long_msg.as_bytes());
        let sig_b58 = bs58::encode(sig.to_bytes()).into_string();
        let pubkey_b58 = bs58::encode(key.verifying_key().to_bytes()).into_string();

        let result = verify::handle_verify(&long_msg, &sig_b58, &pubkey_b58);
        assert!(result.is_ok());
    }

    /// Invalid Base58 strings (contain non‑Base58 characters) must be rejected.
    #[test]
    fn test_invalid_base58_should_fail() {
        let msg = "dummy";
        let sig_b58 = "%%notbase58%%";
        let pubkey_b58 = "%%notbase58%%";

        let result = verify::handle_verify(msg, sig_b58, pubkey_b58);
        assert!(result.is_err());
    }

}
