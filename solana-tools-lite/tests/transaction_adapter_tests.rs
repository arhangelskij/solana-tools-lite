use solana_tools_lite::models::transaction::Transaction;
use solana_tools_lite::models::input_transaction::InputTransaction;
use std::fs;
use std::path::PathBuf;

#[path = "utils.rs"]
mod utils;
use utils::{generate_mock_pubkey, generate_mock_signature};


#[test]
fn test_adapter_with_generated_data() {
    let pubkey1 = generate_mock_pubkey();
    let pubkey2 = generate_mock_pubkey();
    let program_id = generate_mock_pubkey();
    let blockhash = generate_mock_pubkey();

    let signature = generate_mock_signature();

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/data/tx_template.json");
    let template = fs::read_to_string(path).expect("Template not found");

    let json = template
        .replace("$PK1", &pubkey1)
        .replace("$PK2", &pubkey2)
        .replace("$PROGID", &program_id)
        .replace("$BLOCKHASH", &blockhash)
        .replace("$SIG", &signature)
        .replace("$DATA", "3Bxs4SHffsqLHuC3");


    let parsed: InputTransaction = serde_json::from_str(&json).expect("failed to parse");
    let tx: Transaction = TryFrom::try_from(parsed).expect("conversion failed");

    assert_eq!(tx.signatures.len(), 1);
     assert_eq!(tx.message.account_keys.len(), 3);
    assert_eq!(tx.message.instructions.len(), 1);
}

#[test]
fn test_adapter_invalid_pubkey() {
    let input = r#"
    {
        "signatures": ["3prfupj2PMawf5PBYTnCaJzD1eBaFApgx2MkckXQoo7o4deNyEeeRzKA4JTqpXUWPfYw5PmHBQmUVHTba9vS3wXh"],
        "message": {
            "header": {
                "num_required_signatures": 1,
                "num_readonly_signed_accounts": 0,
                "num_readonly_unsigned_accounts": 1
            },
            "account_keys": ["INVALIDBASE58!"],
            "recent_blockhash": "3bsiZrwkE1FtgxeJtMrdBpBteYpXMHYutFSbLMVYEFH4",
            "instructions": []
        }
    }
    "#;

    let parsed = serde_json::from_str::<InputTransaction>(input);
    assert!(parsed.is_ok(), "InputTransaction JSON should be valid");

    let parsed = parsed.unwrap();
    let result: Result<Transaction, _> = TryFrom::try_from(parsed);

    assert!(result.is_err(), "Expected conversion to fail");
    let err = result.err().unwrap();
    assert!(
        err.to_string().to_lowercase().contains("invalid"),
        "Expected base58 decode error, got: {err}"
    );
}

#[test]
fn test_adapter_invalid_base64_input() {
    let invalid_base64 = InputTransaction::Base64("not@@base64$$".to_string());

    let result: Result<Transaction, _> = TryFrom::try_from(invalid_base64);

    assert!(result.is_err(), "Expected Base64 decoding to fail");
    let err = result.err().unwrap();
    assert!(
        err.to_string().to_lowercase().contains("base64"),
        "Expected base64 error, got: {err}"
    );
}

#[test]
fn test_adapter_invalid_base58_input() {
    let invalid_base58 = InputTransaction::Base58("not@@base58$$".to_string());

    let result: Result<Transaction, _> = TryFrom::try_from(invalid_base58);

    assert!(result.is_err(), "Expected Base58 decoding to fail");
    let err = result.err().unwrap();
    assert!(
        err.to_string().to_lowercase().contains("base58"),
        "Expected base58 error, got: {err}"
    );
}

//TODO: add more tests

///////////////////////////////////////////////