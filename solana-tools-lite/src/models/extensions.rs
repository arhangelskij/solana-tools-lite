use serde::Serialize;

pub mod light_protocol;

pub use light_protocol::LightProtocolAction;

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
