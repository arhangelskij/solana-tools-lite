use crate::models::analysis::TxAnalysis;
use crate::models::message::Message;
use crate::models::pubkey_base58::PubkeyBase58;
use crate::ToolError;

/// Base trait for protocol-specific extension actions.
/// 
/// This is the foundation trait that all extension action types must implement.
/// It provides basic identification and description methods.
/// 
/// Specialized extensions (e.g., for analysis) can extend this trait with additional methods.
pub trait ExtensionAction: Send + Sync {
    /// Get the protocol name (e.g., "Light Protocol", "Arcium").
    fn protocol_name(&self) -> &'static str;
    
    /// Get a human-readable description of this action.
    fn description(&self) -> String;
}

/// Trait for protocol-specific analyzers (Plugins).
///
/// Analyzers have full access to the transaction message and can mutate the
/// final `TxAnalysis` result to add actions, warnings, or adjust metrics.
pub trait ProtocolAnalyzer: Send + Sync {
    /// Friendly name of the protocol (e.g., "Light Protocol").
    fn name(&self) -> &'static str;

    /// List of program IDs owned/used by this protocol.
    /// Used by the registry for high-performance early filtering.
    /// 
    /// # Returns
    /// 
    /// `Ok(&'static [PubkeyBase58])` containing the program IDs (cached after first call),
    /// or `Err(ToolError)` if the program IDs cannot be initialized.
    /// 
    /// # Implementation Notes
    /// 
    /// Implementations should use `OnceLock` to cache the parsed program IDs,
    /// ensuring parsing happens only once even with multiple calls.
    fn supported_programs(&self) -> Result<&'static [PubkeyBase58], ToolError>;

    /// Get description for a program ID.
    fn program_description(&self, _program_id: &PubkeyBase58) -> Option<&'static str> {
        None
    }

    /// Quick check if the transaction contains relevant instructions.
    fn detect(&self, message: &Message) -> bool {
        let supported = match self.supported_programs() {
            Ok(programs) => programs,
            Err(_) => return false, // If we can't get programs, assume no detection
        };
        
        message.instructions().iter().any(|instr| {
            if let Some(pk) = message.account_keys().get(instr.program_id_index as usize) {
                supported.contains(pk)
            } else {
                false
            }
        })
    }

    /// Check if any of the protocol's programs are present in the transaction's account keys.
    /// This detects potential CPI interactions even when the program is not directly invoked.
    fn detect_in_accounts(&self, message: &Message) -> bool {
        let supported = match self.supported_programs() {
            Ok(programs) => programs,
            Err(_) => return false,
        };
        
        message.account_keys().iter().any(|pk| supported.contains(pk))
    }

    /// Deep analysis with access to the full message and mutable analysis state.
    fn analyze(
        &self,
        message: &Message,
        account_list: &[PubkeyBase58],
        signer: &PubkeyBase58,
        analysis: &mut TxAnalysis,
    );

    /// Add custom notices or disclaimers to the analysis output.
    /// This is typically used to warn users about non-standard behavior or privacy implications.
    /// Implementation should append descriptive strings to `analysis.extension_notices`.
    fn enrich_notice(&self, analysis: &mut TxAnalysis);
}