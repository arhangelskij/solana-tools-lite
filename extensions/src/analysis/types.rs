/// Newtype wrapper for Solana Program IDs.
/// 
/// This type is used by extensions to avoid circular dependencies with solana-tools-lite.
/// The core crate provides conversion from this type to PubkeyBase58.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProgramId(String);
//TODO: 🔴 delete if not used
impl ProgramId {
    /// Create a new ProgramId from a base58 string.
    /// 
    /// # Errors
    /// 
    /// Returns an error if the string is not valid base58 or not 32 bytes when decoded.
    pub fn new(s: &str) -> Result<Self, String> {
        // Validate it's valid base58 and 32 bytes
        let decoded = bs58::decode(s)
            .into_vec()
            .map_err(|e| format!("Invalid base58: {}", e))?;
        
        if decoded.len() != 32 {
            return Err(format!("Invalid pubkey length: {} (expected 32)", decoded.len()));
        }
        
        Ok(Self(s.to_string()))
    }
    
    /// Get the base58 string representation.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for ProgramId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ProgramId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
