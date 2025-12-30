use solana_tools_lite::errors::ToolError;
use solana_tools_lite::handlers::base58;

#[test]
fn encode_decode_roundtrip() {
    let input = "hello world";
    let encoded = base58::encode(input).expect("encode");
    assert_eq!(encoded.action, "encode");
    assert_eq!(encoded.input, input);

    let decoded = base58::decode(&encoded.output).expect("decode");
    assert_eq!(decoded.action, "decode");
    assert_eq!(decoded.output, input);
}

#[test]
fn decode_invalid_base58_returns_error() {
    let err = base58::decode("!!!").expect_err("expected base58 error");
    match err {
        ToolError::Base58(_) => {}
        other => panic!("unexpected error: {other:?}"),
    }
}
