use serde::Serialize;
use std::sync::Arc;

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

/// Trait for protocol-specific actions detected during analysis.
/// 
/// Extensions implement this trait for their specific action types,
/// allowing core to remain agnostic to specific protocol details.
pub trait ExtensionAction: Send + Sync  {
    /// Get the protocol name (e.g., "Light Protocol", "Arcium").
    fn protocol_name(&self) -> &'static str;
    
    /// Get a human-readable description of this action.
    fn description(&self) -> String;
    
    /// Get the privacy impact classification of this action.
    fn privacy_impact(&self) -> PrivacyImpact;
}

//TODO: 🟡 18jan check if 

/// Type-erased wrapper for extension actions.
#[derive(Clone)]
pub struct AnalysisExtensionAction(Arc<dyn ExtensionAction>);

impl AnalysisExtensionAction {
    pub fn new(action: Arc<dyn ExtensionAction>) -> Self {
        Self(action)
    }
    
    pub fn protocol_name(&self) -> &'static str {
        self.0.protocol_name()
    }
    
    pub fn description(&self) -> String {
        self.0.description()
    }
    
    pub fn privacy_impact(&self) -> PrivacyImpact {
        self.0.privacy_impact()
    }
}

impl std::fmt::Debug for AnalysisExtensionAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.description())
    }
}
