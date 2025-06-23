use serde::Serialize;

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
    pub note: &'static str
}

/// Output for signing (sign command)
#[derive(Serialize)]
pub struct SignResult {
    pub message: String,
    pub signature_base58: String,
    pub public_key: String
}