//use anyhow::bail;
use crate::models::results::VerifyResult;
use crate::utils::pretty_print_json;
use anyhow::{Result};
use ed25519_dalek::{Verifier};
use std::convert::TryInto;
use thiserror::Error;

const SIG_LEN: usize = 64;
const PUBKEY_LEN: usize = 32;

#[derive(Error, Debug)]
enum VerifyError {
    #[error("Base58 decode error: {0}")]
    Base58Decode(#[from] bs58::decode::Error),
    #[error("Invalid signature length: expected {SIG_LEN}, got {0}")]
    InvalidSignatureLength(usize),
    #[error("Invalid public key length: expected {PUBKEY_LEN}, got {0}")]
    InvalidPubkeyLength(usize),
    #[error("Invalid signature format")]
    InvalidSignatureFormat,
    #[error("Invalid public key format")]
    InvalidPubkeyFormat,
    #[error("Signature verification failed")]
    VerificationFailed,
}

fn verify_signature(
    message: &str,
    signature_b58: &str,
    pubkey_b58: &str,
) -> Result<(), VerifyError> {
    let sig_bytes = bs58::decode(signature_b58).into_vec()?;
    let pubkey_bytes = bs58::decode(pubkey_b58).into_vec()?;

    if sig_bytes.len() != SIG_LEN {
        return Err(VerifyError::InvalidSignatureLength(sig_bytes.len()));
    }
    if pubkey_bytes.len() != PUBKEY_LEN {
        return Err(VerifyError::InvalidPubkeyLength(pubkey_bytes.len()));
    }

    let sig_array: [u8; SIG_LEN] = sig_bytes
        .try_into()
        .map_err(|_| VerifyError::InvalidSignatureFormat)?;
    let pubkey_array: [u8; PUBKEY_LEN] = pubkey_bytes
        .try_into()
        .map_err(|_| VerifyError::InvalidPubkeyFormat)?;

    let signature = ed25519_dalek::Signature::from_bytes(&sig_array);
    let pubkey = ed25519_dalek::VerifyingKey::from_bytes(&pubkey_array)
        .map_err(|_| VerifyError::InvalidPubkeyFormat)?;

    pubkey
        .verify(message.as_bytes(), &signature)
        .map_err(|_| VerifyError::VerificationFailed)?;

    Ok(())
}

pub fn handle_verify(message: &str, signature_b58: &str, pubkey_b58: &str, json: bool) -> i32 {
    let result = match verify_signature(message, signature_b58, pubkey_b58) {
        Ok(()) => VerifyResult {
            message: message.to_string(),
            pubkey: pubkey_b58.to_string(),
            signature: signature_b58.to_string(),
            valid: true,
            error: None,
        },
        Err(e) => VerifyResult {
            message: message.to_string(),
            pubkey: pubkey_b58.to_string(),
            signature: signature_b58.to_string(),
            valid: false,
            error: Some(e.to_string()),
        },
    };

    if json {
        pretty_print_json(&result);
    } else if result.valid {
        println!("[✓] Signature is valid");
    } else {
        eprintln!(
            "[✗] Signature is invalid: {}",
            result.error.as_deref().unwrap_or("unknown error")
        );
    }

    if result.valid { 0 } else { 1 }
}

/*
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
*/
