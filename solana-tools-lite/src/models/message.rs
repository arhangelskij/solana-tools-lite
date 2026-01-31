use crate::models::instruction::Instruction;
use crate::models::{hash_base58::HashBase58, pubkey_base58::PubkeyBase58};
use serde::{Deserialize, Serialize};

/// The signed content of a Solana transaction.
///
/// A message contains all instructions, account keys, and metadata needed to execute
/// a transaction on-chain. It can be either a Legacy format or a Versioned (v0) format.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Message {
    /// Traditional Solana message format.
    Legacy(MessageLegacy),
    /// Versioned message format supporting Address Lookup Tables (ALTs).
    V0(MessageV0),
}

impl Message {
    /// Return the message header.
    pub fn header(&self) -> &MessageHeader {
        match self {
            Message::Legacy(m) => &m.header,
            Message::V0(m) => &m.header,
        }
    }

    /// Returns the list of account keys involved in this message.
    pub fn account_keys(&self) -> &Vec<PubkeyBase58> {
        match self {
            Message::Legacy(m) => &m.account_keys,
            Message::V0(m) => &m.account_keys,
        }
    }

    /// Returns the recent blockhash used for replay protection.
    pub fn recent_blockhash(&self) -> &HashBase58 {
        match self {
            Message::Legacy(m) => &m.recent_blockhash,
            Message::V0(m) => &m.recent_blockhash,
        }
    }

    /// Returns the list of instructions to execute.
    pub fn instructions(&self) -> &Vec<Instruction> {
        match self {
            Message::Legacy(m) => &m.instructions,
            Message::V0(m) => &m.instructions,
        }
    }

    /// Returns a mutable reference to the account keys list.
    pub fn account_keys_mut(&mut self) -> &mut Vec<PubkeyBase58> {
        match self {
            Message::Legacy(m) => &mut m.account_keys,
            Message::V0(m) => &mut m.account_keys,
        }
    }
    /// Validate internal constraints (e.g., duplicate keys) for this message.
    pub fn sanitize(&self) -> crate::errors::Result<()> {
        match self {
            Message::Legacy(m) => m.sanitize(),
            Message::V0(m) => m.sanitize(),
        }
    }
}

/// Legacy message format used in the original Solana protocol.
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageLegacy {
    /// Metadata about account signing and read/write requirements.
    pub header: MessageHeader,
    /// Ordered list of all account addresses involved in the transaction.
    #[serde(with = "solana_short_vec")]
    pub account_keys: Vec<PubkeyBase58>,
    /// Recent blockhash hash used for replay protection and lifetime control.
    pub recent_blockhash: HashBase58,
    /// List of executable instructions in the transaction.
    #[serde(with = "solana_short_vec")]
    pub instructions: Vec<Instruction>,
}

impl MessageLegacy {
    /// Validate internal constraints (e.g., duplicate keys) for legacy messages.
    pub fn sanitize(&self) -> crate::errors::Result<()> {
        if has_duplicates(&self.account_keys) {
            return Err(crate::errors::ToolError::InvalidInput(
                "Message contains duplicate account keys".into(),
            ));
        }
        Ok(())
    }
}

/// Versioned message format (v0) supporting address table lookups.
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageV0 {
    /// Metadata about account signing and read/write requirements.
    pub header: MessageHeader,
    /// List of static account keys (those not loaded via lookup tables).
    #[serde(with = "solana_short_vec")]
    pub account_keys: Vec<PubkeyBase58>,
    /// Recent blockhash hash for replay protection.
    pub recent_blockhash: HashBase58,
    /// List of executable instructions.
    #[serde(with = "solana_short_vec")]
    pub instructions: Vec<Instruction>,
    /// Dynamic account resolution via Address Lookup Tables (ALTs).
    #[serde(with = "solana_short_vec")]
    pub address_table_lookups: Vec<MessageAddressTableLookup>,
}

impl MessageV0 {
    /// Validate internal constraints (e.g., duplicate keys) for v0 messages.
    pub fn sanitize(&self) -> crate::errors::Result<()> {
        if has_duplicates(&self.account_keys) {
            return Err(crate::errors::ToolError::InvalidInput(
                "Message contains duplicate account keys".into(),
            ));
        }
        // Scan lookups for duplicate writable/readonly indexes within the same table is allowed (but silly),
        // but duplicate lookup keys are usually allowed by the runtime (merged), though wasteful.
        // We will stick to checking the static key list for duplicates as the primary constraint.
        Ok(())
    }
}

// Helper strict check.
// We use a simple O(N^2) check here because `account_keys` is always very small
// (max ~64 items due to transaction size limits).
// Avoiding heap allocation for a HashSet/Sort is faster for these small sizes.
fn has_duplicates(keys: &[PubkeyBase58]) -> bool {
    for (i, k1) in keys.iter().enumerate() {
        for k2 in keys.iter().skip(i + 1) {
            if k1 == k2 {
                return true;
            }
        }
    }
    false
}

/// Address table lookup for v0 transactions.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageAddressTableLookup {
    /// Address of the lookup table account
    pub account_key: PubkeyBase58,

    /// Indexes of writable accounts in the lookup table
    #[serde(with = "solana_short_vec")]
    pub writable_indexes: Vec<u8>,

    /// Indexes of readonly accounts in the lookup table
    #[serde(with = "solana_short_vec")]
    pub readonly_indexes: Vec<u8>,
}

/// Message metadata describing the number and type of signatures required.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MessageHeader {
    /// The number of signatures required for this transaction.
    /// All required signatures must appear at the beginning of the `signatures` array.
    #[serde(alias = "numRequiredSignatures")]
    pub num_required_signatures: u8,
    /// The number of required signatures that come from read-only accounts.
    #[serde(alias = "numReadonlySignedAccounts")]
    pub num_readonly_signed_accounts: u8,
    /// The number of read-only accounts that do not require a signature.
    #[serde(alias = "numReadonlyUnsignedAccounts")]
    pub num_readonly_unsigned_accounts: u8,
}
