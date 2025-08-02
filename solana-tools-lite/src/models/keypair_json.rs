use serde::Deserialize;

/// Minimal DTO for Solana keypair JSON with Base58 secretKey string.
#[derive(Debug, Deserialize)]
pub struct KeypairJson {
    #[serde(rename = "publicKey")]
    pub public_key: String,
    #[serde(rename = "secretKey")]
    pub secret_key: String
}