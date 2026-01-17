/// Light Protocol constants and program identifiers.
/// 
/// This module contains all the program IDs, discriminators, and other constants
/// used by the Light Protocol (ZK Compression) on Solana. These values are used
/// to identify and parse Light Protocol instructions.

use crate::models::pubkey_base58::PubkeyBase58;
use crate::ToolError;
use std::sync::OnceLock;

/// Light Protocol system program ID (verified Jan 2026).
/// 
/// This program handles core Light Protocol operations including SOL compression
/// and system-level state management.
pub const LIGHT_SYSTEM_PROGRAM_ID: &str = "Lighton6oQpVkeewmo2mcPTQQp7kYHr4fWpAgJyEmDX";

/// Account compression program ID (verified Jan 2026).
/// 
/// This program manages the compressed account state and merkle tree operations
/// that enable ZK compression functionality.
pub const ACCOUNT_COMPRESSION_PROGRAM_ID: &str = "compr6CUsB5m2jS4Y3831ztGSTnDpnKJTKS95d64XVq";

/// Compressed token program ID (verified Jan 2026).
/// 
/// This program handles compressed SPL token operations including minting,
/// transferring, and managing compressed token accounts.
pub const COMPRESSED_TOKEN_PROGRAM_ID: &str = "cTokenmWW8bLPjZEBAUgYy3zKxQZW6VKi7bqNFEVv3m";

/// Size of instruction discriminator in bytes.
/// 
/// Light Protocol instructions use 8-byte discriminators to identify
/// the specific operation being performed.
pub const DISCRIMINATOR_SIZE: usize = 8;

/// Size of u64 value in bytes.
/// 
/// Used for parsing amounts, lamports, and other numeric values
/// from instruction data.
pub const U64_SIZE: usize = 8;

/// Light Protocol instruction discriminators.
/// 
/// These 8-byte arrays uniquely identify different Light Protocol operations.
/// They are derived from the instruction method names using anchor's discriminator
/// generation process.

/// Discriminator for CreateMint instruction.
/// Creates a new compressed token mint.
pub const DISCRIMINATOR_CREATE_MINT: [u8; 8] = [69, 44, 215, 132, 253, 214, 41, 45];

/// Discriminator for MintTo instruction.
/// Mints compressed tokens to a recipient.
pub const DISCRIMINATOR_MINT_TO: [u8; 8] = [241, 34, 48, 186, 37, 179, 123, 192];

/// Discriminator for Transfer instruction.
/// Transfers compressed tokens between accounts.
pub const DISCRIMINATOR_TRANSFER: [u8; 8] = [163, 52, 200, 231, 140, 3, 69, 186];

/// Discriminator for CompressSol instruction.
/// Compresses regular SOL into compressed SOL.
pub const DISCRIMINATOR_COMPRESS_SOL: [u8; 8] = [101, 145, 17, 14, 113, 248, 178, 230];

/// Discriminator for CompressToken instruction.
/// Compresses regular SPL tokens into compressed tokens.
pub const DISCRIMINATOR_COMPRESS_TOKEN: [u8; 8] = [145, 26, 238, 131, 177, 60, 60, 35];

/// Discriminator for Decompress instruction.
/// Decompresses compressed assets back to regular form.
pub const DISCRIMINATOR_DECOMPRESS: [u8; 8] = [74, 60, 49, 197, 18, 110, 93, 154];

/// Discriminator for StateUpdate instruction.
/// Updates compressed state or merkle tree data.
pub const DISCRIMINATOR_STATE_UPDATE: [u8; 8] = [81, 156, 178, 100, 94, 144, 128, 20];

/// Discriminator for CloseAccount instruction.
/// Closes a compressed account and reclaims rent.
pub const DISCRIMINATOR_CLOSE_ACCOUNT: [u8; 8] = [125, 255, 149, 14, 110, 34, 72, 24];

static SUPPORTED_PROGRAMS: OnceLock<Result<[PubkeyBase58; 3], ToolError>> = OnceLock::new();

/// Returns the list of Light Protocol program IDs.
/// 
/// Uses lazy initialization with proper error handling. If any program ID
/// fails to parse, returns a ToolError instead of panicking. This ensures
/// robust error handling while maintaining type safety.
/// 
/// # Returns
/// 
/// `Ok(&[PubkeyBase58; 3])` containing exactly three Light Protocol program IDs,
/// or `Err(&ToolError)` if any program ID fails to parse.
/// 
/// # Errors
/// 
/// Returns `ToolError::ConfigurationError` if any of the hardcoded program ID strings
/// cannot be parsed as valid base58 public keys. This indicates a bug in the code
/// rather than user error.
pub fn supported_programs() -> Result<&'static [PubkeyBase58; 3], &'static ToolError> {
    SUPPORTED_PROGRAMS.get_or_init(|| {
        let light_system = PubkeyBase58::try_from(LIGHT_SYSTEM_PROGRAM_ID)
            .map_err(|e| ToolError::ConfigurationError(format!("Hardcoded LIGHT_SYSTEM_PROGRAM_ID is invalid: {}", e)))?;
        
        let account_compression = PubkeyBase58::try_from(ACCOUNT_COMPRESSION_PROGRAM_ID)
            .map_err(|e| ToolError::ConfigurationError(format!("Hardcoded ACCOUNT_COMPRESSION_PROGRAM_ID is invalid: {}", e)))?;
        
        let compressed_token = PubkeyBase58::try_from(COMPRESSED_TOKEN_PROGRAM_ID)
            .map_err(|e| ToolError::ConfigurationError(format!("Hardcoded COMPRESSED_TOKEN_PROGRAM_ID is invalid: {}", e)))?;
        
        Ok([light_system, account_compression, compressed_token])
    }).as_ref().map_err(|e| e)
}