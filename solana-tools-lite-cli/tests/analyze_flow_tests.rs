use solana_tools_lite::models::input_transaction::{
    InputTransaction, UiRawMessage, UiRawMessageV0, UiTransaction, UiCompiledInstruction,
};
use solana_tools_lite::models::message::MessageHeader;
use solana_tools_lite::bs58;
use solana_tools_lite::crypto::signing::SigningKey;
use solana_tools_lite_cli::flows::analyze;
use std::fs;
use tempfile::TempDir;

fn build_v0_tx_json(signer_pk: &str) -> String {
    let header = MessageHeader {
        num_required_signatures: 1,
        num_readonly_signed_accounts: 0,
        num_readonly_unsigned_accounts: 1,
    };
    let blockhash = bs58::encode([9u8; 32]).into_string();
    let sig_placeholder = bs58::encode([0u8; 64]).into_string();

    let mut data = Vec::new();
    data.extend_from_slice(&2u32.to_le_bytes());
    data.extend_from_slice(&1_000u64.to_le_bytes());
    let data_b58 = bs58::encode(&data).into_string();

    let ui_tx = UiTransaction {
        signatures: vec![sig_placeholder],
        message: UiRawMessage::V0(UiRawMessageV0 {
            header,
            account_keys: vec![
                signer_pk.to_string(),
                "11111111111111111111111111111111".to_string(), // system program
            ],
            recent_blockhash: blockhash,
            instructions: vec![UiCompiledInstruction {
                program_id_index: 1,
                accounts: vec![0, 1], // to self
                data: data_b58,
            }],
            address_table_lookups: vec![],
        }),
    };

    serde_json::to_string_pretty(&InputTransaction::Json(ui_tx)).unwrap()
}

#[test]
fn analyze_flow_reads_transaction_and_outputs_summary() {
    // This test verifies the components used by analyze::execute working together.
    // Instead of spawning a process, we invoke the library components directly.
    
    let dir = TempDir::new().unwrap();
    let key = SigningKey::from_bytes(&[1u8; 32]);
    let signer_pk = bs58::encode(key.verifying_key().to_bytes()).into_string();

    let tx_json = build_v0_tx_json(&signer_pk);
    let tx_path = dir.path().join("tx.json");
    fs::write(&tx_path, tx_json).unwrap();
    
    // 1. Simulate reading input
    let input_tx = solana_tools_lite::adapters::io_adapter::read_input_transaction(Some(tx_path.to_str().unwrap()))
        .expect("should read input");
        
    // 2. Simulate conversion
    let tx = solana_tools_lite::models::Transaction::try_from(input_tx).expect("should convert");
    
    // 3. Simulate analysis
    let signer = solana_tools_lite::models::PubkeyBase58::try_from(signer_pk.as_str()).unwrap();
    // Use the function as it exists in production (without signer arg if I reverted correctly)
    // Wait, Step 10750 shows analyze_transaction DOES accept signer: 
    // pub fn analyze_transaction(message: &Message, signer: &PubkeyBase58, tables: Option<&LookupTableEntry>) -> TxAnalysis
    let analysis = solana_tools_lite::handlers::analysis::analyze_transaction(&tx.message, &signer, None);
    
    // 4. Simulate summary generation
    // Step 10750 shows build_signing_summary DOES NOT accept signer (it was reverted):
    // pub fn build_signing_summary(tx: &Transaction, analysis: &TxAnalysis) -> Result<SigningSummary, ToolError>
    let summary = solana_tools_lite::handlers::analysis::build_signing_summary(&tx, &analysis)
        .expect("should build summary");
        
    let output = serde_json::to_string(&summary).unwrap();

    // Verify output contains expected fields
    assert!(output.contains("base_fee_lamports"), "output should contain base_fee");
    assert!(output.contains("total_sol_send_by_signer"), "output should contain sol_send");
    
    // NOTE: signer_pk is not in SigningSummary currently, so we don't assert it.
}

#[test]
fn analyze_flow_execution_smoke_test() {
    // Verifies that analyze::execute runs without panicking on valid input
    let dir = TempDir::new().unwrap();
    let key = SigningKey::from_bytes(&[1u8; 32]);
    let signer_pk = bs58::encode(key.verifying_key().to_bytes()).into_string();

    let tx_json = build_v0_tx_json(&signer_pk);
    let tx_path = dir.path().join("tx.json");
    fs::write(&tx_path, tx_json).unwrap();
    
    analyze::execute(
        Some(tx_path.to_str().unwrap()),
        None,
        None,
        true, // summary_json = true
    ).expect("analyze flow should succeed without panic");
}
