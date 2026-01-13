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
    CompressSol {
        /// Amount of SOL being compressed in lamports (if parseable).
        lamports: Option<u64>,
    },
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
                    format!("Compress SOL ({} lamports)", l)
                } else {
                    "Compress SOL".to_string()
                }
            }
            Self::Unknown { .. } => "Unknown ZK Action".to_string(),
        }
    }
}

/// A wrapper enum for all supported propriety/extension protocol actions.
///
/// This enum allows the analysis engine to treat different protocols uniformly.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "protocol", content = "action")]
pub enum ExtensionAction {
    /// Action detected from Light Protocol (ZK Compression).
    LightProtocol(LightProtocolAction),
}
