use crate::ToolError;
use crate::models::pubkey_base58::PubkeyBase58;
use std::collections::HashMap;

/// Parse lookup tables from JSON format.
///
/// Expected format:
/// ```json
/// {
///   "table_address": ["account1", "account2", ...],
///   ...
/// }
/// ```
pub fn parse_lookup_tables(
    json: &str,
) -> Result<HashMap<PubkeyBase58, Vec<PubkeyBase58>>, ToolError> {
    let raw: HashMap<String, Vec<String>> = serde_json::from_str(json)
        .map_err(|e| ToolError::InvalidInput(format!("invalid lookup tables JSON: {e}")))?;

    let mut out = HashMap::new();
    for (table_key, addresses) in raw {
        let table_pk = PubkeyBase58::try_from(table_key.as_str()).map_err(|e| {
            ToolError::InvalidInput(format!("invalid lookup table key {table_key}: {e}"))
        })?;

        let mut parsed = Vec::with_capacity(addresses.len());
        for addr in addresses {
            let pk = PubkeyBase58::try_from(addr.as_str()).map_err(|e| {
                ToolError::InvalidInput(format!(
                    "invalid lookup address {addr} for table {table_key}: {e}"
                ))
            })?;
            parsed.push(pk);
        }
        out.insert(table_pk, parsed);
    }

    Ok(out)
}
