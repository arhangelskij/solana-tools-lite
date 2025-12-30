use crate::crypto::signing::sign_message;
use crate::errors::Result;
use crate::models::results::SignResult;
use ed25519_dalek::{Signature, SigningKey};

/// Pure handler: sign a message with the provided SigningKey
pub fn handle(message: &str, signing_key: &SigningKey) -> Result<SignResult> {
    // Sign the message
    let signature: Signature = sign_message(signing_key, message.as_bytes());

    // Encode signature and public key in Base58
    let signature_b58 = bs58::encode(signature.to_bytes()).into_string();
    let pubkey_b58 = bs58::encode(signing_key.verifying_key().to_bytes()).into_string();

    Ok(SignResult {
        message: message.to_string(),
        signature_base58: signature_b58,
        public_key: pubkey_b58,
    })
}
