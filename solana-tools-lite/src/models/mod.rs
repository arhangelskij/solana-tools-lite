pub mod hash_base58;
pub mod input_transaction;
pub mod instruction;
pub mod keypair_json;
pub mod message;
pub mod analysis;
pub mod pubkey_base58;
pub mod results;
pub mod transaction;
pub mod extensions;

/// 32-byte blockhash wrapper encoded in Base58.
pub use crate::models::hash_base58::HashBase58;

/// Unified input format accepted by the signer (JSON/Base58/Base64).
pub use crate::models::input_transaction::{InputTransaction, UiTransaction};

/// Canonical Solana transaction type (signatures + message).
pub use crate::models::transaction::Transaction;

/// Abstract representation of Solana message (legacy or v0).
pub use crate::models::message::Message;

/// 32-byte public key wrapper encoded in Base58.
pub use crate::models::pubkey_base58::PubkeyBase58;