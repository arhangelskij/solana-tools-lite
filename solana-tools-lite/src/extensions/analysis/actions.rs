use serde::Serialize;
use std::sync::Arc;
use super::super::traits::ExtensionAction;

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

/// Analysis-specific action trait that extends `ExtensionAction` with privacy classification.
pub trait AnalysisAction: ExtensionAction {
    /// Get the privacy impact classification of this action.
    fn privacy_impact(&self) -> PrivacyImpact;
} 

/// Type-erased wrapper for analysis-specific extension actions.
/// 
/// Wraps any action implementing `AnalysisAction` (which includes privacy impact classification).
/// Used in `TxAnalysis` to store heterogeneous protocol-specific actions.
#[derive(Clone)]
pub struct AnalysisExtensionAction(Arc<dyn AnalysisAction>);

impl AnalysisExtensionAction {
    pub fn new(action: Arc<dyn AnalysisAction>) -> Self {
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