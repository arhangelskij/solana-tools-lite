use solana_tools_lite::errors::{
    AsExitCode, Bip39Error, DeserializeError, ExitCode, GenError, KeypairError, SignError,
    TransactionParseError, ToolError, VerifyError,
};
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error(transparent)]
    Core(#[from] ToolError),
    #[error("--summary-json requires --output (file) to keep signed tx off stdout")]
    SummaryRequiresOutput,
    #[error("Fee {fee_lamports} exceeds max-fee limit {max_lamports} lamports")]
    FeeLimitExceeded {
        fee_lamports: u128,
        max_lamports: u64,
    },
    #[error("User rejected signing")]
    UserRejected,
    #[error("failed to encode summary json: {0}")]
    SummaryEncode(String),
    #[error("failed to encode json output: {0}")]
    PresentationEncode(String),
    #[error("failed to read stdin: {0}")]
    StdinRead(String),
}

impl AsExitCode for CliError {
    fn as_exit_code(&self) -> i32 {
        match self {
            CliError::Core(err) => err.as_exit_code(),
            CliError::SummaryRequiresOutput | CliError::UserRejected => ExitCode::Usage.as_i32(),
            CliError::FeeLimitExceeded { .. } => ExitCode::DataErr.as_i32(),
            CliError::SummaryEncode(_) | CliError::PresentationEncode(_) => {
                ExitCode::Software.as_i32()
            }
            CliError::StdinRead(_) => ExitCode::IoErr.as_i32(),
        }
    }
}

/// Helper for early exit when required options are missing or invalid.
pub fn fail_invalid_input(context: &str, message: &str) -> ! {
    eprintln!("{}: {}", context, message);
    std::process::exit(64);
}

/// Unified CLI error reporting with user-friendly formatting.
pub fn report_cli_error(context: &str, err: CliError) -> ! {
    eprintln!("{}: {}", context, format_cli_error(&err));
    std::process::exit(err.as_exit_code());
}

fn format_cli_error(err: &CliError) -> String {
    match err {
        CliError::Core(core) => format_user_friendly(core),
        CliError::SummaryRequiresOutput => {
            "--summary-json requires --output (file) to keep signed tx off stdout".to_string()
        }
        CliError::FeeLimitExceeded {
            fee_lamports,
            max_lamports,
        } => format!(
            "Fee {} exceeds max-fee limit {} lamports",
            fee_lamports, max_lamports
        ),
        CliError::UserRejected => "User rejected signing".to_string(),
        CliError::SummaryEncode(msg) => format!("failed to encode summary json: {msg}"),
        CliError::PresentationEncode(msg) => format!("failed to encode json output: {msg}"),
        CliError::StdinRead(msg) => format!("failed to read stdin: {msg}"),
    }
}

fn format_user_friendly(err: &ToolError) -> String {
    match err {
        ToolError::Bip39(e) => format_bip39(e),
        ToolError::Base58(e) => format!("Invalid Base58 encoding: {}", e),
        ToolError::Sign(e) => format_sign(e),
        ToolError::Keypair(e) => format_keypair(e),
        ToolError::Gen(e) => format_gen(e),
        ToolError::TransactionParse(e) => format_tx_parse(e),
        ToolError::Deserialize(e) => format_deserialize(e),
        ToolError::Verify(e) => format_verify(e),
        ToolError::Io(io_err) => format_io(io_err),
        ToolError::FileExists { path } => {
            format!(
                "Cannot create file '{}': already exists\nHint: Use --force to overwrite",
                path
            )
        }
        ToolError::InvalidInput(msg) => msg.clone(),
    }
}

fn format_bip39(e: &Bip39Error) -> String {
    match e {
        Bip39Error::InvalidWordCount(got) => {
            format!(
                "Invalid mnemonic length: got {} words, expected 12 or 24",
                got
            )
        }
        Bip39Error::Mnemonic(msg) => {
            format!(
                "Mnemonic validation failed: {}\nHint: Check that all words are from the BIP-39 wordlist",
                msg
            )
        }
    }
}

fn format_sign(e: &SignError) -> String {
    match e {
        SignError::InvalidBase58 => "Invalid Base58 encoding in secret key".to_string(),
        SignError::InvalidPubkeyFormat => {
            "Invalid public key format\nHint: Public keys must be valid Base58-encoded Ed25519 keys"
                .to_string()
        }
        SignError::InvalidKeyLength => "Secret key must be exactly 32 bytes".to_string(),
        SignError::SigningFailed(msg) => {
            format!(
                "Failed to sign transaction: {}\nHint: Verify your secret key is valid",
                msg
            )
        }
        SignError::SignerKeyNotFound => {
            "Signer public key not found in transaction account keys\nHint: The keypair you're using doesn't match any signer in this transaction".to_string()
        }
        SignError::SigningNotRequiredForKey => {
            "The provided signer is not a required signer for this transaction\nHint: Check that you're using the correct keypair".to_string()
        }
        SignError::JsonParse(e) => {
            format!(
                "Failed to parse input JSON: {}\nHint: Check that your JSON is valid",
                e
            )
        }
    }
}

fn format_keypair(e: &KeypairError) -> String {
    match e {
        KeypairError::SeedTooShort(got) => {
            format!(
                "Seed is too short: got {} bytes, expected at least 32 bytes",
                got
            )
        }
        KeypairError::SeedSlice(msg) => {
            format!("Invalid seed data: {}", msg)
        }
    }
}

fn format_gen(e: &GenError) -> String {
    match e {
        GenError::InvalidSeedLength => {
            "Invalid seed length: expected 64 bytes\nHint: Use a valid BIP-39 mnemonic or provide a 64-byte seed".to_string()
        }
        GenError::CryptoError(msg) => {
            format!(
                "Cryptographic error: {}\nHint: Check your mnemonic and derivation path",
                msg
            )
        }
        GenError::InvalidDerivationPath(msg) => {
            format!(
                "Invalid derivation path: {}\nHint: Use BIP-44 format like m/44'/501'/0'/0'",
                msg
            )
        }
    }
}

fn format_tx_parse(e: &TransactionParseError) -> String {
    match e {
        TransactionParseError::InvalidBase64(msg) => {
            format!(
                "Invalid Base64 encoding in transaction: {}\nHint: Check that the transaction is properly Base64-encoded",
                msg
            )
        }
        TransactionParseError::InvalidBase58(msg) => {
            format!(
                "Invalid Base58 encoding in transaction: {}\nHint: Check that the transaction is properly Base58-encoded",
                msg
            )
        }
        TransactionParseError::InvalidInstructionData(msg) => {
            format!("Invalid instruction data: {}", msg)
        }
        TransactionParseError::InvalidPubkeyFormat(msg) => {
            format!("Invalid public key in transaction: {}", msg)
        }
        TransactionParseError::InvalidSignatureLength(len) => {
            format!("Invalid signature length: expected 64 bytes, got {}", len)
        }
        TransactionParseError::InvalidPubkeyLength(len) => {
            format!("Invalid public key length: expected 32 bytes, got {}", len)
        }
        TransactionParseError::InvalidSignatureFormat(msg) => {
            format!("Invalid signature format: {}", msg)
        }
        TransactionParseError::InvalidBlockhashLength(len) => {
            format!("Invalid blockhash length: expected 32 bytes, got {}", len)
        }
        TransactionParseError::InvalidBlockhashFormat(msg) => {
            format!("Invalid blockhash format: {}", msg)
        }
        TransactionParseError::InvalidFormat(msg) => {
            format!(
                "Invalid transaction format: {}\nHint: Ensure the transaction is in JSON, Base64, or Base58 format",
                msg
            )
        }
        TransactionParseError::Serialization(msg) => {
            format!("Failed to serialize transaction: {}", msg)
        }
    }
}

fn format_deserialize(e: &DeserializeError) -> String {
    match e {
        DeserializeError::Deserialization(msg) => {
            format!(
                "Failed to deserialize transaction: {}\nHint: Check that the input is a valid Solana transaction",
                msg
            )
        }
    }
}

fn format_verify(e: &VerifyError) -> String {
    match e {
        VerifyError::Base58Decode(e) => format!("Invalid Base58 encoding: {}", e),
        VerifyError::InvalidSignatureLength(len) => {
            format!("Invalid signature length: expected 64 bytes, got {}", len)
        }
        VerifyError::InvalidPubkeyLength(len) => {
            format!("Invalid public key length: expected 32 bytes, got {}", len)
        }
        VerifyError::InvalidSignatureFormat => {
            "Invalid signature format\nHint: Signatures must be 64-byte Ed25519 signatures".to_string()
        }
        VerifyError::InvalidPubkeyFormat => {
            "Invalid public key format\nHint: Public keys must be 32-byte Ed25519 public keys"
                .to_string()
        }
        VerifyError::VerificationFailed => {
            "Signature verification failed\nHint: The signature does not match the message and public key"
                .to_string()
        }
    }
}


fn format_io(err: &solana_tools_lite::errors::IoError) -> String {
    // Core error already formats nicely as "io(path: error)" or "io(error)"
    // We just want to append a hint if possible.
    
    let hint = match err.kind() {
        io::ErrorKind::NotFound => "\nHint: Check that the file path is correct",
        io::ErrorKind::PermissionDenied => {
            "\nHint: Ensure you have read/write permissions for this file"
        }
        io::ErrorKind::AlreadyExists => "\nHint: Use --force to overwrite existing files",
        _ => "",
    };

    // Use the core Display repr which includes path and source error
    format!("{}{}", err, hint)
}
