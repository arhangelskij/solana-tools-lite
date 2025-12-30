use ed25519_dalek::SigningKey;
use std::convert::TryInto;

use crate::constants::crypto::SEED_LEN;
use crate::errors::SignError;
use crate::models::keypair_json::KeypairJson;

/// Build SigningKey from decoded bytes: accept 32-byte seed or 64-byte keypair bytes.
pub fn signing_key_from_decoded(bytes: Vec<u8>) -> Result<SigningKey, SignError> {
    match bytes.len() {
        64 => {
            let mut seed = [0u8; SEED_LEN];
            seed.copy_from_slice(&bytes[..32]);
            Ok(SigningKey::from_bytes(&seed))
        }
        32 => {
            let arr: [u8; SEED_LEN] = bytes
                .as_slice()
                .try_into()
                .map_err(|_| SignError::InvalidKeyLength)?;
            Ok(SigningKey::from_bytes(&arr))
        }
        _ => Err(SignError::InvalidKeyLength),
    }
}

/// Parse signing key from content string (no I/O).
/// Supported formats:
/// 1) JSON array of bytes (32 or 64)
/// 2) Keypair JSON with Base58 secretKey
/// 3) Raw Base58 string
pub fn parse_signing_key_content(content: &str) -> Result<SigningKey, SignError> {
    let text = content.trim();

    // 1) JSON array of bytes
    if let Ok(arr) = serde_json::from_str::<Vec<u8>>(text) {
        return match arr.len() {
            64 => {
                let mut seed = [0u8; SEED_LEN];
                seed.copy_from_slice(&arr[..SEED_LEN]);
                Ok(SigningKey::from_bytes(&seed))
            }
            32 => {
                let mut seed = [0u8; SEED_LEN];
                seed.copy_from_slice(&arr[..SEED_LEN]);
                Ok(SigningKey::from_bytes(&seed))
            }
            _ => Err(SignError::InvalidKeyLength),
        };
    }

    // 2) Keypair JSON
    if let Ok(kp_json) = serde_json::from_str::<KeypairJson>(text) {
        let sec = kp_json.secret_key.trim();
        let bytes = bs58::decode(sec)
            .into_vec()
            .map_err(|_| SignError::InvalidBase58)?;
        return signing_key_from_decoded(bytes);
    }

    // 3) Raw Base58
    let decoded = bs58::decode(text)
        .into_vec()
        .map_err(|_| SignError::InvalidBase58)?;
    signing_key_from_decoded(decoded)
}
