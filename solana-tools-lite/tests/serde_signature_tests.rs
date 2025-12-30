use ed25519_dalek::Signature;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct SigList {
    #[serde(with = "solana_tools_lite::serde::signature")]
    sigs: Vec<Signature>,
}

#[test]
fn signatures_roundtrip_via_json() {
    let s1 = Signature::from_bytes(&[1u8; 64]);
    let s2 = Signature::from_bytes(&[2u8; 64]);

    let payload = SigList { sigs: vec![s1, s2] };
    let json = serde_json::to_string(&payload).expect("serialize");
    let decoded: SigList = serde_json::from_str(&json).expect("deserialize");

    assert_eq!(decoded.sigs.len(), 2);
    assert_eq!(decoded.sigs[0].to_bytes(), [1u8; 64]);
    assert_eq!(decoded.sigs[1].to_bytes(), [2u8; 64]);
}

#[test]
fn signatures_invalid_length_errors() {
    let bad = bs58::encode(vec![9u8; 10]).into_string();
    let json = format!(r#"{{"sigs":["{bad}"]}}"#);
    let result: Result<SigList, _> = serde_json::from_str(&json);
    assert!(result.is_err(), "expected invalid signature length error");
}

#[test]
fn signatures_invalid_base58_errors() {
    let json = r#"{"sigs":["!!!"]}"#;
    let result: Result<SigList, _> = serde_json::from_str(json);
    assert!(result.is_err(), "expected invalid base58 error");
}
