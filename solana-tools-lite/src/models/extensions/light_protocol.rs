use super::PrivacyImpact;
use crate::utils::format_sol;
use serde::Serialize;

/// Action types detected for Light Protocol (ZK Compression).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum LightProtocolAction {
    /// Creation of a new compressed mint.
    CreateMint,
    /// Minting tokens confidentially.
    MintTo,
    /// Privacy-preserving transfer of compressed assets.
    Transfer,
    /// Compression of SOL (Public -> Compressed).
    CompressSol { lamports: Option<u64> },
    /// Compression of Tokens (Public -> Compressed).
    CompressToken { amount: Option<u64> },
    /// Decompression of assets (Compressed -> Public).
    Decompress,
    /// Read/Update of compressed state (Generic).
    StateUpdate,
    /// Closing a compressed account.
    CloseAccount,
    /// Action not specifically parsed but identified as Light Protocol.
    Unknown {
        discriminator: [u8; 8], //TODO: type alias
    },
}

impl LightProtocolAction {
    pub fn description(&self) -> String {
        match self {
            Self::CreateMint => "Create ZK Mint".to_string(),
            Self::MintTo => "Mint Private Tokens".to_string(),
            Self::Transfer => "Private Transfer".to_string(),
            Self::CompressSol { lamports } => {
                if let Some(l) = lamports {
                    format!("Compress {}", format_sol(*l as u128))
                } else {
                    "Compress SOL".to_string()
                }
            }
            Self::CompressToken { amount } => {
                if let Some(a) = amount {
                    format!("Compress Token (amount: {})", a)
                } else {
                    "Compress Token".to_string()
                }
            }
            Self::Decompress => "Decompress Assets".to_string(),
            Self::StateUpdate => "Read/Update ZK State".to_string(),
            Self::CloseAccount => "Close ZK Account".to_string(),
            Self::Unknown { .. } => "Unknown ZK Action".to_string(),
        }
    }

    /// Determine the privacy impact of this Light Protocol action.
    ///
    /// # Classification
    ///
    /// - `StorageCompression`: Operations that manage compressed state infrastructure
    ///   (compression, decompression, state updates) without directly transferring value.
    /// - `Confidential`: Operations involving private value transfers where amounts
    ///   and recipients are hidden via zero-knowledge proofs.
    /// - `None`: Unknown operations that cannot be classified.
    pub fn privacy_impact(&self) -> PrivacyImpact {
        match self {
            // Storage compression operations - manage compressed state infrastructure
            Self::CreateMint => PrivacyImpact::StorageCompression,
            Self::CompressSol { .. } => PrivacyImpact::StorageCompression,
            Self::CompressToken { .. } => PrivacyImpact::StorageCompression,
            Self::Decompress => PrivacyImpact::StorageCompression,
            Self::CloseAccount => PrivacyImpact::StorageCompression,

            // StateUpdate is classified as StorageCompression because:
            // - It updates on-chain compressed state (merkle trees, nullifiers)
            // - Does not directly transfer assets between parties
            // - Infrastructure operation for the compression system
            // - Proof verification for transfers happens in Transfer/MintTo instructions
            Self::StateUpdate => PrivacyImpact::StorageCompression,

            // Confidential operations - involve private value transfers
            Self::MintTo => PrivacyImpact::Confidential,
            Self::Transfer => PrivacyImpact::Confidential,

            // Unknown operations cannot be classified
            Self::Unknown { .. } => PrivacyImpact::None,
        }
    }
}
