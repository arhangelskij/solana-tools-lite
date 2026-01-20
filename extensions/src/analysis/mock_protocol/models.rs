/// Mock Protocol action types for testing.
use solana_tools_lite::models::extensions::PrivacyImpact;

/// Mock Protocol actions.
#[derive(Debug, Clone)]
pub enum MockProtocolAction {
    /// A test action that demonstrates protocol detection.
    TestAction,
}

impl MockProtocolAction {
    /// Get human-readable description of this action.
    pub fn description(&self) -> String {
        match self {
            Self::TestAction => "Test Action (Mock Protocol)".to_string(),
        }
    }

    /// Get privacy impact of this action.
    pub fn privacy_impact(&self) -> PrivacyImpact {
        match self {
            Self::TestAction => PrivacyImpact::None,
        }
    }
}

/// Implement ExtensionAction trait for Mock Protocol actions.
impl solana_tools_lite::models::extensions::ExtensionAction for MockProtocolAction {
    fn protocol_name(&self) -> &'static str {
        "Mock Protocol"
    }
    
    fn description(&self) -> String {
        self.description()
    }
    
    fn privacy_impact(&self) -> PrivacyImpact {
        self.privacy_impact()
    }
}
