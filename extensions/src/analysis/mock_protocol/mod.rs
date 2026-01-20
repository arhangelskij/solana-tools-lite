/// Mock Protocol analyzer for testing multiple protocol support.
use solana_tools_lite::extensions::traits::ProtocolAnalyzer;
use solana_tools_lite::models::analysis::TxAnalysis;
use solana_tools_lite::models::message::Message;
use solana_tools_lite::models::pubkey_base58::PubkeyBase58;
use solana_tools_lite::ToolError;

pub mod constants;
pub mod models;

use models::MockProtocolAction;

/// Mock Protocol analyzer for testing.
/// 
/// This is a test protocol that demonstrates how multiple protocols
/// can be detected and displayed in a single transaction.
pub struct MockProtocol;

impl ProtocolAnalyzer for MockProtocol {
    fn name(&self) -> &'static str {
        "Mock Protocol"
    }

    fn supported_programs(&self) -> Result<&'static [PubkeyBase58], ToolError> {
        constants::supported_programs()
    }

    fn analyze(
        &self,
        message: &Message,
        account_list: &[PubkeyBase58],
        signer: &PubkeyBase58,
        analysis: &mut TxAnalysis,
    ) {
        let programs = match self.supported_programs() {
            Ok(programs) => programs,
            Err(_) => return,
        };

        for instr in message.instructions() {
            let program_id = match account_list.get(instr.program_id_index as usize) {
                Some(pk) => pk,
                None => continue,
            };

            if !programs.contains(program_id) {
                continue;
            }

            let action = MockProtocolAction::TestAction;

            // Check if signer is involved
            let signer_involved = instr.accounts.iter().any(|&idx| {
                account_list.get(idx as usize).map(|pk| pk == signer).unwrap_or(false)
            });

            if signer_involved {
                analysis
                    .extension_actions
                    .push(solana_tools_lite::models::extensions::AnalysisExtensionAction::new(
                        std::sync::Arc::new(action),
                    ));
            }
        }
    }

    fn enrich_notice(&self, analysis: &mut TxAnalysis) {
        let mut notice = String::new();
        notice.push_str("!!! MOCK PROTOCOL NOTICE !!!\n");
        notice.push_str("This is a test protocol for demonstrating multiple protocol support.\n");
        notice.push_str("In production, this would be replaced with real protocol analyzers.\n");
        notice.push_str("\n");
        notice.push_str("Features demonstrated:\n");
        notice.push_str("- Multiple protocols in a single transaction\n");
        notice.push_str("- Independent protocol notices\n");
        notice.push_str("- Protocol-specific action detection");

        analysis.extension_notices.push(notice);
    }
}
