use std::io;
use thiserror::Error;

pub type Result<T, E = ToolError> = std::result::Result<T, E>;

/// Top-level error every command bubbles up.
#[derive(Debug, Error)]
pub enum ToolError {
    // Crypto / key stuff
    #[error("BIP-39 error: {0}")]
    Bip39(#[from] Bip39Error),

     #[error(transparent)]
    Base58(#[from] bs58::decode::Error),

    // Handlers
    #[error("Sign error: {0}")]
    Sign(#[from] SignError),

    #[error("Keypair error: {0}")]
    Keypair(#[from] KeypairError),

    #[error("Keypair error: {0}")]
    Gen(#[from] GenError)
}

/// Errors that can arise when working with BIP‑39 helpers.
#[derive(Error, Debug)]
pub enum Bip39Error {
    #[error("failed to generate mnemonic: {0}")]
    Mnemonic(#[from] bip39::Error),
    #[error("PBKDF2 failed: {0}")]
    Pbkdf2(&'static str),
    #[error("Validation failed: {0}")]
    Validation(bip39::Error)
}

#[derive(Error, Debug)]
pub enum SignError {
    #[error("Invalid base58 in secret key")]
    InvalidBase58,
    #[error("Secret key must be 32 bytes")]
    InvalidKeyLength,
    #[error("Failed to sign transaction: {0}")]
    SigningFailed(String),

    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("Serialization error: {0}")]
    SerdeJson(#[from] serde_json::Error)
}

// VerifyError constants
pub const SIG_LEN: usize = 64;
pub const PUBKEY_LEN: usize = 32;

#[derive(Error, Debug)]
pub enum VerifyError {
    #[error("Base58 decode error: {0}")]
    Base58Decode(#[from] bs58::decode::Error),
    #[error("Invalid signature length: expected {SIG_LEN}, got {0}")]
    InvalidSignatureLength(usize),
    #[error("Invalid public key length: expected {PUBKEY_LEN}, got {0}")]
    InvalidPubkeyLength(usize),
    #[error("Invalid signature format")]
    InvalidSignatureFormat,
    #[error("Invalid public key format")]
    InvalidPubkeyFormat,
    #[error("Signature verification failed")]
    VerificationFailed
}

#[derive(Error, Debug)]
pub enum KeypairError {
    #[error("Seed must be at least 32 bytes, got {0}")]
    SeedTooShort(usize),
    #[error("Invalid seed slice length: {0}")]
    SeedSlice(&'static str)          // from TryInto
}

#[derive(Error, Debug)]
pub enum GenError {
    #[error("Invalid Seed Length: ")]
    InvalidSeedLength
}