use ed25519_dalek::{Signature, Signer, SigningKey};
use crate::models::results::SignResult;
use crate::errors::{Result};

/// Pure handler: sign a message with the provided SigningKey
pub fn handle(message: &str, signing_key: &SigningKey) -> Result<SignResult> {
    // Sign the message
    let signature: Signature = signing_key.sign(message.as_bytes());

    // Encode signature and public key in Base58
    let signature_b58 = bs58::encode(signature.to_bytes()).into_string();
    let pubkey_b58 = bs58::encode(signing_key.verifying_key().to_bytes()).into_string();

    Ok(SignResult {
        message: message.to_string(),
        signature_base58: signature_b58,
        public_key: pubkey_b58,
    })
}
