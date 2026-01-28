use crate::ToolError;
use crate::models::pubkey_base58::PubkeyBase58;
use serde::{Deserialize, Serialize};

/// Lookup table entry from Solana RPC.
/// Stores writable and readonly accounts for sequential offset processing.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LookupTableEntry {
    #[serde(default)]
    pub writable: Vec<PubkeyBase58>,
    #[serde(default)]
    pub readonly: Vec<PubkeyBase58>,
}

/// Parse lookup tables from Solana RPC format.
///
/// Expected format:
/// ```json
/// {
///   "readonly": ["account1", "account2", ...],
///   "writable": ["account3", "account4", ...]
/// }
/// ```
///
/// Returns a HashMap where:
/// - Key: dummy key (all zeros) - user provides just the table data without address
/// - Value: LookupTableEntry with writable and readonly accounts
pub fn parse_lookup_tables(
    json: &str,
) -> Result<LookupTableEntry, ToolError> {
    // Parse as raw strings
    #[derive(Deserialize)]
    struct RawEntry {
        #[serde(default)]
        pub writable: Vec<String>,
        #[serde(default)]
        pub readonly: Vec<String>,
    }

    let raw: RawEntry = serde_json::from_str(json)
        .map_err(|e| ToolError::InvalidInput(format!("invalid lookup tables JSON: {e}")))?;

    let mut writable = Vec::new();
    let mut readonly = Vec::new();
    
    // Parse writable accounts
    for addr in raw.writable {
        let pk = PubkeyBase58::try_from(addr.as_str()).map_err(|e| {
            ToolError::InvalidInput(format!("invalid writable address {addr}: {e}"))
        })?;
        writable.push(pk);
    }
    
    // Parse readonly accounts
    for addr in raw.readonly {
        let pk = PubkeyBase58::try_from(addr.as_str()).map_err(|e| {
            ToolError::InvalidInput(format!("invalid readonly address {addr}: {e}"))
        })?;
        readonly.push(pk);
    }

    let entry = LookupTableEntry { writable, readonly };

    Ok(entry)
}
