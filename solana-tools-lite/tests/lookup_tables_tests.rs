use solana_tools_lite::handlers::analysis::parse_lookup_tables;
use solana_tools_lite::models::pubkey_base58::PubkeyBase58;
use std::collections::HashMap;

#[test]
fn parse_lookup_tables_valid_json() {
    let json = r#"{
        "11111111111111111111111111111111": [
            "ComputeBudget111111111111111111111111111111",
            "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        ]
    }"#;

    let tables = parse_lookup_tables(json).expect("must parse");
    let key = PubkeyBase58::try_from("11111111111111111111111111111111").unwrap();
    let addr1 =
        PubkeyBase58::try_from("ComputeBudget111111111111111111111111111111").unwrap();
    let addr2 =
        PubkeyBase58::try_from("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();

    let mut expected = HashMap::new();
    expected.insert(key, vec![addr1, addr2]);
    assert_eq!(tables, expected);
}

#[test]
fn parse_lookup_tables_invalid_json() {
    let err = parse_lookup_tables("{ not json }").unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("invalid lookup tables JSON"));
}

#[test]
fn parse_lookup_tables_invalid_key() {
    let json = r#"{"not-base58": ["11111111111111111111111111111111"]}"#;
    let err = parse_lookup_tables(json).unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("invalid lookup table key"));
}
