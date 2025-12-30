pub mod bip39;
pub mod derivation;
pub mod ed25519;
pub mod helpers;

/// Mnemonic types and operations (BIP-39).
pub mod mnemonic {
    pub use crate::crypto::bip39::{
        Bip39Config, Bip39Result, NormalizedMnemonic, Seed, derive_seed,
        derive_seed_from_mnemonic, generate_mnemonic, generate_mnemonic_with, parse_mnemonic,
        validate_mnemonic,
    };
}

/// Signing types and operations (Ed25519).
pub mod signing {
    pub use crate::crypto::ed25519::{
        keypair_from_seed, sign_message, verify_signature, verify_signature_raw,
    };
    pub use crate::constants::crypto::{PUBKEY_LEN, SIG_LEN};
    pub use ed25519_dalek::{Signature, SigningKey, VerifyingKey};
}

/// Derivation types and operations for Solana-compatible key material.
pub mod derive {
    pub use crate::crypto::derivation::{derive_key_from_seed, DerivationPath, SOLANA_DERIVATION_PATH};
}
