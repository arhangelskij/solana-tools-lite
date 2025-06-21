use bs58;

/// Encode bytes to Base58 string (prod).
pub fn encode(data: &[u8]) -> String {
    bs58::encode(data).into_string()
}

/// Decode Base58 string to bytes (prod).
pub fn decode(s: &str) -> anyhow::Result<Vec<u8>> {
    bs58::decode(s)
        .into_vec()
        .map_err(|e| anyhow::anyhow!("Base58 decode error: {}", e))
}