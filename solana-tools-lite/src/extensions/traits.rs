use crate::models::analysis::TxAnalysis;
use crate::models::message::Message;
use crate::models::pubkey_base58::PubkeyBase58;
use crate::ToolError;

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

    /// Deep analysis with access to the full message and mutable analysis state.
    fn analyze(
        &self,
        message: &Message,
        account_list: &[PubkeyBase58],
        signer: &PubkeyBase58,
        analysis: &mut TxAnalysis,
    );

    /// //TODO: 🟡 1) add doc 
    fn enrich_notice(&self, analysis: &mut TxAnalysis);
}