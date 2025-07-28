use std::error::Error;

use solana_tools_lite::handlers::base58::handle_base58;
use solana_tools_lite::models::cmds::Base58Action;

#[test]
fn test_encode_ok() -> Result<(), Box<dyn Error>> {
    let input = "hello world".to_string();
    let action = Base58Action::Encode { input };
    // We assert only that it succeeds (prints to stdout inside)
    handle_base58(&action)?;
    Ok(())
}

#[test]
fn test_decode_ok() -> Result<(), Box<dyn Error>> {
    // base58 for "hello world"
    let encoded = bs58::encode("hello world").into_string();
    let action = Base58Action::Decode { input: encoded };
    handle_base58(&action)?;
    Ok(())
}

#[test]
fn test_decode_invalid_err() {
    // Contains invalid base58 characters
    let bad = "0OIl+/=".to_string();
    let action = Base58Action::Decode { input: bad };
    let res = handle_base58(&action);
    assert!(res.is_err(), "expected error on invalid base58 input");
}