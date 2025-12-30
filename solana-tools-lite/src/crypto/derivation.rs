use crate::crypto::bip39::Seed;
use crate::errors::GenError;
use hmac::{Hmac, Mac};
use sha2::Sha512;
use std::fmt;
use std::str::FromStr;

type HmacSha512 = Hmac<Sha512>;

const ED25519_SEED: &[u8] = b"ed25519 seed";
const HARDENED_BIT: u32 = 0x8000_0000;
/// Standard Solana/Phantom derivation path (BIP44).
pub const SOLANA_DERIVATION_PATH: &str = "m/44'/501'/0'/0'";

pub struct DerivationPath {
    pub indexes: Vec<u32>,
}

impl DerivationPath {
    /// Parse standard derivation path string (e.g., "m/44'/501'/0'/0'")
    /// Supports ' and h as hardened markers.
    pub fn parse(path: &str) -> Result<Self, GenError> {
        let mut iter = path.split('/');

        match iter.next() {
            Some("m") => {}
            _ => {
                return Err(GenError::InvalidDerivationPath(
                    "Path must start with 'm'".into(),
                ))
            }
        }

        let mut indexes = Vec::new();

        for part in iter {
            if part.is_empty() {
                return Err(GenError::InvalidDerivationPath(
                    "Empty path segment".into(),
                ));
            }

            // Supports ' and h as hardened markers.
            let (num_str, hardened) = if let Some(s) = part.strip_suffix('\'') {
                (s, true)
            } else if let Some(s) = part.strip_suffix('h') {
                (s, true)
            } else {
                (part, false)
            };

            if num_str.is_empty() {
                return Err(GenError::InvalidDerivationPath(format!(
                    "Invalid index: {}",
                    part
                )));
            }

            let index: u32 = num_str.parse().map_err(|_| {
                GenError::InvalidDerivationPath(format!("Invalid index: {}", part))
            })?;

            if !hardened {
                return Err(GenError::InvalidDerivationPath(
                    "Only hardened derivation is supported for Ed25519".into(),
                ));
            }

            // Hardened derivation uses the high bit (0x8000_0000). Reject values that would overflow.
            if index >= HARDENED_BIT {
                return Err(GenError::InvalidDerivationPath(format!(
                    "Invalid hardened index: {}",
                    part
                )));
            }

            indexes.push(index | HARDENED_BIT);
        }

        Ok(Self { indexes })
    }
}

impl FromStr for DerivationPath {
    type Err = GenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        DerivationPath::parse(s)
    }
}

impl fmt::Display for DerivationPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "m")?;
        for idx in &self.indexes {
            if *idx >= HARDENED_BIT {
                write!(f, "/{}'", idx - HARDENED_BIT)?;
            } else {
                write!(f, "/{}", idx)?;
            }
        }
        Ok(())
    }
}

pub fn derive_key_from_seed(
    seed: &Seed,
    path: &DerivationPath,
) -> Result<([u8; 32], [u8; 32]), GenError> {
    // 1. Master Key Generation
    let mut mac = HmacSha512::new_from_slice(ED25519_SEED)
        .map_err(|_| GenError::CryptoError("HMAC initialization failed".into()))?;
    mac.update(seed.as_bytes());
    let result = mac.finalize().into_bytes();

    let mut key = [0u8; 32];
    let mut chain_code = [0u8; 32];
    key.copy_from_slice(&result[0..32]);
    chain_code.copy_from_slice(&result[32..64]);

    // 2. Child Key Derivation (CKD)
    for &index in &path.indexes {
        // Hardened derivation: 0x00 || key || index(be)
        let mut mac = HmacSha512::new_from_slice(&chain_code)
            .map_err(|_| GenError::CryptoError("HMAC initialization failed".into()))?;

        mac.update(&[0u8]);
        mac.update(&key);
        mac.update(&index.to_be_bytes());

        let result = mac.finalize().into_bytes();
        key.copy_from_slice(&result[0..32]);
        chain_code.copy_from_slice(&result[32..64]);
    }

    Ok((key, chain_code))
}
