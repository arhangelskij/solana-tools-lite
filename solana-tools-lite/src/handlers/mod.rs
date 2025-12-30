pub mod base58;
pub mod analysis;
pub mod generate;
pub mod sign_message;
pub mod sign_tx;
pub mod verify;


/// Signing helpers without I/O or parsing, used by the CLI and library callers.
pub use crate::handlers::sign_tx::{handle as handle_sign_transaction, sign_transaction_by_key};

/// High-level handler for mnemonic generation and wallet derivation (no I/O side effects).
pub use crate::handlers::generate::handle as handle_generate_mnemonic_and_wallet;

/// High-level handler for message signing with structured result.
pub use crate::handlers::sign_message::handle as handle_sign_message;

/// High-level handler for signature verification.
pub use crate::handlers::verify::handle as handle_verify_message;

/// Base58 encode/decode helpers with structured results.
pub use crate::handlers::base58::{decode as decode_base58, encode as encode_base58};