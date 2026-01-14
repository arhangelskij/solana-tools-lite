use crate::models::extensions::AnalysisExtensionAction;
use crate::models::pubkey_base58::PubkeyBase58;
use serde::Serialize;

/// Transaction analysis output used by CLI and other front-ends.
#[derive(Debug, Clone)]
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
    /// Aggregated count of confidential (ZK) operations.
    pub confidential_ops_count: usize,
    /// Aggregated count of storage compression (public bridge) operations.
    pub storage_ops_count: usize,
    /// Whether the analyzed signer is the fee payer for this transaction.
    pub is_fee_payer: bool,
    /// Whether non-SOL assets (SPL/Token-2022) are involved in movement.
    pub has_non_sol_assets: bool,
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
    pub extension_actions: Vec<AnalysisExtensionAction>,
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
