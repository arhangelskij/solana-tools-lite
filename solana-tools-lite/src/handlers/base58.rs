use crate::errors::Result;
use crate::models::results::Base58Result;

/// Perform Base58 encode and return a structured result.
pub fn encode(data: &str) -> Result<Base58Result> {
    let encoded = bs58::encode(data.as_bytes()).into_string();

    Ok(Base58Result {
        action: "encode".into(),
        input: data.into(),
        output: encoded,
    })
}

/// Perform Base58 decode and return a structured result.
pub fn decode(encoded: &str) -> Result<Base58Result> {
    let bytes = bs58::decode(encoded).into_vec()?;
    let decoded = String::from_utf8_lossy(&bytes).into();

    Ok(Base58Result {
        action: "decode".into(),
        input: encoded.into(),
        output: decoded,
    })
}
