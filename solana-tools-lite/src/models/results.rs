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
    #[serde(rename = "publicKey")]
    pub public_key: String,
    #[serde(rename = "secretKey")]
    pub secret_key: String,
    pub seed_hex: String
}

//TODO: 🔴 check if needed / looks like not
impl fmt::Display for GenResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Mnemonic: {}\nPublic Key: {}\nSecret Key: {}\nSeed Hex: {}",
            self.mnemonic,
            self.public_key,
            self.secret_key,
            self.seed_hex
        )
    }
}

impl GenResult {
    /// Returns a human-friendly display showing only mnemonic and public key.
    pub fn to_public_display(&self) -> String {
        format!("Public Key: {}", self.public_key)
    }

    /// Returns a pretty-printed JSON containing all generation result fields.
    pub fn to_full_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("Failed to serialize GenResult to JSON")
    }
}

/// Output for signing (sign command)
#[derive(Serialize, Debug)]
pub struct SignResult {
    pub message: String,
    pub signature_base58: String,
    pub public_key: String 
}
