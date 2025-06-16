use ed25519_dalek::{SigningKey, Signature, Signer};
use anyhow::{Result, Context};
use std::convert::TryInto;

/// Signs a given message with a provided secret key (base58 encoded)
pub fn handle_sign(message: &str, secret_key_b58: &str) -> Result<String> {
    // Decode the base58 secret key
    let secret_bytes = bs58::decode(secret_key_b58)
        .into_vec()
        .context("Invalid base58 in secret key")?;

    // Convert to [u8; 32] (only the private seed part is needed)
    let secret_bytes_arr: &[u8; 32] = secret_bytes
        .as_slice()
        .try_into()
        .context("Secret key must be 32 bytes")?;

    // Create SigningKey from seed
    let signing_key = SigningKey::from_bytes(secret_bytes_arr);

    // Sign the message
    let signature: Signature = signing_key.sign(message.as_bytes());

    // Encode the signature in base58
    let signature_b58 = bs58::encode(signature.to_bytes()).into_string();

    Ok(signature_b58)
}