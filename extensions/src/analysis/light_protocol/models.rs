use solana_tools_lite::models::extensions::PrivacyImpact;

/// Action types detected for Light Protocol (ZK Compression).
#[derive(Debug, Clone, PartialEq)]
pub enum LightProtocolAction {
    // ========================================================================
    // COMPRESSED TOKEN PROGRAM - 1-BYTE DISCRIMINATORS
    // ========================================================================
    
    /// CTokenTransfer: SPL-compatible transfer of compressed tokens.
    CTokenTransfer { amount: Option<u64> },
    /// CTokenApprove: Approve a delegate for compressed token operations.
    CTokenApprove { amount: Option<u64> },
    /// CTokenRevoke: Revoke a delegate's approval.
    CTokenRevoke,
    /// CTokenMintTo: Mint compressed tokens to a recipient.
    CTokenMintTo { amount: Option<u64> },
    /// CTokenBurn: Burn compressed tokens.
    CTokenBurn { amount: Option<u64> },
    /// CloseTokenAccount: Close a compressed token account.
    CloseTokenAccount,
    /// CTokenFreezeAccount: Freeze a compressed token account.
    CTokenFreezeAccount,
    /// CTokenThawAccount: Thaw a frozen compressed token account.
    CTokenThawAccount,
    /// CTokenTransferChecked: Transfer with decimals validation (SPL compatible).
    CTokenTransferChecked { amount: Option<u64> },
    /// CTokenMintToChecked: Mint with decimals validation.
    CTokenMintToChecked { amount: Option<u64> },
    /// CTokenBurnChecked: Burn with decimals validation.
    CTokenBurnChecked { amount: Option<u64> },
    /// CreateTokenAccount: Create a new compressed token account.
    CreateTokenAccount,
    /// CreateAssociatedTokenAccount: Create an associated token account for compressed tokens.
    CreateAssociatedTokenAccount,
    /// Transfer2: Batch instruction for compressed token transfers and compression/decompression.
    Transfer2 { 
        in_lamports: Option<u64>, 
        out_lamports: Option<u64>,
        amount: Option<u64>
    },
    /// CreateAssociatedTokenAccountIdempotent: Idempotent creation of associated token account.
    CreateAssociatedTokenAccountIdempotent,
    /// MintAction: Batch instruction for operations on compressed mint accounts.
    MintAction,
    /// Claim: Claim rent for past completed epochs from compressible token account.
    Claim,
    /// WithdrawFundingPool: Withdraw funds from pool PDA.
    WithdrawFundingPool { amount: Option<u64> },
    
    // ========================================================================
    // LIGHT SYSTEM PROGRAM - 8-BYTE DISCRIMINATORS
    // ========================================================================
    
    /// Invoke: Basic invocation of Light System Program.
    Invoke { lamports: Option<u64>, from_index: Option<u8>, to_index: Option<u8> },
    /// InvokeCpi: Invocation with Cross-Program Invocation support.
    InvokeCpi { lamports: Option<u64>, from_index: Option<u8>, to_index: Option<u8> },
    /// InvokeCpiWithReadOnly: CPI invocation with read-only accounts.
    InvokeCpiWithReadOnly { lamports: Option<u64>, from_index: Option<u8>, to_index: Option<u8> },
    /// InvokeCpiWithAccountInfo: CPI invocation with AccountInfo support.
    InvokeCpiWithAccountInfo { lamports: Option<u64>, from_index: Option<u8>, to_index: Option<u8> },
    /// InitCpiContextAccount: Initialize a CPI context account.
    InitCpiContextAccount,
    /// ReInitCpiContextAccount: Reinitialize a CPI context account.
    ReInitCpiContextAccount,
    
    // ========================================================================
    // ACCOUNT COMPRESSION PROGRAM - 8-BYTE DISCRIMINATORS
    // ========================================================================
    
    /// InsertIntoQueues: Insert compressed data into merkle tree queues.
    InsertIntoQueues,
    /// InitializeCompressionConfig: Initialize the compression configuration.
    InitializeCompressionConfig,
    /// UpdateCompressionConfig: Update the compression configuration.
    UpdateCompressionConfig,
    /// DecompressAccountsIdempotent: Idempotently decompress accounts.
    DecompressAccountsIdempotent,
    /// CompressAccountsIdempotent: Idempotently compress accounts.
    CompressAccountsIdempotent,
    
    // ========================================================================
    // LIGHT REGISTRY PROGRAM - 8-BYTE DISCRIMINATORS
    // ========================================================================
    
    /// CreateConfigCounter: Create the config counter PDA.
    CreateConfigCounter,
    /// CreateCompressibleConfig: Create a new compressible config.
    CreateCompressibleConfig,
    /// RegistryClaim: Claim rewards or rent.
    RegistryClaim,
    /// CompressAndClose: Compress and close an account.
    CompressAndClose,
    /// RegisterForester: Register a forester.
    RegisterForester,
    /// RegisterForesterEpoch: Register a forester for an epoch.
    RegisterForesterEpoch,
    /// FinalizeRegistration: Finalize forester registration.
    FinalizeRegistration,
    /// ReportWork: Report work done by forester.
    ReportWork,
    
    // ========================================================================
    // TOKEN INTERFACE - 8-BYTE DISCRIMINATORS
    // ========================================================================
    
    /// TokenInterfaceMintTo: Mint tokens via Token Interface.
    TokenInterfaceMintTo { amount: Option<u64> },
    /// TokenInterfaceTransfer: Transfer tokens via Token Interface.
    TokenInterfaceTransfer { amount: Option<u64> },
    /// BatchCompress: Batch compression of tokens.
    BatchCompress { amount: Option<u64> },
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
    
    /// Freeze: Freeze a compressed token account (8-byte discriminator).
    CTokenFreeze,
    /// Thaw: Thaw a compressed token account (8-byte discriminator).
    CTokenThaw,
    
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
            Self::CTokenTransfer { amount } => {
                match amount {
                    Some(amt) => format!("Transfer Compressed Tokens ({} amount)", amt),
                    None => "Transfer Compressed Tokens".to_string(),
                }
            }
            Self::CTokenApprove { amount } => {
                match amount {
                    Some(amt) => format!("Approve Compressed Token Delegate ({} amount)", amt),
                    None => "Approve Compressed Token Delegate".to_string(),
                }
            }
            Self::CTokenRevoke => "Revoke Compressed Token Delegate".to_string(),
            Self::CTokenMintTo { amount } => {
                match amount {
                    Some(amt) => format!("Mint Compressed Tokens ({} amount)", amt),
                    None => "Mint Compressed Tokens".to_string(),
                }
            }
            Self::CTokenBurn { amount } => {
                match amount {
                    Some(amt) => format!("Burn Compressed Tokens ({} amount)", amt),
                    None => "Burn Compressed Tokens".to_string(),
                }
            }
            Self::CloseTokenAccount => "Close Compressed Token Account".to_string(),
            Self::CTokenFreezeAccount => "Freeze Compressed Token Account".to_string(),
            Self::CTokenThawAccount => "Thaw Compressed Token Account".to_string(),
            Self::CTokenTransferChecked { amount } => {
                match amount {
                    Some(amt) => format!("Transfer Compressed Tokens Checked ({} amount)", amt),
                    None => "Transfer Compressed Tokens (Checked)".to_string(),
                }
            }
            Self::CTokenMintToChecked { amount } => {
                match amount {
                    Some(amt) => format!("Mint Compressed Tokens Checked ({} amount)", amt),
                    None => "Mint Compressed Tokens (Checked)".to_string(),
                }
            }
            Self::CTokenBurnChecked { amount } => {
                match amount {
                    Some(amt) => format!("Burn Compressed Tokens Checked ({} amount)", amt),
                    None => "Burn Compressed Tokens (Checked)".to_string(),
                }
            }
            Self::CreateTokenAccount => "Create Compressed Token Account".to_string(),
            Self::CreateAssociatedTokenAccount => "Create Associated Compressed Token Account".to_string(),
            Self::Transfer2 { in_lamports, out_lamports, amount } => {
                let mut parts = Vec::new();
                if let Some(amt) = amount { parts.push(format!("{} amount", amt)); }
                if let Some(l) = in_lamports { parts.push(format!("{} in_lamports", l)); }
                if let Some(l) = out_lamports { parts.push(format!("{} out_lamports", l)); }
                
                if parts.is_empty() {
                    "Batch Transfer Compressed Tokens".to_string()
                } else {
                    format!("Batch Transfer Compressed Tokens ({})", parts.join(", "))
                }
            }
            Self::CreateAssociatedTokenAccountIdempotent => "Create Associated Compressed Token Account (Idempotent)".to_string(),
            Self::MintAction => "Batch Mint Action".to_string(),
            Self::Claim => "Claim Rent".to_string(),
            Self::WithdrawFundingPool { amount } => {
                match amount {
                    Some(amt) => format!("Withdraw Funding Pool ({} amount)", amt),
                    None => "Withdraw Funding Pool".to_string(),
                }
            }
            
            // Light System Program
            Self::Invoke { lamports, .. } => {
                match lamports {
                    Some(l) => format!("Light System Invoke ({} lamports)", l),
                    None => "Light System Invoke".to_string(),
                }
            }
            Self::InvokeCpi { lamports, .. } => {
                match lamports {
                    Some(l) => format!("Light System Invoke (CPI) ({} lamports)", l),
                    None => "Light System Invoke (CPI)".to_string(),
                }
            }
            Self::InvokeCpiWithReadOnly { lamports, .. } => {
                match lamports {
                    Some(l) => format!("Light System Invoke (CPI with Read-Only) ({} lamports)", l),
                    None => "Light System Invoke (CPI with Read-Only)".to_string(),
                }
            }
            Self::InvokeCpiWithAccountInfo { lamports, .. } => {
                match lamports {
                    Some(l) => format!("Light System Invoke (CPI with AccountInfo) ({} lamports)", l),
                    None => "Light System Invoke (CPI with AccountInfo)".to_string(),
                }
            }
            Self::InitCpiContextAccount => "Initialize CPI Context Account".to_string(),
            Self::ReInitCpiContextAccount => "Reinitialize CPI Context Account".to_string(),
            
            // Account Compression Program
            Self::InsertIntoQueues => "Insert Into Merkle Tree Queues".to_string(),
            Self::InitializeCompressionConfig => "Initialize Compression Config".to_string(),
            Self::UpdateCompressionConfig => "Update Compression Config".to_string(),
            Self::DecompressAccountsIdempotent => "Decompress Accounts (Idempotent)".to_string(),
            Self::CompressAccountsIdempotent => "Compress Accounts (Idempotent)".to_string(),
            
            // Light Registry Program
            Self::CreateConfigCounter => "Create Config Counter".to_string(),
            Self::CreateCompressibleConfig => "Create Compressible Config".to_string(),
            Self::RegistryClaim => "Claim Rewards/Rent (Registry)".to_string(),
            Self::CompressAndClose => "Compress And Close Account".to_string(),
            Self::RegisterForester => "Register Forester".to_string(),
            Self::RegisterForesterEpoch => "Register Forester Epoch".to_string(),
            Self::FinalizeRegistration => "Finalize Forester Registration".to_string(),
            Self::ReportWork => "Report Forester Work".to_string(),
            
            // Token Interface
            Self::TokenInterfaceMintTo { amount } => {
                match amount {
                    Some(amt) => format!("Mint Tokens Interface ({} amount)", amt),
                    None => "Mint Tokens (Token Interface)".to_string(),
                }
            }
            Self::TokenInterfaceTransfer { amount } => {
                match amount {
                    Some(amt) => format!("Transfer Tokens Interface ({} amount)", amt),
                    None => "Transfer Tokens (Token Interface)".to_string(),
                }
            }
            Self::BatchCompress { amount } => {
                match amount {
                    Some(amt) => format!("Batch Compress Tokens ({} amount)", amt),
                    None => "Batch Compress Tokens".to_string(),
                }
            }
            Self::TokenInterfaceApprove => "Approve Delegate (Token Interface)".to_string(),
            Self::TokenInterfaceRevoke => "Revoke Delegate (Token Interface)".to_string(),
            Self::TokenInterfaceFreeze => "Freeze Account (Token Interface)".to_string(),
            Self::TokenInterfaceThaw => "Thaw Account (Token Interface)".to_string(),
            Self::CreateTokenPool => "Create Token Pool".to_string(),
            Self::AddTokenPool => "Add Token Pool".to_string(),
            
            // Freeze/Thaw
            Self::CTokenFreeze => "Freeze Compressed Token Account (8-byte)".to_string(),
            Self::CTokenThaw => "Thaw Compressed Token Account (8-byte)".to_string(),
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
            Self::CTokenTransfer { .. } | Self::CTokenTransferChecked { .. } |
            Self::Transfer2 { .. } | Self::CTokenMintTo { .. } | Self::CTokenMintToChecked { .. } |
            Self::CTokenBurn { .. } | Self::CTokenBurnChecked { .. } |
            Self::TokenInterfaceMintTo { .. } | Self::TokenInterfaceTransfer { .. } |
            Self::BatchCompress { .. } => {
                PrivacyImpact::Confidential
            }

            // Storage compression operations - infrastructure management
            Self::CreateTokenAccount | Self::CreateAssociatedTokenAccount |
            Self::CreateAssociatedTokenAccountIdempotent | Self::CloseTokenAccount |
            Self::CTokenFreezeAccount | Self::CTokenThawAccount |
            Self::CTokenApprove { .. } | Self::CTokenRevoke |
            Self::MintAction | Self::Claim | Self::WithdrawFundingPool { .. } |
            Self::Invoke { .. } | Self::InvokeCpi { .. } | Self::InvokeCpiWithReadOnly { .. } | Self::InvokeCpiWithAccountInfo { .. } |
            Self::InitCpiContextAccount | Self::ReInitCpiContextAccount |
            Self::InsertIntoQueues | Self::InitializeCompressionConfig | Self::UpdateCompressionConfig |
            Self::DecompressAccountsIdempotent | Self::CompressAccountsIdempotent |
            Self::CreateConfigCounter | Self::CreateCompressibleConfig |
            Self::RegistryClaim | Self::CompressAndClose | Self::RegisterForester |
            Self::RegisterForesterEpoch | Self::FinalizeRegistration | Self::ReportWork |
            Self::TokenInterfaceApprove | Self::TokenInterfaceRevoke |
            Self::TokenInterfaceFreeze | Self::TokenInterfaceThaw |
            Self::CreateTokenPool | Self::AddTokenPool |
            Self::Freeze | Self::Thaw | Self::CTokenFreeze | Self::CTokenThaw => {
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
