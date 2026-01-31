use solana_tools_lite::serde::parse_lookup_tables;
use solana_tools_lite::models::pubkey_base58::PubkeyBase58;

#[test]
fn parse_lookup_tables_valid_json() {
    let json = r#"{
        "writable": [],
        "readonly": [
            "ComputeBudget111111111111111111111111111111",
            "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        ]
    }"#;
    
    let tables = parse_lookup_tables(json).expect("must parse");
    let addr1 =
        PubkeyBase58::try_from("ComputeBudget111111111111111111111111111111").unwrap();
    let addr2 =
        PubkeyBase58::try_from("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();

    let expected = solana_tools_lite::serde::LookupTableEntry {
        writable: vec![],
        readonly: vec![addr1, addr2],
    };
    
    assert_eq!(tables, expected);
}

#[test]
fn parse_lookup_tables_invalid_json() {
    let err = parse_lookup_tables("{ not json }").unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("invalid lookup tables JSON"));
}

#[test]
fn parse_lookup_tables_invalid_address() {
    let json = r#"{"writable": ["not-base58"]}"#;
    let err = parse_lookup_tables(json).unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("invalid writable address"));
}
