#[cfg(test)]
mod tests {
    use bs58;
    use ed25519_dalek::Verifier;
    use solana_tools_lite::crypto::ed25519;
    use solana_tools_lite::handlers::{sign, verify};

    /// End‑to‑end test: sign with `handle_sign`, then verify the same signature
    /// both with the low‑level `ed25519_dalek::Verifier` trait and with our
    /// high‑level `verify::handle_verify` wrapper.
    #[test]
    fn test_sign_and_verify_roundtrip() {
        let seed = [42u8; 64];
        let key = ed25519::keypair_from_seed(&seed).unwrap();
        let pubkey = key.verifying_key();

        let secret_b58 = bs58::encode(key.to_bytes()[..32].to_vec()).into_string();
        let message = "test-signing";

        // Sign using our handler
        let sign_result =  sign::handle_sign(message, &secret_b58).expect("signature failed");
        
        let sig_b58_from_handler = sign_result.signature_base58;

        // Decode signature from base58
        let sig_bytes = bs58::decode(&sig_b58_from_handler)
            .into_vec()
            .expect("invalid base58 signature");
        let sig =
            ed25519_dalek::Signature::from_bytes(&sig_bytes.try_into().expect("wrong sig len"));

        // Verify
        pubkey
            .verify(message.as_bytes(), &sig)
            .expect("signature verification failed");

        // Also validate through the public API
        let exit_code = verify::handle_verify(
            message,
            &sig_b58_from_handler,
            &bs58::encode(pubkey.to_bytes()).into_string(),
            false
        );

        assert_eq!(exit_code, 0, "high-level handler failed to verify");
    }

    /// An invalid Base58 secret key must cause signing to fail.
    #[test]
    fn test_sign_invalid_base58_secret_should_fail() {
        let bad = "%%%not_base58%%%";
        
        let err = sign::handle_sign("foo", bad).unwrap_err().to_string();
        assert!(err.contains("Invalid base58 in secret key"));
    }

    /// A secret key that decodes to a wrong length (too short) should be rejected.
    #[test]
    fn test_sign_secret_too_short_should_fail() {
        // Generate a valid secret key and then remove one byte to make it too short
        let seed = [42u8; 64];
        let key = ed25519::keypair_from_seed(&seed).unwrap();
        let mut sk = bs58::encode(key.to_bytes()[..32].to_vec()).into_string();
        sk.pop();
        assert!(sign::handle_sign("foo", &sk).is_err());
    }

    /// An empty message should still produce a valid signature of correct length.
    #[test]
    fn test_sign_empty_message_should_succeed() {
        let seed = [42u8; 64];
        let key = ed25519::keypair_from_seed(&seed).unwrap();
        let sk = bs58::encode(key.to_bytes()[..32].to_vec()).into_string();
        let sig = sign::handle_sign("", &sk).unwrap().signature_base58;
        // Decode the signature and verify its byte length is SIG_LEN
        let bytes = bs58::decode(&sig).into_vec().unwrap();
        assert_eq!(bytes.len(), 64);
    }

    /// Signing one message and verifying a different message should fail.
    #[test]
    fn test_sign_and_verify_mismatch_message_should_fail() {
        // Sign msg1, attempt to verify msg2 via our verify::handle_verify
        let seed = [42u8; 64];
        let key = ed25519::keypair_from_seed(&seed).unwrap();
        let sk = bs58::encode(key.to_bytes()[..32].to_vec()).into_string();
        let pubkey = key.verifying_key();
        let sig = sign::handle_sign("foo", &sk).unwrap().signature_base58;
        let exit_code = verify::handle_verify(
            "bar",
            &sig,
            &bs58::encode(pubkey.to_bytes()).into_string(),
            false
        );

        assert_eq!(exit_code, 1, "verification should fail for mismatched message");
    }
}
