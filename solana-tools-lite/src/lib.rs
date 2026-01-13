//! Offline-first Solana signing toolkit core library.

pub mod handlers;
pub mod layers;
pub mod models;
pub mod extensions;

pub mod adapters;
pub mod crypto;
pub mod errors;
pub mod utils;

pub mod codec;
pub mod constants;
pub mod serde;

// Re-exports for CLI and external consumers to minimize their Cargo.toml
pub use bs58;
pub use data_encoding;
pub use serde as serde_core;
pub use serde_json;
pub use thiserror;


/// Core error type and shorthand result alias for fallible helpers.
pub use crate::errors::{Result, ToolError};

/// Unified analysis facade.
pub mod analysis {
    pub use crate::handlers::analysis::{
        analyze_transaction, build_signing_summary, parse_lookup_tables,
    };
    /// Analysis models for transaction inspection and summaries.
    pub use crate::models::analysis::{
        AnalysisWarning, SigningSummary, TokenProgramKind, TransferView, TxAnalysis,
    };
}
