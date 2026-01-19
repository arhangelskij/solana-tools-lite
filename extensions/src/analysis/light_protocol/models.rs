use solana_tools_lite::models::extensions::PrivacyImpact;

/// Action types detected for Light Protocol (ZK Compression).
#[derive(Debug, Clone, PartialEq)]
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
    Unknown { discriminator: [u8; 8] },
}

impl LightProtocolAction {
    pub fn description(&self) -> String {
        match self {
            Self::CreateMint => "Create ZK Mint".to_string(),
            Self::MintTo => "Mint Private Tokens".to_string(),
            Self::Transfer => "Private Transfer".to_string(),
            Self::CompressSol { lamports } => {
                if let Some(l) = lamports {
                    format!("Compress {} SOL", *l as f64 / 1_000_000_000.0)
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
    /// - `Hybrid`: Operations that transition between public and private state
    ///   (compression/decompression). These operations involve both public and
    ///   private data in a single transaction.
    /// - `Confidential`: Operations involving private value transfers where amounts
    ///   and recipients are hidden via zero-knowledge proofs.
    /// - `StorageCompression`: Operations that manage compressed state infrastructure
    ///   without directly transferring value.
    /// - `None`: Unknown operations that cannot be classified.
    pub fn privacy_impact(&self) -> PrivacyImpact {
        match self {
            // Hybrid operations - transition between public and private state
            // Compress: Public → Private (hybrid transaction)
            // Decompress: Private → Public (hybrid transaction)
            Self::CompressSol { .. } => PrivacyImpact::Hybrid,
            Self::CompressToken { .. } => PrivacyImpact::Hybrid,
            Self::Decompress => PrivacyImpact::Hybrid,

            // Confidential operations - fully private value transfers
            Self::MintTo => PrivacyImpact::Confidential,
            Self::Transfer => PrivacyImpact::Confidential,

            // Storage compression operations - infrastructure management
            Self::CreateMint => PrivacyImpact::StorageCompression,
            Self::StateUpdate => PrivacyImpact::StorageCompression,
            Self::CloseAccount => PrivacyImpact::StorageCompression,

            // Unknown
            Self::Unknown { .. } => PrivacyImpact::None,
        }
    }
}

/// Implement ExtensionAction trait for Light Protocol actions.
impl solana_tools_lite::models::extensions::ExtensionAction for LightProtocolAction {
    fn protocol_name(&self) -> &'static str {
        "Light Protocol"
    }
    
    fn description(&self) -> String {
        self.description()
    }
    
    fn privacy_impact(&self) -> PrivacyImpact {
        self.privacy_impact()
    }
}
