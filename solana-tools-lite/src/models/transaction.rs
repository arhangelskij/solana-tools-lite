use serde::{Serialize, Deserialize};

/// Represents a full Solana transaction, including all signatures and the serialized message.
/// Signatures may be empty for unsigned transactions (for cold signing).
#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    /// Array of base58-encoded signatures (one for each required signer).
    /// For unsigned TX, this can be empty or contain empty strings.
    pub signatures: Vec<String>,

    /// The actual message (to be signed): contains accounts, recent blockhash, and instructions.
    pub message: Message
}

/// Solana transaction message — the payload to be signed.
/// This must be serialized in a canonical format before signing!
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    /// List of all account addresses (base58).
    pub account_keys: Vec<String>,

    /// Recent blockhash as base58 string (used for replay protection).
    pub recent_blockhash: String,

    /// List of instructions — each instruction defines a program call (e.g., transfer, mint).
    pub instructions: Vec<Instruction>
}

/// Solana instruction — represents a single call to a smart contract/program.
#[derive(Debug, Serialize, Deserialize)]
pub struct Instruction {
    /// Index of the program in the account_keys array.
    pub program_id_index: u8,

    /// List of indices of the involved accounts (in account_keys array).
    pub accounts: Vec<u8>,

    /// Instruction data, base58 or base64 encoded (depends on source, but base58 is common in Solana).
    pub data: String
}