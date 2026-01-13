use serde::Serialize;

/// Privacy impact of a specific instruction or protocol action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum PrivacyImpact {
    /// No privacy impact (purely public).
    None,
    /// Transition that involves public-to-private data archival (e.g. Compression).
    StorageCompression,
    /// Transaction involves both public and private state transitions.
    Hybrid,
    /// Fully confidential operation.
    Confidential,
}

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
    CompressSol,
    /// Compression of Tokens (Public -> Compressed).
    CompressToken,
    /// Decompression of assets (Compressed -> Public).
    Decompress,
    /// Read/Update of compressed state (Generic).
    StateUpdate,
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
            Self::CompressSol => "Compress SOL".to_string(),
            Self::CompressToken => "Compress Token".to_string(),
            Self::Decompress => "Decompress Assets".to_string(),
            Self::StateUpdate => "Read/Update ZK State".to_string(),
            Self::Unknown { .. } => "Unknown ZK Action".to_string(),
        }
    }

    /// Determine the privacy impact of this Light Protocol action.
    pub fn privacy_impact(&self) -> PrivacyImpact {
        match self {
            Self::CreateMint => PrivacyImpact::StorageCompression,
            Self::MintTo => PrivacyImpact::Confidential,
            Self::Transfer => PrivacyImpact::Confidential,
            Self::CompressSol => PrivacyImpact::StorageCompression,
            Self::CompressToken => PrivacyImpact::StorageCompression,
            Self::Decompress => PrivacyImpact::StorageCompression,
            Self::StateUpdate => PrivacyImpact::Confidential,
            Self::Unknown { .. } => PrivacyImpact::None,
        }
    }
}

/// A wrapper enum for all supported propriety/extension protocol actions.
///
/// This enum allows the analysis engine to treat different protocols uniformly.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "protocol", content = "action")]
pub enum AnalysisExtensionAction {
    /// Action detected from Light Protocol (ZK Compression).
    LightProtocol(LightProtocolAction),
}

impl AnalysisExtensionAction {
    /// Get the privacy impact of the underlying extension action.
    pub fn privacy_impact(&self) -> PrivacyImpact {
        match self {
            Self::LightProtocol(lp) => lp.privacy_impact(),
        }
    }
}
