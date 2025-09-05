use ed25519_dalek::{Signature, Signer, SigningKey};
use crate::models::results::SignResult;
use crate::errors::{Result};
use crate::adapters::io_adapter::{read_and_parse_secret_key};

//TODO: 🟡🟠 test new execute
/// Signs a message using a secret key loaded from a file or stdin ("-").
pub fn handle(message: &str, secret_key_path: &str) -> Result<SignResult> {
    // Read secret key text and parse supported formats
    let signing_key = read_and_parse_secret_key(secret_key_path)?;
    handle_with_key(message, &signing_key)
}

/// Pure handler: sign a message with the provided SigningKey (no I/O).
pub fn handle_with_key(message: &str, signing_key: &SigningKey) -> Result<SignResult> {
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
