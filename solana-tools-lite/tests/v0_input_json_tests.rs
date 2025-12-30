use solana_tools_lite::models::input_transaction::InputTransaction;
use solana_tools_lite::models::transaction::Transaction;
use std::convert::TryInto;

#[test]
fn test_parse_v0_transaction_json_with_lookups() {
    let json_v0 = r#"{
        "signatures": ["1111111111111111111111111111111111111111111111111111111111111111"],
        "message": {
            "header": {
                "num_required_signatures": 1,
                "num_readonly_signed_accounts": 0,
                "num_readonly_unsigned_accounts": 0
            },
            "accountKeys": [
                "11111111111111111111111111111111"
            ],
            "recentBlockhash": "11111111111111111111111111111111",
            "instructions": [
                {
                    "programIdIndex": 0,
                    "accounts": [],
                    "data": "11"
                }
            ],
            "addressTableLookups": [
                {
                    "accountKey": "AddressLookupTab1e1111111111111111111111111",
                    "writableIndexes": [1],
                    "readonlyIndexes": [2]
                }
            ]
        }
    }"#;

    // 1. Deserialization: JSON -> InputTransaction
    let input: InputTransaction = serde_json::from_str(json_v0).expect("failed to parse V0 JSON");

    // 2. Verification: InputTransaction -> UiTransaction
    let ui_tx = match input {
        InputTransaction::Json(ui) => ui,
        _ => panic!("Expected InputTransaction::Json variant"),
    };

    // 3. Verification: Check V0-specific field presence via converting to Rust Transaction
    // (This rigorously tests the mapping logic in `input_transaction.rs`)
    let tx: Transaction = ui_tx
        .try_into()
        .expect("failed to convert to internal Transaction");

    // Assert it resulted in a V0 message variant
    match tx.message {
        solana_tools_lite::models::message::Message::V0(msg) => {
            assert_eq!(msg.address_table_lookups.len(), 1);
            assert_eq!(
                msg.address_table_lookups[0].account_key.to_string(),
                "AddressLookupTab1e1111111111111111111111111"
            );
            assert_eq!(msg.address_table_lookups[0].writable_indexes, vec![1]);
            assert_eq!(msg.address_table_lookups[0].readonly_indexes, vec![2]);
        }
        _ => panic!("Parsed transaction ended up as Legacy, expected V0"),
    }
}
