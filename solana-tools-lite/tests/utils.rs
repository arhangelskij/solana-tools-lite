use rand::RngCore;
use rand::rng;
use bs58;

/// Generates a valid 32-byte Base58 public key
pub fn generate_mock_pubkey() -> String {
    let mut bytes = [0u8; 32];
    let mut rng = rng();
    rng.fill_bytes(&mut bytes);
    bs58::encode(bytes).into_string()
}

/// Generates a valid 64-byte Base58 signature
pub fn generate_mock_signature() -> String {
    let mut bytes = [0u8; 64];
    let mut rng = rng();
    rng.fill_bytes(&mut bytes);
    bs58::encode(bytes).into_string()
}