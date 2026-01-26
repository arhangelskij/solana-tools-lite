use crate::ToolError;
use crate::models::pubkey_base58::PubkeyBase58;
use serde::{Deserialize, Serialize};

/// Lookup table entry from Solana RPC.
#[derive(Debug, Deserialize, Serialize)]
pub struct LookupTableEntry {
    #[serde(default)]
    pub writable: Vec<String>,
    #[serde(default)]
    pub readonly: Vec<String>,
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
/// Returns a combined list of all accounts (writable + readonly).
pub fn parse_lookup_tables(
    json: &str,
) -> Result<Vec<PubkeyBase58>, ToolError> {
    let entry: LookupTableEntry = serde_json::from_str(json)
        .map_err(|e| ToolError::InvalidInput(format!("invalid lookup tables JSON: {e}")))?;

    let mut all_accounts = Vec::new();
    
    // Parse writable accounts
    for addr in entry.writable {
        let pk = PubkeyBase58::try_from(addr.as_str()).map_err(|e| {
            ToolError::InvalidInput(format!("invalid writable address {addr}: {e}"))
        })?;
        all_accounts.push(pk);
    }
    
    // Parse readonly accounts
    for addr in entry.readonly {
        let pk = PubkeyBase58::try_from(addr.as_str()).map_err(|e| {
            ToolError::InvalidInput(format!("invalid readonly address {addr}: {e}"))
        })?;
        all_accounts.push(pk);
    }

    Ok(all_accounts)
}
