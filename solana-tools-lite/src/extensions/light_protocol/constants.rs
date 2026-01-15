use crate::models::pubkey_base58::PubkeyBase58;
use std::sync::OnceLock;

/// Light Protocol programs (verified Jan 2026)
pub const LIGHT_SYSTEM_PROGRAM_ID: &str = "Lighton6oQpVkeewmo2mcPTQQp7kYHr4fWpAgJyEmDX";
pub const ACCOUNT_COMPRESSION_PROGRAM_ID: &str = "compr6CUsB5m2jS4Y3831ztGSTnDpnKJTKS95d64XVq";
pub const COMPRESSED_TOKEN_PROGRAM_ID: &str = "cTokenmWW8bLPjZEBAUgYy3zKxQZW6VKi7bqNFEVv3m";

pub const DISCRIMINATOR_SIZE: usize = 8;
pub const U64_SIZE: usize = 8;

/// Discriminators
pub const DISCRIMINATOR_CREATE_MINT: [u8; 8] = [69, 44, 215, 132, 253, 214, 41, 45];
pub const DISCRIMINATOR_MINT_TO: [u8; 8] = [241, 34, 48, 186, 37, 179, 123, 192];
pub const DISCRIMINATOR_TRANSFER: [u8; 8] = [163, 52, 200, 231, 140, 3, 69, 186];
pub const DISCRIMINATOR_COMPRESS_SOL: [u8; 8] = [101, 145, 17, 14, 113, 248, 178, 230];
pub const DISCRIMINATOR_COMPRESS_TOKEN: [u8; 8] = [145, 26, 238, 131, 177, 60, 60, 35];
pub const DISCRIMINATOR_DECOMPRESS: [u8; 8] = [74, 60, 49, 197, 18, 110, 93, 154];
pub const DISCRIMINATOR_STATE_UPDATE: [u8; 8] = [81, 156, 178, 100, 94, 144, 128, 20];
pub const DISCRIMINATOR_CLOSE_ACCOUNT: [u8; 8] = [125, 255, 149, 14, 110, 34, 72, 24];

static SUPPORTED_PROGRAMS: OnceLock<Vec<PubkeyBase58>> = OnceLock::new();

/// Returns the list of Light Protocol program IDs.
/// 
/// Uses lazy initialization with graceful error handling. If any program ID
/// fails to parse, it is skipped rather than causing a panic. This ensures
/// the system remains functional even if program IDs are updated incorrectly.
/// 
/// # Returns
/// 
/// A slice of valid Light Protocol program IDs. In normal operation, this
/// should contain 3 program IDs, but may contain fewer if some fail to parse.
pub fn supported_programs() -> &'static [PubkeyBase58] {
    SUPPORTED_PROGRAMS.get_or_init(|| {
        let mut programs = Vec::new();
        
        // Try to parse each program ID, skip invalid ones with graceful degradation
        if let Ok(pk) = PubkeyBase58::try_from(LIGHT_SYSTEM_PROGRAM_ID) {
            programs.push(pk);
        }
        if let Ok(pk) = PubkeyBase58::try_from(ACCOUNT_COMPRESSION_PROGRAM_ID) {
            programs.push(pk);
        }
        if let Ok(pk) = PubkeyBase58::try_from(COMPRESSED_TOKEN_PROGRAM_ID) {
            programs.push(pk);
        }
        
        programs
    })
}