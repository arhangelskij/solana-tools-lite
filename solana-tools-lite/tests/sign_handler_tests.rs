#[cfg(test)]
mod tests {
    use bs58;
    use ed25519_dalek::Verifier;
    use solana_tools_lite::adapters::io_adapter::read_and_parse_secret_key;
    use solana_tools_lite::crypto::ed25519;
    use solana_tools_lite::crypto::helpers::parse_signing_key_content;
    use solana_tools_lite::handlers::{sign_message, verify};
    use std::fs;

    fn write_tmp(path: &str, contents: &str) {
        fs::write(path, contents).expect("failed to write temp file");
    }

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

        // Write secret to a temp file (raw base58) and sign using key read from file
        let tmp_path = "tmp_sk_roundtrip.txt";
        std::fs::write(tmp_path, &secret_b58).expect("failed to write temp secret");

        let signing_key =
            read_and_parse_secret_key(tmp_path).expect("failed to read key from file");
        let sign_result = sign_message::handle(message, &signing_key).expect("signature failed");

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
        assert!(
            verify::handle(
                message,
                &sig_b58_from_handler,
                &bs58::encode(pubkey.to_bytes()).into_string()
            )
            .is_ok(),
            "high-level handler failed to verify"
        );
        let _ = std::fs::remove_file(tmp_path);
    }

    /// An invalid Base58 secret key must cause signing to fail.
    #[test]
    fn test_sign_invalid_base58_secret_should_fail() {
        let bad = "%%%not_base58%%%";
        let err = parse_signing_key_content(bad).unwrap_err().to_string();
        // Core error message is technical: "sign: InvalidBase58"
        assert!(err.contains("InvalidBase58"));
    }

    /// A secret key that decodes to a wrong length (too short) should be rejected.
    #[test]
    fn test_sign_secret_too_short_should_fail() {
        // Generate a valid secret key and then remove one byte to make it too short
        let seed = [42u8; 64];
        let key = ed25519::keypair_from_seed(&seed).unwrap();
        // Build a 31-byte secret (invalid length) and then Base58-encode it
        let mut secret_bytes = key.to_bytes()[..32].to_vec();
        secret_bytes.pop(); // now 31 bytes

        let sk_b58 = bs58::encode(secret_bytes).into_string();
        assert!(parse_signing_key_content(&sk_b58).is_err());
    }

    /// An empty message should still produce a valid signature of correct length.
    #[test]
    fn test_sign_empty_message_should_succeed() {
        let seed = [42u8; 64];
        let key = ed25519::keypair_from_seed(&seed).unwrap();
        let sig = sign_message::handle("", &key).unwrap().signature_base58;
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
        
        // Sign using key read from a temp file to ensure file-based path works
        let sk_b58 = bs58::encode(key.to_bytes()[..32].to_vec()).into_string();
        let tmp = "tmp_mismatch_sk.txt";
        write_tmp(tmp, &sk_b58);
        
        let signing_key = read_and_parse_secret_key(tmp).expect("failed to read key from file");
        let pubkey = signing_key.verifying_key();
        let sig = sign_message::handle("foo", &signing_key)
            .unwrap()
            .signature_base58;
        let _ = fs::remove_file(tmp);

        let res =
            verify::handle("bar", &sig, &bs58::encode(pubkey.to_bytes()).into_string());
        
        assert!(res.is_err());
    }
}
