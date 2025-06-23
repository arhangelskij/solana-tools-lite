use anyhow::bail;
use anyhow::{Context, Result};
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use log::info;
use std::convert::TryInto;
use crate::models::results::VerifyResult;
use crate::utils::pretty_print_json;

const SIG_LEN: usize = 64;
const PUBKEY_LEN: usize = 32;

/// Verifies a base58-encoded signature against a message and base58-encoded public key
pub fn handle_verify(
    message: &str,
    signature_b58: &str,
    pubkey_b58: &str,
    json: bool,
) -> Result<()> {
    // Decode signature from base58
    let sig_bytes = bs58::decode(signature_b58)
        .into_vec()
        .context("Invalid base58 in signature")?;

    // Decode pubkey from base58
    let pubkey_bytes = bs58::decode(pubkey_b58)
        .into_vec()
        .context("Invalid base58 in public key")?;

    // Ensure the signature is exactly SIG_LEN bytes
    if sig_bytes.len() != SIG_LEN {
        bail!(
            "Signature must be exactly {} bytes, got {}",
            SIG_LEN,
            sig_bytes.len()
        );
    }

    // Ensure the public key is exactly PUBKEY_LEN bytes
    if pubkey_bytes.len() != PUBKEY_LEN {
        bail!(
            "Public key must be exactly {} bytes, got {}",
            PUBKEY_LEN,
            pubkey_bytes.len()
        );
    }

    // Convert to [u8; SIG_LEN] for Signature
    let sig_bytes_arr: &[u8; SIG_LEN] = sig_bytes
        .as_slice()
        .try_into()
        .context(format!("Signature must be {} bytes", SIG_LEN))?;
    let signature = Signature::from_bytes(sig_bytes_arr);

    // Convert to [u8; PUBKEY_LEN] for VerifyingKey
    let pubkey_bytes_arr: &[u8; PUBKEY_LEN] = pubkey_bytes
        .as_slice()
        .try_into()
        .context(format!("Public key must be {} bytes", PUBKEY_LEN))?;
    let pubkey = VerifyingKey::from_bytes(pubkey_bytes_arr);

    // Verify signature
    pubkey?.verify(message.as_bytes(), &signature).context("Signature verification failed")?;

    if json {
    let result = VerifyResult {
            message: message.to_string(),
            pubkey: pubkey_b58.to_string(),
            signature: signature.to_string(),
            valid: true,
            error: None 
        };
        pretty_print_json(&result);
    } else {
        info!("✅ Signature is valid");
    }

    Ok(())
}
