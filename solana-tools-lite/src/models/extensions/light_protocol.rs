use serde::Serialize;
use super::PrivacyImpact;
use crate::utils::format_sol;

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
        discriminator: [u8; 8],
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
    pub fn privacy_impact(&self) -> PrivacyImpact {
        match self {
            Self::CreateMint => PrivacyImpact::StorageCompression,
            Self::MintTo => PrivacyImpact::Confidential,
            Self::Transfer => PrivacyImpact::Confidential,
            Self::CompressSol { .. } => PrivacyImpact::StorageCompression,
            Self::CompressToken { .. } => PrivacyImpact::StorageCompression,
            Self::Decompress => PrivacyImpact::StorageCompression,
            Self::StateUpdate => PrivacyImpact::Confidential,
            Self::CloseAccount => PrivacyImpact::StorageCompression,
            Self::Unknown { .. } => PrivacyImpact::None,
        }
    }
}
