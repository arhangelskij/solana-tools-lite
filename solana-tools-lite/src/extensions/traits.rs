use crate::models::analysis::TxAnalysis;
use crate::models::message::Message;
use crate::models::pubkey_base58::PubkeyBase58;

/// Trait for protocol-specific analyzers (Plugins).
///
/// Analyzers have full access to the transaction message and can mutate the
/// final `TxAnalysis` result to add actions, warnings, or adjust metrics.
pub trait ProtocolAnalyzer: Send + Sync {
    /// Friendly name of the protocol (e.g., "Light Protocol").
    fn name(&self) -> &'static str;

    /// List of program IDs owned/used by this protocol.
    /// Used by the registry for high-performance early filtering.
    fn supported_programs(&self) -> &'static [PubkeyBase58];

    /// Quick check if the transaction contains relevant instructions.
    fn detect(&self, message: &Message) -> bool {
        let supported = self.supported_programs();
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

    /// Optional rich text or notice to append to the summary.
    fn enrich_notice(&self, _analysis: &TxAnalysis) -> Option<String> {
        None
    }
}