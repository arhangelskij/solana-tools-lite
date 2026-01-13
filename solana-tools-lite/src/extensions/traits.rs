use crate::models::extensions::ExtensionAction;
use crate::models::pubkey_base58::PubkeyBase58;

/// Trait for protocol-specific analyzers.
///
/// This trait allows the core analysis engine to be extended with
/// custom logic for specific protocols (e.g., Privacy, DeFi) without
/// verifying hardcoded logic in the main loop.
pub trait ProtocolAnalyzer {
    /// Analyze an instruction to see if it belongs to this protocol.
    fn analyze(&self, program_id: &PubkeyBase58, data: &[u8]) -> Option<ExtensionAction>;
}
