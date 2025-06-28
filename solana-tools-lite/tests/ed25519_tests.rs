#[cfg(test)]
mod tests {
    use solana_tools_lite::crypto::ed25519;

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

//TODO: add tests and use these 
    // #[error("Base58 decode error: {0}")]
    // Base58Decode(#[from] bs58::decode::Error),
    // #[error("Invalid signature length: expected {SIG_LEN}, got {0}")]
    // InvalidSignatureLength(usize),
    // #[error("Invalid public key length: expected {PUBKEY_LEN}, got {0}")]
    // InvalidPubkeyLength(usize),
    // #[error("Invalid signature format")]
    // InvalidSignatureFormat,
    // #[error("Invalid public key format")]
    // InvalidPubkeyFormat,
    // #[error("Signature verification failed")]
    // VerificationFailed
