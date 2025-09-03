use crate::constants::crypto::{PUBKEY_LEN, SIG_LEN};
use std::io;
use thiserror::Error;

/// Crate-wide result type alias that bubbles up `ToolError` by default.
pub type Result<T, E = ToolError> = std::result::Result<T, E>;

/// Top-level error every command bubbles up.
///
/// Routing rules
/// - `Sign` – I/O with context (IoWithPath), key parsing/format/length, signing domain errors
/// - `TransactionParse` – user input/output format issues (JSON/Base64/Base58, blockhash, etc.)
/// - `Deserialize` – internal raw transaction deserialization (binary → domain)
/// - `Bip39`, `Bincode`, `Base58` – wrapped library errors
/// - `InvalidInput` – CLI-level validation (mutually exclusive args, stdin forbidden, etc.)
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
    Gen(#[from] GenError),

    #[error("Bincode encode error: {0}")]
    Bincode(#[from] bincode::error::EncodeError),

    #[error("Transaction parse error: {0}")]
    TransactionParse(#[from] TransactionParseError),

    #[error("Deserialization error: {0}")]
    Deserialize(#[from] DeserializeError),

    /// File/stdin/stdout I/O errors at adapter/CLI boundaries
    #[error("I/O error: {0}")]
    Io(#[from] IoError),

    #[error("Save file error (already exists): {path}")]
    FileExists { path: String },

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

/// Errors that can arise when working with BIP‑39 helpers.
/// Errors that can arise when working with BIP‑39 helpers.
#[derive(Error, Debug)]
pub enum Bip39Error {
    #[error("failed to generate mnemonic: {0}")]
    Mnemonic(#[from] bip39::Error),
    #[error("PBKDF2 failed: {0}")]
    Pbkdf2(&'static str),
    #[error("Validation failed: {0}")]
    Validation(bip39::Error),
}

/// Signing-related errors (keys, I/O for secrets, signature placement).
#[derive(Error, Debug)]
pub enum SignError {
    #[error("Invalid base58 in secret key")]
    InvalidBase58,
    #[error("Invalid pubkey format")]
    InvalidPubkeyFormat,
    #[error("Secret key must be 32 bytes")]
    InvalidKeyLength,
    #[error("Failed to sign transaction: {0}")]
    SigningFailed(String),

    #[error("Signer pubkey not found in account_keys")]
    SignerKeyNotFound,

    #[error("Provided signer is not within required signers")]
    SigningNotRequiredForKey,

    // I/O errors are tracked via `IoError` at adapter/CLI level

    #[error("Failed to parse input JSON: {0}")]
    JsonParse(#[source] serde_json::Error),

    #[error("Failed to serialize JSON for output: {0}")]
    JsonSerialize(#[source] serde_json::Error),
}

/// Signature verification domain errors.
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
    VerificationFailed, //TODO: 🟡 use it or delete!
}

/// Keypair construction errors (seed handling).
#[derive(Error, Debug)]
pub enum KeypairError {
    #[error("Seed must be at least 32 bytes, got {0}")]
    SeedTooShort(usize),
    #[error("Invalid seed slice length: {0}")]
    SeedSlice(&'static str), // from TryInto
}

/// Wallet generation errors.
#[derive(Error, Debug)]
pub enum GenError {
    #[error("Invalid Seed Length: ")]
    InvalidSeedLength,
}

/// High-level transaction parsing errors (UI formats, textual fields).
#[derive(Debug, Error)]
pub enum TransactionParseError {
    #[error("Invalid base64: {0}")]
    InvalidBase64(String),
    #[error("Invalid base58: {0}")]
    InvalidBase58(String),
    #[error("Invalid base58: {0}")]
    InvalidInstructionData(String),
    #[error("Invalid pubkey string: {0}")]
    InvalidPubkeyFormat(String),
    #[error("Invalid signature length: {0}")]
    InvalidSignatureLength(usize),
    #[error("Invalid pubkey length: {0}")]
    InvalidPubkeyLength(usize),
    #[error("Invalid signature format: {0}")]
    InvalidSignatureFormat(String),
    #[error("Bincode deserialize error: {0}")]
    BincodeDeserialize(String),
    #[error("Expected 32 bytes for blockhash, got {0}")]
    InvalidBlockhashLength(usize),
    #[error("Invalid blockhash string: {0}")]
    InvalidBlockhashFormat(String),
    #[error("Invalid input format: {0}")]
    InvalidFormat(String),
    #[error("Serialization error: {0}")]
    Serialization(String), //TODO: 🟡 serde error?
}

/// Low-level deserialization errors (binary transaction decoding).
#[derive(Debug, Error)]
pub enum DeserializeError {
    #[error("Deserialization error: {0}")]
    Deserialization(String),
}

/// Adapter-level I/O errors (files, stdin/stdout) with optional path context.
#[derive(Debug, Error)]
pub enum IoError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    /// `path=None` denotes stdin/stdout
    #[error("I/O error {path:?}: {source}")]
    IoWithPath {
        #[source]
        source: std::io::Error,
        path: Option<String>,
    },
}
