use crate::models::message::Message;
use serde::{Deserialize, Serialize};

/// A Solana transaction consists of a list of signatures and a message.
///
/// Each signature corresponds to a required signer defined in the message header.
/// If the transaction is unsigned (e.g., created for cold signing), the signatures
/// list might be empty or contain placeholder (zero) signatures.
#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    /// Ordered list of signatures.
    /// The number of signatures must match the message header's `num_required_signatures`.
    #[serde(with = "crate::serde::signature")]
    pub signatures: Vec<ed25519_dalek::Signature>,
    /// The content of the transaction, including instructions, account keys, and blockhash.
    pub message: Message,
}
