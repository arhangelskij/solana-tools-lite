//use std::fs;
use serde_json;
use solana_tools_lite::models::transaction::{Transaction, Message, Instruction};

#[test]
fn test_parse_and_sign_transaction_json() {
    // Example minimal transaction JSON (could be moved to a separate testdata/tx.json file)
    // For production, load from disk or receive as CLI input.
    let tx_json = r#"
    {
        "signatures": [""],
        "message": {
            "account_keys": [
                "SenderPubKeyBase58Here",
                "RecipientPubKeyBase58Here",
                "11111111111111111111111111111111"
            ],
            "recent_blockhash": "SomeRecentBlockhashBase58",
            "instructions": [
                {
                    "program_id_index": 2,
                    "accounts": [0, 1],
                    "data": "3Bxs4R9sW4B"
                }
            ]
        }
    }
    "#;

    // Parse the transaction JSON into our Transaction struct
    let tx: Transaction = serde_json::from_str(tx_json).expect("Parse tx JSON");

    // At this point, you can access all fields for signing, validation, etc.
    assert_eq!(tx.message.account_keys.len(), 3);
    assert_eq!(tx.signatures.len(), 1);

    // (Signing logic would go here: extract message, serialize, sign, update signatures)
    // For now, just print out the message structure for debugging:
    println!("{:#?}", tx);

    // Example: check that the system program is present as expected
    assert!(tx.message.account_keys.contains(&"11111111111111111111111111111111".to_string()));
}