/// Light Protocol constants and program identifiers.
/// 
/// This module contains all the program IDs, discriminators, and other constants
/// used by the Light Protocol (ZK Compression) on Solana. These values are used
/// to identify and parse Light Protocol instructions.
use solana_tools_lite::{ToolError, models::pubkey_base58::PubkeyBase58};

/// Light Protocol system program ID (verified Jan 2026).
/// 
/// This program handles core Light Protocol operations including SOL compression
/// and system-level state management.

pub const LIGHT_SYSTEM_PROGRAM_ID: &str = "SySTEM1eSU2p4BGQfQpimFEWWSC1XDFeun3Nqzz3rT7";

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

pub const SPL_NOOP_PROGRAM_ID: &str = "noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV";

pub const LIGHT_REGISTRY_ID: &str = "Lighton6oQpVkeewmo2mcPTQQp7kYHr4fWpAgJyEmDX";


/// Size of u64 value in bytes.
/// 
/// Used for parsing amounts, lamports, and other numeric values
/// from instruction data.
pub const U64_SIZE: usize = 8;

/// Size of u16 value in bytes.
/// 
/// Used for parsing max_top_up and other u16 values.
pub const U16_SIZE: usize = 2;

// ============================================================================
// COMPRESSED TOKEN PROGRAM - 1-BYTE DISCRIMINATORS
// ============================================================================

/// Discriminator for CTokenTransfer instruction (1 byte).
/// SPL-compatible transfer of compressed tokens.
pub const DISCRIMINATOR_CTOKEN_TRANSFER: u8 = 3;

/// Discriminator for CTokenApprove instruction (1 byte).
/// Approve a delegate for compressed token operations.
pub const DISCRIMINATOR_CTOKEN_APPROVE: u8 = 4;

/// Discriminator for CTokenRevoke instruction (1 byte).
/// Revoke a delegate's approval.
pub const DISCRIMINATOR_CTOKEN_REVOKE: u8 = 5;

/// Discriminator for CTokenMintTo instruction (1 byte).
/// Mint compressed tokens to a recipient.
pub const DISCRIMINATOR_CTOKEN_MINT_TO: u8 = 7;

/// Discriminator for CTokenBurn instruction (1 byte).
/// Burn compressed tokens.
pub const DISCRIMINATOR_CTOKEN_BURN: u8 = 8;

/// Discriminator for CloseTokenAccount instruction (1 byte).
/// Close a compressed token account.
pub const DISCRIMINATOR_CLOSE_TOKEN_ACCOUNT: u8 = 9;

/// Discriminator for CTokenFreezeAccount instruction (1 byte).
/// Freeze a compressed token account.
pub const DISCRIMINATOR_CTOKEN_FREEZE_ACCOUNT: u8 = 10;

/// Discriminator for CTokenThawAccount instruction (1 byte).
/// Thaw a frozen compressed token account.
pub const DISCRIMINATOR_CTOKEN_THAW_ACCOUNT: u8 = 11;

/// Discriminator for CTokenTransferChecked instruction (1 byte).
/// Transfer with decimals validation (SPL compatible).
pub const DISCRIMINATOR_CTOKEN_TRANSFER_CHECKED: u8 = 12;

/// Discriminator for CTokenMintToChecked instruction (1 byte).
/// Mint with decimals validation.
pub const DISCRIMINATOR_CTOKEN_MINT_TO_CHECKED: u8 = 14;

/// Discriminator for CTokenBurnChecked instruction (1 byte).
/// Burn with decimals validation.
pub const DISCRIMINATOR_CTOKEN_BURN_CHECKED: u8 = 15;

/// Discriminator for CreateTokenAccount instruction (1 byte).
/// Create a new compressed token account (SPL InitializeAccount3 equivalent).
pub const DISCRIMINATOR_CREATE_TOKEN_ACCOUNT: u8 = 18;

/// Discriminator for CreateAssociatedTokenAccount instruction (1 byte).
/// Create an associated token account for compressed tokens.
pub const DISCRIMINATOR_CREATE_ASSOCIATED_TOKEN_ACCOUNT: u8 = 100;

/// Discriminator for Transfer2 instruction (1 byte).
/// Batch instruction for compressed token transfers and compression/decompression.
pub const DISCRIMINATOR_TRANSFER2: u8 = 101;

/// Discriminator for CreateAssociatedTokenAccountIdempotent instruction (1 byte).
/// Idempotent creation of associated token account.
pub const DISCRIMINATOR_CREATE_ASSOCIATED_TOKEN_ACCOUNT_IDEMPOTENT: u8 = 102;

/// Discriminator for MintAction instruction (1 byte).
/// Batch instruction for operations on compressed mint accounts.
pub const DISCRIMINATOR_MINT_ACTION: u8 = 103;

/// Discriminator for Claim instruction (1 byte).
/// Claim rent for past completed epochs from compressible token account.
pub const DISCRIMINATOR_CLAIM: u8 = 104;

/// Discriminator for WithdrawFundingPool instruction (1 byte).
/// Withdraw funds from pool PDA.
pub const DISCRIMINATOR_WITHDRAW_FUNDING_POOL: u8 = 105;

// ============================================================================
// LIGHT SYSTEM PROGRAM - 8-BYTE DISCRIMINATORS
// ============================================================================

/// Discriminator for Invoke instruction (8 bytes).
/// Basic invocation of Light System Program.
pub const DISCRIMINATOR_INVOKE: [u8; 8] = [26, 16, 169, 7, 21, 202, 242, 25];

/// Discriminator for InvokeCpi instruction (8 bytes).
/// Invocation with Cross-Program Invocation support.
pub const DISCRIMINATOR_INVOKE_CPI: [u8; 8] = [49, 212, 191, 129, 39, 194, 43, 196];

/// Discriminator for InvokeCpiWithReadOnly instruction (8 bytes).
/// CPI invocation with read-only accounts.
pub const DISCRIMINATOR_INVOKE_CPI_WITH_READ_ONLY: [u8; 8] = [86, 47, 163, 166, 21, 223, 92, 8];

/// Discriminator for InvokeCpiWithAccountInfo instruction (8 bytes).
/// CPI invocation with AccountInfo support.
pub const DISCRIMINATOR_INVOKE_CPI_WITH_ACCOUNT_INFO: [u8; 8] = [228, 34, 128, 84, 47, 139, 86, 240];

// ============================================================================
// ACCOUNT COMPRESSION PROGRAM - 8-BYTE DISCRIMINATORS
// ============================================================================

/// Discriminator for InsertIntoQueues instruction (8 bytes).
/// Insert compressed data into merkle tree queues.
pub const DISCRIMINATOR_INSERT_INTO_QUEUES: [u8; 8] = [180, 143, 159, 153, 35, 46, 248, 163];

// ============================================================================
// LIGHT REGISTRY PROGRAM - 8-BYTE DISCRIMINATORS
// ============================================================================

/// Discriminator for CreateConfigCounter instruction (8 bytes).
/// Create the config counter PDA that tracks compressible configs.
pub const DISCRIMINATOR_CREATE_CONFIG_COUNTER: [u8; 8] = [221, 9, 219, 187, 215, 138, 209, 87];

/// Discriminator for CreateCompressibleConfig instruction (8 bytes).
/// Create a new compressible config with specified parameters.
pub const DISCRIMINATOR_CREATE_COMPRESSIBLE_CONFIG: [u8; 8] = [13, 182, 188, 82, 224, 82, 11, 174];

// ============================================================================
// TOKEN INTERFACE - 8-BYTE DISCRIMINATORS
// ============================================================================

/// Discriminator for MintTo instruction (8 bytes).
/// Mint tokens via Token Interface.
pub const DISCRIMINATOR_TOKEN_INTERFACE_MINT_TO: [u8; 8] = [241, 34, 48, 186, 37, 179, 123, 192];

/// Discriminator for Transfer instruction (8 bytes).
/// Transfer tokens via Token Interface.
pub const DISCRIMINATOR_TOKEN_INTERFACE_TRANSFER: [u8; 8] = [163, 52, 200, 231, 140, 3, 69, 186];

/// Discriminator for BatchCompress instruction (8 bytes).
/// Batch compression of tokens.
pub const DISCRIMINATOR_BATCH_COMPRESS: [u8; 8] = [65, 206, 101, 37, 147, 42, 221, 144];

/// Discriminator for Approve instruction (8 bytes).
/// Approve a delegate via Token Interface.
pub const DISCRIMINATOR_TOKEN_INTERFACE_APPROVE: [u8; 8] = [69, 74, 217, 36, 115, 117, 97, 76];

/// Discriminator for Revoke instruction (8 bytes).
/// Revoke a delegate via Token Interface.
pub const DISCRIMINATOR_TOKEN_INTERFACE_REVOKE: [u8; 8] = [170, 23, 31, 34, 133, 173, 93, 242];

/// Discriminator for Freeze instruction (8 bytes).
/// Freeze an account via Token Interface.
pub const DISCRIMINATOR_TOKEN_INTERFACE_FREEZE: [u8; 8] = [255, 91, 207, 84, 251, 194, 254, 63];

/// Discriminator for Thaw instruction (8 bytes).
/// Thaw an account via Token Interface.
pub const DISCRIMINATOR_TOKEN_INTERFACE_THAW: [u8; 8] = [226, 249, 34, 57, 189, 21, 177, 101];

/// Discriminator for CreateTokenPool instruction (8 bytes).
/// Create a new token pool.
pub const DISCRIMINATOR_CREATE_TOKEN_POOL: [u8; 8] = [23, 169, 27, 122, 147, 169, 209, 152];

/// Discriminator for AddTokenPool instruction (8 bytes).
/// Add a token pool.
pub const DISCRIMINATOR_ADD_TOKEN_POOL: [u8; 8] = [114, 143, 210, 73, 96, 115, 1, 228];
/// Returns the list of Light Protocol program IDs.
/// 
/// Uses `OnceLock` to cache the parsed program IDs, ensuring they are only
/// parsed once even with multiple calls. Subsequent calls return a reference
/// to the cached slice with zero allocations.
/// 
/// # Returns
/// 
/// `Ok(&'static [PubkeyBase58])` containing exactly three Light Protocol program IDs,
/// or `Err(ToolError)` if any program ID fails to parse.
/// 
/// # Errors
/// 
/// Returns `ToolError::ConfigurationError` if any of the hardcoded program ID strings
/// cannot be parsed as valid base58 public keys. This indicates a bug in the code
/// rather than user error.
/// 
/// # Thread Safety
/// 
/// This function is thread-safe. Multiple concurrent calls during initialization
/// will coordinate via `OnceLock`, with only one thread performing the parsing.
pub fn supported_programs() -> Result<&'static [PubkeyBase58], ToolError> {
    use std::sync::OnceLock;
    
    static PROGRAMS: OnceLock<Result<Vec<PubkeyBase58>, ToolError>> = OnceLock::new();
    
    let result = PROGRAMS.get_or_init(|| {
        let light_system = PubkeyBase58::try_from(LIGHT_SYSTEM_PROGRAM_ID)
            .map_err(|_| ToolError::ConfigurationError("LIGHT_SYSTEM_PROGRAM_ID".to_string()))?;
        
        let account_compression = PubkeyBase58::try_from(ACCOUNT_COMPRESSION_PROGRAM_ID)
            .map_err(|_| ToolError::ConfigurationError("ACCOUNT_COMPRESSION_PROGRAM_ID".to_string()))?;
        
        let compressed_token = PubkeyBase58::try_from(COMPRESSED_TOKEN_PROGRAM_ID)
            .map_err(|_| ToolError::ConfigurationError("COMPRESSED_TOKEN_PROGRAM_ID".to_string()))?;
        
        let light_registry = PubkeyBase58::try_from(LIGHT_REGISTRY_ID)
            .map_err(|_| ToolError::ConfigurationError("LIGHT_REGISTRY_ID".to_string()))?;
        
        let spl_noop = PubkeyBase58::try_from(SPL_NOOP_PROGRAM_ID)
            .map_err(|_| ToolError::ConfigurationError("SPL_NOOP_PROGRAM_ID".to_string()))?;
        
        Ok(vec![light_system, account_compression, compressed_token, light_registry, spl_noop])
    });
    
    match result {
        Ok(vec) => Ok(vec.as_slice()),
        Err(e) => Err(ToolError::ConfigurationError(format!("Failed to initialize Light Protocol programs: {}", e))),
    }
}
