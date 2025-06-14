use solana_tools_lite::crypto::ed25519;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_sign_verify() {
        let seed = [42u8; 64];
        let key = ed25519::keypair_from_seed(&seed).unwrap();
        let pubkey = key.verifying_key();
        let msg = b"test-message";
        let sig = ed25519::sign_message(&key, msg);

        assert!(ed25519::verify_signature(&pubkey, msg, &sig));
    }
}