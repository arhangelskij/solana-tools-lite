use std::fmt;
use std::io;
use thiserror::Error;

/// Crate-wide result type alias that bubbles up `ToolError` by default.
pub type Result<T, E = ToolError> = std::result::Result<T, E>;

/// Exit-code contract shared with CLI/front-ends.
pub trait AsExitCode {
    fn as_exit_code(&self) -> i32;
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ExitCode {
    Usage = 64,
    DataErr = 65,
    NoInput = 66,
    Software = 70,
    IoErr = 74,
}

impl ExitCode {
    pub const fn as_i32(self) -> i32 {
        self as i32
    }
}

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
    #[error("bip39: {0}")]
    Bip39(#[from] Bip39Error),

    #[error("base58: {0}")]
    Base58(#[from] bs58::decode::Error),

    #[error("sign: {0}")]
    Sign(#[from] SignError),

    #[error("keypair: {0}")]
    Keypair(#[from] KeypairError),

    #[error("gen: {0}")]
    Gen(#[from] GenError),

    #[error("tx_parse: {0}")]
    TransactionParse(#[from] TransactionParseError),

    #[error("deserialize: {0}")]
    Deserialize(#[from] DeserializeError),

    #[error("io: {0}")]
    Io(#[from] IoError),

    #[error("verify: {0}")]
    Verify(#[from] VerifyError),

    #[error("file_exists: {path}")]
    FileExists { path: String },

    #[error("{0}")]
    InvalidInput(String),

    #[error("configuration: {0}")]
    ConfigurationError(String),
}

/// Errors that can arise when working with BIP‑39 helpers.
#[derive(Error, Debug)]
pub enum Bip39Error {
    #[error("InvalidWordCount({0})")]
    InvalidWordCount(usize),
    #[error("Mnemonic({0})")]
    Mnemonic(String),
}

/// Signing-related errors (keys, I/O for secrets, signature placement).
#[derive(Error, Debug)]
pub enum SignError {
    #[error("InvalidBase58")]
    InvalidBase58,
    #[error("InvalidPubkeyFormat")]
    InvalidPubkeyFormat,
    #[error("InvalidKeyLength")]
    InvalidKeyLength,
    #[error("SigningFailed({0})")]
    SigningFailed(String),

    #[error("SignerKeyNotFound")]
    SignerKeyNotFound,

    #[error("SigningNotRequired")]
    SigningNotRequiredForKey,

    #[error("JsonParse({0})")]
    JsonParse(#[source] serde_json::Error),
}

/// Signature verification domain errors.
#[derive(Error, Debug)]
pub enum VerifyError {
    #[error("Base58Decode({0})")]
    Base58Decode(#[from] bs58::decode::Error),
    #[error("InvalidSigLen({0})")]
    InvalidSignatureLength(usize),
    #[error("InvalidPubkeyLen({0})")]
    InvalidPubkeyLength(usize),
    #[error("InvalidSigFormat")]
    InvalidSignatureFormat,
    #[error("InvalidPubkeyFormat")]
    InvalidPubkeyFormat,
    #[error("VerifyFailed")]
    VerificationFailed,
}

/// Keypair construction errors (seed handling).
#[derive(Error, Debug)]
pub enum KeypairError {
    #[error("SeedTooShort({0})")]
    SeedTooShort(usize),
    #[error("SeedSlice({0})")]
    SeedSlice(&'static str),
}

/// Wallet generation errors.
#[derive(Error, Debug)]
pub enum GenError {
    #[error("InvalidSeedLength")]
    InvalidSeedLength,
    #[error("CryptoError({0})")]
    CryptoError(String),
    #[error("DerivationPath({0})")]
    InvalidDerivationPath(String),
}

/// High-level transaction parsing errors (UI formats, textual fields).
#[derive(Debug, Error)]
pub enum TransactionParseError {
    #[error("Base64({0})")]
    InvalidBase64(String),
    #[error("Base58({0})")]
    InvalidBase58(String),
    #[error("InstructionData({0})")]
    InvalidInstructionData(String),
    #[error("PubkeyFormat({0})")]
    InvalidPubkeyFormat(String),
    #[error("SigLen({0})")]
    InvalidSignatureLength(usize),
    #[error("PubkeyLen({0})")]
    InvalidPubkeyLength(usize),
    #[error("SigFormat({0})")]
    InvalidSignatureFormat(String),

    #[error("BlockhashLen({0})")]
    InvalidBlockhashLength(usize),
    #[error("BlockhashFormat({0})")]
    InvalidBlockhashFormat(String),
    #[error("Format({0})")]
    InvalidFormat(String),
    #[error("Serialization({0})")]
    Serialization(String),
}

/// Low-level deserialization errors (binary transaction decoding).
#[derive(Debug, Error)]
pub enum DeserializeError {
    #[error("Deserialize({0})")]
    Deserialization(String),
}

/// Adapter-level I/O errors (files, stdin/stdout) with optional path context.
#[derive(Debug, Error)]
pub enum IoError {
    Io(#[from] io::Error),

    /// `path=None` denotes stdin/stdout
    IoWithPath {
        #[source]
        source: std::io::Error,
        path: Option<String>,
    },
}

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IoError::Io(e) => write!(f, "io({})", e),
            IoError::IoWithPath { source, path } => {
                let p = path.as_deref().unwrap_or("stdio");
                write!(f, "io({}: {})", p, source)
            }
        }
    }
}

impl IoError {
    pub fn kind(&self) -> io::ErrorKind {
        match self {
            IoError::Io(err) => err.kind(),
            IoError::IoWithPath { source, .. } => source.kind(),
        }
    }

    pub fn with_path(source: io::Error, path: impl Into<String>) -> Self {
        Self::IoWithPath {
            source,
            path: Some(path.into()),
        }
    }

    pub fn stdio(source: io::Error) -> Self {
        Self::IoWithPath { source, path: None }
    }
}

impl AsExitCode for IoError {
    fn as_exit_code(&self) -> i32 {
        use io::ErrorKind::*;
        match self.kind() {
            NotFound => ExitCode::NoInput.as_i32(),
            InvalidInput => ExitCode::Usage.as_i32(),
            PermissionDenied | AlreadyExists => ExitCode::IoErr.as_i32(),
            _ => ExitCode::IoErr.as_i32(),
        }
    }
}

impl AsExitCode for ToolError {
    fn as_exit_code(&self) -> i32 {
        match self {
            ToolError::InvalidInput(_) => ExitCode::Usage.as_i32(),
            ToolError::ConfigurationError(_) => ExitCode::Software.as_i32(),
            ToolError::Io(err) => err.as_exit_code(),
            ToolError::FileExists { .. } => ExitCode::IoErr.as_i32(),
            ToolError::Bip39(_)
            | ToolError::Base58(_)
            | ToolError::Sign(_)
            | ToolError::Keypair(_)
            | ToolError::Gen(_)
            | ToolError::Verify(_)
            | ToolError::Deserialize(_)
            | ToolError::TransactionParse(_) => ExitCode::DataErr.as_i32(),
        }
    }
}
