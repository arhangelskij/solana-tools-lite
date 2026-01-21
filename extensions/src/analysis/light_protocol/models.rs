use solana_tools_lite::models::extensions::PrivacyImpact;

/// Action types detected for Light Protocol (ZK Compression).
#[derive(Debug, Clone, PartialEq)]
pub enum LightProtocolAction {
    // ========================================================================
    // COMPRESSED TOKEN PROGRAM - 1-BYTE DISCRIMINATORS
    // ========================================================================
    
    /// CTokenTransfer: SPL-compatible transfer of compressed tokens.
    CTokenTransfer,
    /// CTokenApprove: Approve a delegate for compressed token operations.
    CTokenApprove,
    /// CTokenRevoke: Revoke a delegate's approval.
    CTokenRevoke,
    /// CTokenMintTo: Mint compressed tokens to a recipient.
    CTokenMintTo,
    /// CTokenBurn: Burn compressed tokens.
    CTokenBurn,
    /// CloseTokenAccount: Close a compressed token account.
    CloseTokenAccount,
    /// CTokenFreezeAccount: Freeze a compressed token account.
    CTokenFreezeAccount,
    /// CTokenThawAccount: Thaw a frozen compressed token account.
    CTokenThawAccount,
    /// CTokenTransferChecked: Transfer with decimals validation (SPL compatible).
    CTokenTransferChecked,
    /// CTokenMintToChecked: Mint with decimals validation.
    CTokenMintToChecked,
    /// CTokenBurnChecked: Burn with decimals validation.
    CTokenBurnChecked,
    /// CreateTokenAccount: Create a new compressed token account.
    CreateTokenAccount,
    /// CreateAssociatedTokenAccount: Create an associated token account for compressed tokens.
    CreateAssociatedTokenAccount,
    /// Transfer2: Batch instruction for compressed token transfers and compression/decompression.
    Transfer2,
    /// Decompress: Decompress compressed assets back to regular form.
    Decompress,
    /// CreateAssociatedTokenAccountIdempotent: Idempotent creation of associated token account.
    CreateAssociatedTokenAccountIdempotent,
    /// MintAction: Batch instruction for operations on compressed mint accounts.
    MintAction,
    /// Claim: Claim rent for past completed epochs from compressible token account.
    Claim,
    /// WithdrawFundingPool: Withdraw funds from pool PDA.
    WithdrawFundingPool,
    
    // ========================================================================
    // LIGHT SYSTEM PROGRAM - 8-BYTE DISCRIMINATORS
    // ========================================================================
    
    /// Invoke: Basic invocation of Light System Program.
    Invoke,
    /// InvokeCpi: Invocation with Cross-Program Invocation support.
    InvokeCpi,
    /// InvokeCpiWithReadOnly: CPI invocation with read-only accounts.
    InvokeCpiWithReadOnly,
    /// InvokeCpiWithAccountInfo: CPI invocation with AccountInfo support.
    InvokeCpiWithAccountInfo,
    
    // ========================================================================
    // ACCOUNT COMPRESSION PROGRAM - 8-BYTE DISCRIMINATORS
    // ========================================================================
    
    /// InsertIntoQueues: Insert compressed data into merkle tree queues.
    InsertIntoQueues,
    
    // ========================================================================
    // LIGHT REGISTRY PROGRAM - 8-BYTE DISCRIMINATORS
    // ========================================================================
    
    /// CreateConfigCounter: Create the config counter PDA.
    CreateConfigCounter,
    /// CreateCompressibleConfig: Create a new compressible config.
    CreateCompressibleConfig,
    
    // ========================================================================
    // TOKEN INTERFACE - 8-BYTE DISCRIMINATORS
    // ========================================================================
    
    /// TokenInterfaceMintTo: Mint tokens via Token Interface.
    TokenInterfaceMintTo,
    /// TokenInterfaceTransfer: Transfer tokens via Token Interface.
    TokenInterfaceTransfer,
    /// BatchCompress: Batch compression of tokens.
    BatchCompress,
    /// TokenInterfaceApprove: Approve a delegate via Token Interface.
    TokenInterfaceApprove,
    /// TokenInterfaceRevoke: Revoke a delegate via Token Interface.
    TokenInterfaceRevoke,
    /// TokenInterfaceFreeze: Freeze an account via Token Interface.
    TokenInterfaceFreeze,
    /// TokenInterfaceThaw: Thaw an account via Token Interface.
    TokenInterfaceThaw,
    /// CreateTokenPool: Create a new token pool.
    CreateTokenPool,
    /// AddTokenPool: Add a token pool.
    AddTokenPool,
    
    // ========================================================================
    // ANCHOR INSTRUCTIONS - 8-BYTE DISCRIMINATORS
    // ========================================================================
    
    /// Freeze: Anchor freeze instruction.
    Freeze,
    /// Thaw: Anchor thaw instruction.
    Thaw,
    
    /// Action not specifically parsed but identified as Light Protocol.
    Unknown { discriminator: u8 },
    /// Unknown 8-byte discriminator.
    UnknownEightByte { discriminator: [u8; 8] },
}

impl LightProtocolAction {
    pub fn description(&self) -> String {
        match self {
            // Compressed Token Program
            Self::CTokenTransfer => "Transfer Compressed Tokens".to_string(),
            Self::CTokenApprove => "Approve Compressed Token Delegate".to_string(),
            Self::CTokenRevoke => "Revoke Compressed Token Delegate".to_string(),
            Self::CTokenMintTo => "Mint Compressed Tokens".to_string(),
            Self::CTokenBurn => "Burn Compressed Tokens".to_string(),
            Self::CloseTokenAccount => "Close Compressed Token Account".to_string(),
            Self::CTokenFreezeAccount => "Freeze Compressed Token Account".to_string(),
            Self::CTokenThawAccount => "Thaw Compressed Token Account".to_string(),
            Self::CTokenTransferChecked => "Transfer Compressed Tokens (Checked)".to_string(),
            Self::CTokenMintToChecked => "Mint Compressed Tokens (Checked)".to_string(),
            Self::CTokenBurnChecked => "Burn Compressed Tokens (Checked)".to_string(),
            Self::CreateTokenAccount => "Create Compressed Token Account".to_string(),
            Self::CreateAssociatedTokenAccount => "Create Associated Compressed Token Account".to_string(),
            Self::Transfer2 => "Batch Transfer Compressed Tokens".to_string(),
            Self::Decompress => "Decompress Assets".to_string(),
            Self::CreateAssociatedTokenAccountIdempotent => "Create Associated Compressed Token Account (Idempotent)".to_string(),
            Self::MintAction => "Batch Mint Action".to_string(),
            Self::Claim => "Claim Rent".to_string(),
            Self::WithdrawFundingPool => "Withdraw Funding Pool".to_string(),
            
            // Light System Program
            Self::Invoke => "Light System Invoke".to_string(),
            Self::InvokeCpi => "Light System Invoke (CPI)".to_string(),
            Self::InvokeCpiWithReadOnly => "Light System Invoke (CPI with Read-Only)".to_string(),
            Self::InvokeCpiWithAccountInfo => "Light System Invoke (CPI with AccountInfo)".to_string(),
            
            // Account Compression Program
            Self::InsertIntoQueues => "Insert Into Merkle Tree Queues".to_string(),
            
            // Light Registry Program
            Self::CreateConfigCounter => "Create Config Counter".to_string(),
            Self::CreateCompressibleConfig => "Create Compressible Config".to_string(),
            
            // Token Interface
            Self::TokenInterfaceMintTo => "Mint Tokens (Token Interface)".to_string(),
            Self::TokenInterfaceTransfer => "Transfer Tokens (Token Interface)".to_string(),
            Self::BatchCompress => "Batch Compress Tokens".to_string(),
            Self::TokenInterfaceApprove => "Approve Delegate (Token Interface)".to_string(),
            Self::TokenInterfaceRevoke => "Revoke Delegate (Token Interface)".to_string(),
            Self::TokenInterfaceFreeze => "Freeze Account (Token Interface)".to_string(),
            Self::TokenInterfaceThaw => "Thaw Account (Token Interface)".to_string(),
            Self::CreateTokenPool => "Create Token Pool".to_string(),
            Self::AddTokenPool => "Add Token Pool".to_string(),
            
            // Anchor Instructions
            Self::Freeze => "Freeze Account (Anchor)".to_string(),
            Self::Thaw => "Thaw Account (Anchor)".to_string(),
            
            Self::Unknown { discriminator } => format!("Unknown Light Protocol Action (discriminator: {})", discriminator),
            Self::UnknownEightByte { discriminator } => format!("Unknown Light Protocol Action (discriminator: {:?})", discriminator),
        }
    }

    /// Determine the privacy impact of this Light Protocol action.
    pub fn privacy_impact(&self) -> PrivacyImpact {
        match self {
            // Confidential operations - fully private value transfers
            Self::CTokenTransfer | Self::CTokenTransferChecked |
            Self::Transfer2 | Self::CTokenMintTo | Self::CTokenMintToChecked |
            Self::CTokenBurn | Self::CTokenBurnChecked |
            Self::TokenInterfaceMintTo | Self::TokenInterfaceTransfer |
            Self::BatchCompress => {
                PrivacyImpact::Confidential
            }

            // Hybrid operations - transition between public and private state
            Self::Decompress => {
                PrivacyImpact::Hybrid
            }

            // Storage compression operations - infrastructure management
            Self::CreateTokenAccount | Self::CreateAssociatedTokenAccount |
            Self::CreateAssociatedTokenAccountIdempotent | Self::CloseTokenAccount |
            Self::CTokenFreezeAccount | Self::CTokenThawAccount |
            Self::CTokenApprove | Self::CTokenRevoke |
            Self::MintAction | Self::Claim | Self::WithdrawFundingPool |
            Self::Invoke | Self::InvokeCpi | Self::InvokeCpiWithReadOnly | Self::InvokeCpiWithAccountInfo |
            Self::InsertIntoQueues |
            Self::CreateConfigCounter | Self::CreateCompressibleConfig |
            Self::TokenInterfaceApprove | Self::TokenInterfaceRevoke |
            Self::TokenInterfaceFreeze | Self::TokenInterfaceThaw |
            Self::CreateTokenPool | Self::AddTokenPool |
            Self::Freeze | Self::Thaw => {
                PrivacyImpact::StorageCompression
            }

            // Unknown
            Self::Unknown { .. } | Self::UnknownEightByte { .. } => PrivacyImpact::None,
        }
    }
}

/// Implement ExtensionAction trait for Light Protocol actions.
impl solana_tools_lite::models::extensions::ExtensionAction for LightProtocolAction {
    fn protocol_name(&self) -> &'static str {
        "Light Protocol"
    }
    
    fn description(&self) -> String {
        self.description()
    }
    
    fn privacy_impact(&self) -> PrivacyImpact {
        self.privacy_impact()
    }
}
