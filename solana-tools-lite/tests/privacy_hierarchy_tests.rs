use solana_tools_lite::models::analysis::{PrivacyLevel, TxAnalysis, TransferView};
use solana_tools_lite::models::extensions::{AnalysisExtensionAction, LightProtocolAction, PrivacyImpact};

fn empty_analysis() -> TxAnalysis {
    TxAnalysis {
        transfers: vec![],
        base_fee_lamports: 0,
        priority_fee_lamports: None,
        total_fee_lamports: 0,
        total_sol_send_by_signer: 0,
        compute_unit_limit: None,
        compute_unit_price_micro: None,
        warnings: vec![],
        message_version: "legacy",
        privacy_level: PrivacyLevel::Public,
        extension_actions: vec![],
        extension_notices: vec![],
        confidential_ops_count: 0,
        storage_ops_count: 0,
        is_fee_payer: false,
        has_non_sol_assets: false,
    }
}

#[test]
fn test_privacy_hierarchy_pure_confidential() {
    let mut analysis = empty_analysis();
    analysis.confidential_ops_count = 1;
    analysis.recalculate_privacy_level();
    assert_eq!(analysis.privacy_level, PrivacyLevel::Confidential);
}

#[test]
fn test_privacy_hierarchy_pure_compressed() {
    let mut analysis = empty_analysis();
    analysis.storage_ops_count = 1;
    analysis.recalculate_privacy_level();
    assert_eq!(analysis.privacy_level, PrivacyLevel::Compressed);
}

#[test]
fn test_privacy_hierarchy_hybrid_mixed_confidential() {
    let mut analysis = empty_analysis();
    analysis.confidential_ops_count = 1;
    // Add a public transfer
    analysis.transfers.push(TransferView {
        from: "A".to_string(),
        to: "B".to_string(),
        lamports: 1000,
        from_is_signer: true,
    });
    analysis.recalculate_privacy_level();
    assert_eq!(analysis.privacy_level, PrivacyLevel::Hybrid);
}

#[test]
fn test_privacy_hierarchy_hybrid_bridge_exit() {
    let mut analysis = empty_analysis();
    // Decompress is a Hybrid impact action
    analysis.extension_actions.push(AnalysisExtensionAction::LightProtocol(
        LightProtocolAction::Decompress
    ));
    analysis.recalculate_privacy_level();
    assert_eq!(analysis.privacy_level, PrivacyLevel::Hybrid);
}

#[test]
fn test_privacy_hierarchy_public_only() {
    let mut analysis = empty_analysis();
    analysis.transfers.push(TransferView {
        from: "A".to_string(),
        to: "B".to_string(),
        lamports: 1000,
        from_is_signer: true,
    });
    analysis.recalculate_privacy_level();
    assert_eq!(analysis.privacy_level, PrivacyLevel::Public);
}

#[test]
fn test_privacy_hierarchy_confidential_takes_precedence_over_storage() {
    let mut analysis = empty_analysis();
    analysis.confidential_ops_count = 1;
    analysis.storage_ops_count = 1;
    analysis.recalculate_privacy_level();
    // No public mixing, so it's Confidential
    assert_eq!(analysis.privacy_level, PrivacyLevel::Confidential);
}
