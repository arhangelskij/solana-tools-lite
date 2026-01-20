use crate::models::extensions::AnalysisExtensionAction;
use crate::models::pubkey_base58::PubkeyBase58;
use serde::Serialize;

/// Transaction analysis output used by CLI and other front-ends.
#[derive(Debug)]
pub struct TxAnalysis {
    pub transfers: Vec<TransferView>,
    pub base_fee_lamports: u128,
    pub priority_fee_lamports: Option<(u128, bool)>, // (fee, estimated)
    pub total_fee_lamports: u128,
    pub total_sol_send_by_signer: u128,
    pub compute_unit_limit: Option<u32>,
    pub compute_unit_price_micro: Option<u64>,
    pub warnings: Vec<AnalysisWarning>,
    pub message_version: &'static str,
    /// Privacy level of this transaction based on detected confidential operations
    pub privacy_level: PrivacyLevel,
    /// Actions detected by protocol extensions (e.g. Light Protocol).
    pub extension_actions: Vec<AnalysisExtensionAction>,
    /// Custom notices or reports from protocol extensions.
    pub extension_notices: Vec<String>,
    /// Aggregated count of confidential (ZK) operations.
    pub confidential_ops_count: usize,
    /// Aggregated count of storage compression (public bridge) operations.
    pub storage_ops_count: usize,
    /// Whether the analyzed signer is the fee payer for this transaction.
    pub is_fee_payer: bool,
    /// Whether non-SOL assets (SPL/Token-2022) are involved in movement.
    pub has_non_sol_assets: bool,
}

impl TxAnalysis {
    /// Removes UnknownProgram warnings for the given list of known programs.
    pub fn resolve_unknown_programs(&mut self, known_programs: &[PubkeyBase58]) {
        self.warnings.retain(|w| {
            match w {
                AnalysisWarning::UnknownProgram { program_id } => {
                    !known_programs.contains(program_id)
                },
                _ => true
            }
        });
    }
}

#[derive(Debug, Clone)]
pub struct TransferView {
    pub from: String,
    pub to: String,
    pub lamports: u64,
    pub from_is_signer: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct SigningSummary {
    pub message_version: String,
    pub signatures: Vec<String>,
    pub signed_tx_base64: String,
    pub base_fee_lamports: u64,
    pub priority_fee_lamports: u64,
    pub priority_fee_estimated: bool,
    pub fee_is_estimate: bool,
    pub compute_unit_price_micro: Option<u64>,
    pub compute_unit_limit: Option<u32>,
    pub total_fee_lamports: u64,
    pub total_sol_send_by_signer: u64,
    pub max_total_cost_lamports: u64,
    pub is_fee_payer: bool,
    pub has_non_sol_assets: bool,
    pub warnings: Vec<AnalysisWarning>,
    /// Extension actions serialized as descriptions (not the full objects)
    pub extension_actions: Vec<String>,
    pub extension_notices: Vec<String>,
    pub confidential_ops_count: usize,
    pub storage_ops_count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub enum AnalysisWarning {
    LookupTablesNotProvided,
    LookupTableMissing(PubkeyBase58),
    TokenTransferDetected(TokenProgramKind),
    UnknownProgram { program_id: PubkeyBase58 },
    SignerNotRequired,
    CpiLimit,
    ConfidentialTransferDetected,
    MalformedInstruction,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum TokenProgramKind {
    SplToken,
    Token2022,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub enum PrivacyLevel {
    /// Fully transparent transaction on the public ledger.
    Public,
    /// Pure storage/space optimization using ZK Compression (no private transfers).
    Compressed,
    /// Mixed public and private/bridge operations.
    Hybrid,
    /// Exclusively private/shielded operations.
    Confidential,
}

impl PrivacyLevel {
    pub fn display_info(&self, confidential_ops: usize, storage_ops: usize) -> (String, &'static str) {
        match self {
            Self::Public => ("🟢 Public".to_string(), "Standard transparent transaction"),
            Self::Compressed => (
                "🟡 Compressed".to_string(),
                "Storage/space optimization only (using ZK Compression)"
            ),
            Self::Hybrid => {
                let label = "🟠 Hybrid".to_string();
                if confidential_ops > 0 && storage_ops > 0 {
                    (label, "Private transfers + public bridge (Compress/Decompress)")
                } else if confidential_ops == 0 && storage_ops > 0 {
                    (label, "Bridge operation (Public <-> ZK state transition)")
                } else {
                    (label, "Mixed transaction (both public transfers and private ZK operations)")
                }
            }
            Self::Confidential => (
                "🔴 Confidential".to_string(),
                "Shielded private operations, no public mixing detected"
            ),
        }
    }
}

impl TxAnalysis {
    /// Recalculates the privacy level based on current metrics and extension actions.
    pub fn recalculate_privacy_level(&mut self) {
        use crate::models::extensions::PrivacyImpact;

        let has_confidential = self.confidential_ops_count > 0 
            || self.warnings.iter().any(|w| matches!(w, AnalysisWarning::ConfidentialTransferDetected));
        
        let mut has_hybrid_action = false;
        let has_storage = self.storage_ops_count > 0;

        for action in &self.extension_actions {
            if action.privacy_impact() == PrivacyImpact::Hybrid {
                has_hybrid_action = true;
                break;
            }
        }

        let has_public_mixing = self.has_non_sol_assets || !self.transfers.is_empty();

        self.privacy_level = if has_hybrid_action {
            PrivacyLevel::Hybrid
        } else if has_confidential {
            if has_public_mixing {
                PrivacyLevel::Hybrid
            } else {
                PrivacyLevel::Confidential
            }
        } else if has_storage {
            if has_public_mixing {
                PrivacyLevel::Hybrid
            } else {
                PrivacyLevel::Compressed
            }
        } else {
            PrivacyLevel::Public
        };
    }
}
