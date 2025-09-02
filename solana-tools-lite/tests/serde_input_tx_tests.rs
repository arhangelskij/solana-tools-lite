use solana_tools_lite::serde::input_tx::{parse_input_transaction, is_base64 as is_b64, is_base58 as is_b58};
use solana_tools_lite::models::input_transaction::InputTransaction;

#[test]
fn test_parse_input_transaction_json_ok() {
    let json = r#"{
        "signatures": [],
        "message": {
            "header": {
                "num_required_signatures": 1,
                "num_readonly_signed_accounts": 0,
                "num_readonly_unsigned_accounts": 1
            },
            "account_keys": [
                "11111111111111111111111111111111",
                "11111111111111111111111111111111"
            ],
            "recent_blockhash": "11111111111111111111111111111111",
            "instructions": []
        }
    }"#;
    let v = parse_input_transaction(Some(json)).expect("json parse");
    matches!(v, InputTransaction::Json(_));
}

#[test]
fn test_parse_input_transaction_base64_ok() {
    let b64 = data_encoding::BASE64.encode(b"payload");
    let v = parse_input_transaction(Some(&b64)).expect("b64 parse");
    matches!(v, InputTransaction::Base64(_));
}

#[test]
fn test_parse_input_transaction_base58_ok() {
    let b58 = bs58::encode(b"payload").into_string();
    let v = parse_input_transaction(Some(&b58)).expect("b58 parse");
    matches!(v, InputTransaction::Base58(_));
}

#[test]
fn test_parse_input_transaction_invalid_format() {
    let bad = "@@not a tx@@";
    let err = parse_input_transaction(Some(bad)).unwrap_err();
    assert!(matches!(err, solana_tools_lite::errors::TransactionParseError::InvalidFormat(_)));
}

#[test]
fn test_codecs_helpers_trim_and_nonempty() {
    assert!(is_b64("QUJD\n")); // ABC
    assert!(is_b58(&bs58::encode(b"X").into_string()));
    assert!(!is_b64("   "));
    assert!(!is_b58("   "));
}

