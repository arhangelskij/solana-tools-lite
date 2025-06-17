#[cfg(test)]
mod tests {
    use solana_tools_lite::crypto::bip39;

    /// The generated mnemonic should consist of exactly 12 English words.
    #[test]
    fn test_generate_mnemonic_word_count() {
        let phrase = bip39::generate_mnemonic();
        assert_eq!(
            phrase.split_whitespace().count(),
            12,
            "Expected 12 words, got {}",
            phrase.split_whitespace().count()
        );
    }

    /// Deriving a seed with the same mnemonic and passphrase must be consistent.
    #[test]
    fn test_derive_seed_consistent() {
        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let passphrase = "";
        let seed1 = bip39::derive_seed(mnemonic, passphrase);
        let seed2 = bip39::derive_seed(mnemonic, passphrase);
        assert_eq!(seed1, seed2, "Seeds should match for identical inputs");
    }

    /// Deriving a seed with different passphrases should produce different outputs.
    #[test]
    fn test_derive_seed_different_passphrase() {
        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let seed_no = bip39::derive_seed(mnemonic, "");
        let seed_yes = bip39::derive_seed(mnemonic, "TREZOR");
        assert_ne!(
            seed_no, seed_yes,
            "Seeds should differ for different passphrases"
        );
    }

    /// Integration test: ensure derive_seed generates the official BIP39 test vector seed for the "TREZOR" passphrase.
    #[test]
    fn test_wrapper_produces_standard_seed_with_trezor_passphrase() {
        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let passphrase = "TREZOR";

        // This seed is taken from the official BIP39 test vectors for the above mnemonic and "TREZOR" passphrase.
        let expected_seed_hex = "c55257c360c07c72029aebc1b53c05ed0362ada38ead3e3e9efa3708e53495531f09a6987599d18264c1e1c92f2cf141630c7a3c4ab7c81b2f001698e7463b04";

        let derived_seed = bip39::derive_seed(mnemonic, passphrase);
        let derived_seed_hex = hex::encode(&derived_seed);

        assert_eq!(
            derived_seed_hex, expected_seed_hex,
            "Generated seed does not match the official test vector for passphrase 'TREZOR'"
        );
    }
}
