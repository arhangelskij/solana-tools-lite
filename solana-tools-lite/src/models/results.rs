use serde::Serialize;
use serde_json;
use std::fmt;

/// Output for signature verification (verify command)
#[derive(Serialize)]
pub struct VerifyResult {
    pub message: String,
    pub pubkey: String,
    pub signature: String,
    pub valid: bool,
    pub error: Option<String>
}

/// Output for keypair generation (gen command)
#[derive(Serialize)]
pub struct GenResult {
    pub mnemonic: String,
    pub public_key_base58: String,
    pub secret_key_base58: String,
    pub seed_hex: String,
}

#[derive(Serialize)]
pub struct PublicGenResult {
    pub mnemonic: String,
    pub public_key_base58: String
}

impl fmt::Display for PublicGenResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Mnemonic: {}\nPublic Key: {}",
            self.mnemonic,
            self.public_key_base58
        )
    }
}

impl GenResult {
    pub fn as_public(&self) -> PublicGenResult {
        PublicGenResult {
            mnemonic: self.mnemonic.clone(),
            public_key_base58: self.public_key_base58.clone(),
        }
    }

    /// Returns a human-friendly display showing only mnemonic and public key.
    pub fn to_public_display(&self) -> String {
        format!(
            "Mnemonic: {}\nPublic Key: {}",
            self.mnemonic, self.public_key_base58
        )
    }

    /// Returns a pretty-printed JSON containing all generation result fields.
    pub fn to_full_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("Failed to serialize GenResult to JSON")
    }
}

/// Output for signing (sign command)
#[derive(Serialize)]
pub struct SignResult {
    pub message: String,
    pub signature_base58: String,
    pub public_key: String, //TODO: add - pub error: Option<String> ?
}
