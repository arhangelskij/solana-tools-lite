use crate::models::transaction::Transaction;
use serde::Serialize;

/// Result of a signature verification operation.
#[derive(Serialize)]
pub struct VerifyResult {
    /// The message that was verified.
    pub message: String,
    /// The public key used for verification (Base58).
    pub pubkey: String,
    /// The signature being verified (Base58).
    pub signature: String,
    /// Whether the signature is valid for the given message and public key.
    pub valid: bool,
}

/// Result of a new keypair generation.
#[derive(Serialize)]
pub struct GenResult {
    /// BIP-39 mnemonic phrase.
    pub mnemonic: String,
    /// Derived public key (Base58).
    #[serde(rename = "publicKey")]
    pub public_key: String,
    /// Derived secret key (Base58).
    #[serde(rename = "secretKey")]
    pub secret_key: String,
    /// 64-byte seed derived from mnemonic and passphrase (Hex).
    pub seed_hex: String,
}

/// Result of signing a single message.
#[derive(Serialize, Debug)]
pub struct SignResult {
    /// The message that was signed.
    pub message: String,
    /// The resulting signature (Base58).
    pub signature_base58: String,
    /// The public key of the signer (Base58).
    pub public_key: String,
}

/// Result of signing a structured Solana transaction.
#[derive(Debug)]
pub struct SignTxResult {
    /// The transaction object with the new signature(s) applied.
    pub signed_tx: Transaction,
}

/// Result of a Base58 encoding or decoding operation.
#[derive(Serialize, Debug)]
pub struct Base58Result {
    /// The action performed: "encode" or "decode".
    pub action: String,
    /// The input string or bytes.
    pub input: String,
    /// The resulting output string or bytes.
    pub output: String,
}
