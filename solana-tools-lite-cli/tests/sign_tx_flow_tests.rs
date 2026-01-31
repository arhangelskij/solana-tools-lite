use solana_tools_lite::models::input_transaction::{
    InputTransaction, UiAddressTableLookup, UiCompiledInstruction, UiRawMessage, UiRawMessageV0,
    UiTransaction,
};
use solana_tools_lite::models::message::MessageHeader;
use solana_tools_lite::bs58;
use solana_tools_lite::crypto::signing::SigningKey;
use solana_tools_lite_cli::flows::sign_tx;
use std::fs;
use tempfile::TempDir;

fn write_keypair_file(dir: &TempDir) -> String {
    let key = SigningKey::from_bytes(&[1u8; 32]);
    let key_bytes = key.to_bytes();
    let content = serde_json::to_string(&key_bytes).unwrap();
    let path = dir.path().join("keypair.json");
    fs::write(&path, content).unwrap();
    path.to_string_lossy().to_string()
}

fn write_tables_file(dir: &TempDir, table_key: &str, addrs: &[String]) -> String {
    use std::collections::HashMap;
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    map.insert(table_key.to_string(), addrs.to_vec());
    let content = serde_json::to_string_pretty(&map).unwrap();
    let path = dir.path().join("tables.json");
    fs::write(&path, content).unwrap();
    path.to_string_lossy().to_string()
}

fn build_v0_tx_json(signer_pk: &str, table_key: &str) -> String {
    let header = MessageHeader {
        num_required_signatures: 1,
        num_readonly_signed_accounts: 0,
        num_readonly_unsigned_accounts: 1,
    };
    let blockhash = bs58::encode([9u8; 32]).into_string();
    let sig_placeholder = bs58::encode([0u8; 64]).into_string();

    // System transfer: kind=2 LE + lamports u64 LE
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
                accounts: vec![0, 2], // from signer, to looked-up writable
                data: data_b58,
            }],
            address_table_lookups: vec![UiAddressTableLookup {
                account_key: table_key.to_string(),
                writable_indexes: vec![0],
                readonly_indexes: vec![],
            }],
        }),
    };

    serde_json::to_string_pretty(&InputTransaction::Json(ui_tx)).unwrap()
}

#[test]
fn sign_tx_with_tables_and_assume_yes() {
    let dir = TempDir::new().unwrap();
    let key = SigningKey::from_bytes(&[1u8; 32]);
    let signer_pk = bs58::encode(key.verifying_key().to_bytes()).into_string();

    let table_key = bs58::encode([7u8; 32]).into_string();
    let lookup_addr = bs58::encode([8u8; 32]).into_string();

    let tx_json = build_v0_tx_json(&signer_pk, &table_key);
    let tx_path = dir.path().join("tx.json");
    fs::write(&tx_path, tx_json).unwrap();

    let tables_path = write_tables_file(&dir, &table_key, &[lookup_addr]);
    let keypair_path = write_keypair_file(&dir);
    let output_path = dir.path().join("signed.b64");

    sign_tx::execute(
        Some(tx_path.to_str().unwrap()),
        &keypair_path,
        Some(output_path.to_str().unwrap()),
        false, // json pretty
        None,  // output format mirror
        false, // force
        Some(tables_path.as_str()),
        true,         // assume_yes
        Some(10_000), // max_fee above base fee
        false,        // summary_json
    )
    .expect("signing should succeed");

    let signed = fs::read_to_string(&output_path).unwrap();
    assert!(!signed.is_empty());
}

#[test]
fn sign_tx_max_fee_exceeded_errors() {
    let dir = TempDir::new().unwrap();
    let keypair_path = write_keypair_file(&dir);
    let signer = SigningKey::from_bytes(&[1u8; 32]);
    let signer_pk = bs58::encode(signer.verifying_key().to_bytes()).into_string();

    let table_key = bs58::encode([7u8; 32]).into_string();

    let tx_json = build_v0_tx_json(&signer_pk, &table_key);
    let tx_path = dir.path().join("tx.json");
    fs::write(&tx_path, tx_json).unwrap();

    let err = sign_tx::execute(
        Some(tx_path.to_str().unwrap()),
        &keypair_path,
        None,
        false,
        None,
        false,
        None,
        true,
        Some(1), // too low for base fee
        false,
    )
    .err()
    .expect("should error on fee limit");

    assert!(format!("{err}").contains("exceeds max-fee"));
}

#[test]
fn summary_json_requires_output_path() {
    let err = sign_tx::execute(
        Some("tx.json"), // won't be read because validation happens first
        "wallet.json",
        None,
        false,
        None,
        false,
        None,
        true,
        None,
        true, // summary_json
    )
    .err()
    .expect("must reject summary-json without output");

    assert!(format!("{err}").contains("--summary-json requires --output"));
}
