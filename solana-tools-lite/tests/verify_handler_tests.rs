#[cfg(test)]
mod tests {
    use bs58;
    use solana_tools_lite::crypto::ed25519;
    use solana_tools_lite::handlers::verify;

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

        let exit_code = verify::handle_verify(msg, &sig_b58, &pubkey_b58, false);
        assert_eq!(exit_code, 0, "should success");
    }

    /// An invalid signature or public key must cause verification to fail.
    #[test]
    fn test_invalid_signature_should_fail() {
        let pubkey = [1u8; 32]; // garbage pubkey
        let sig = [2u8; 64]; // garbage signature

        let pubkey_b58 = bs58::encode(pubkey).into_string();
        let sig_b58 = bs58::encode(sig).into_string();

        let exit_code = verify::handle_verify("fake", &sig_b58, &pubkey_b58, false);
        assert_eq!(exit_code, 1, "should fail");
    }

    /// A signature generated for a different message should not verify for the original message.
    #[test]
    fn test_signature_mismatch_should_fail() {
        let seed = [1u8; 64];
        let key = ed25519::keypair_from_seed(&seed).unwrap();
        let pubkey_b58 = bs58::encode(key.verifying_key().to_bytes()).into_string();

        let sig = ed25519::sign_message(&key, b"other-message");
        let sig_b58 = bs58::encode(sig.to_bytes()).into_string();

        let exit_code = verify::handle_verify("original-message", &sig_b58, &pubkey_b58, false);
        assert_eq!(exit_code, 1, "should fail");
    }

    /// Empty message should still verify when the signature matches.
    #[test]
    fn test_empty_message_valid_signature() {
        let seed = [99u8; 64];
        let key = ed25519::keypair_from_seed(&seed).unwrap();
        let sig = ed25519::sign_message(&key, b"");
        let sig_b58 = bs58::encode(sig.to_bytes()).into_string();
        let pubkey_b58 = bs58::encode(key.verifying_key().to_bytes()).into_string();

        let exit_code = verify::handle_verify("", &sig_b58, &pubkey_b58, false);
        assert_eq!(exit_code, 0, "should succes");
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

        let exit_code = verify::handle_verify(&long_msg, &sig_b58, &pubkey_b58, false);
        assert_eq!(exit_code, 0, "should succes");
    }

    /// Invalid Base58 strings (contain non‑Base58 characters) must be rejected.
    #[test]
    fn test_invalid_base58_should_fail() {
        let msg = "dummy";
        let sig_b58 = "%%notbase58%%";
        let pubkey_b58 = "%%notbase58%%";

        let exit_code = verify::handle_verify(msg, sig_b58, pubkey_b58, false);
        assert_eq!(exit_code, 1, "should fail");
    }

    /// A signature that decodes to a wrong length (too short) should be rejected.
    #[test]
    fn test_signature_too_short_should_fail() {
        let seed = [42u8; 64];
        let key = ed25519::keypair_from_seed(&seed).unwrap();
        let msg = "hello";

        // Create a valid signature, then remove one byte to make it too short
        let mut sig_bytes = ed25519::sign_message(&key, msg.as_bytes())
            .to_bytes()
            .to_vec();
        sig_bytes.pop();
        let sig_b58 = bs58::encode(sig_bytes).into_string();
        let pubkey_b58 = bs58::encode(key.verifying_key().to_bytes()).into_string();

        let exit_code = verify::handle_verify(msg, &sig_b58, &pubkey_b58, false);
        assert_eq!(exit_code, 1, "should fail");
    }

    /// A public key that decodes to a wrong length (too long) should be rejected.
    #[test]
    fn test_pubkey_too_long_should_fail() {
        let seed = [42u8; 64];
        let key = ed25519::keypair_from_seed(&seed).unwrap();
        let msg = "hello";

        // Create a valid public key, then add one byte to make it too long
        let sig_b58 =
            bs58::encode(ed25519::sign_message(&key, msg.as_bytes()).to_bytes()).into_string();
        let mut pubkey_bytes = key.verifying_key().to_bytes().to_vec();
        pubkey_bytes.push(0);
        let pubkey_b58 = bs58::encode(pubkey_bytes).into_string();

        let exit_code = verify::handle_verify(msg, &sig_b58, &pubkey_b58, false);
        assert_eq!(exit_code, 1, "should fail");
    }

    /// Verifying with a mismatched public key (not the one used for signing) must fail.
    #[test]
    fn test_verify_with_mismatched_pubkey_should_fail() {
        // Valid key
        let signing_key = ed25519::keypair_from_seed(&[1u8; 64]).unwrap();
        let msg = "message";
        let sig = ed25519::sign_message(&signing_key, msg.as_bytes());
        let sig_b58 = bs58::encode(sig.to_bytes()).into_string();

        // Another one valid key
        let verification_key = ed25519::keypair_from_seed(&[2u8; 64]).unwrap();
        let pubkey_b58 = bs58::encode(verification_key.verifying_key().to_bytes()).into_string();

        let exit_code = verify::handle_verify(msg, &sig_b58, &pubkey_b58, false);
        assert_eq!(exit_code, 1, "Verification should fail with a different public key");
    }
}
