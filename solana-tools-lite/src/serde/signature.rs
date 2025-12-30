use ed25519_dalek::Signature;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Serialize a list of signatures as Base58 strings.
pub fn serialize<S>(sigs: &Vec<Signature>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let encoded: Vec<String> = sigs
        .iter()
        .map(|s| bs58::encode(s.to_bytes()).into_string())
        .collect();
    encoded.serialize(serializer)
}

/// Deserialize a list of signatures from Base58 strings.
pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Signature>, D::Error>
where
    D: Deserializer<'de>,
{
    let encoded: Vec<String> = Deserialize::deserialize(deserializer)?;
    encoded
        .into_iter()
        .map(|s| {
            let bytes = bs58::decode(&s)
                .into_vec()
                .map_err(serde::de::Error::custom)?;
            if bytes.len() != 64 {
                return Err(serde::de::Error::custom("Invalid signature length"));
            }
            let mut raw = [0u8; 64];
            raw.copy_from_slice(&bytes);
            Ok(Signature::from_bytes(&raw))
        })
        .collect()
}
