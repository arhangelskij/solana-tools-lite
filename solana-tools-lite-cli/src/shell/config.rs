use crate::models::cmds::OutFmt;
use std::env;

/// Resolve global and command-specific configuration from environment variables.
pub struct ConfigResolver;

impl ConfigResolver {
    /// Resolve the signer keypair path with fallback to environment.
    pub fn resolve_keypair(explicit: Option<String>) -> Option<String> {
        explicit.or_else(|| env::var("SOLANA_SIGNER_KEYPAIR").ok())
    }

    /// Resolve max fee with fallback to environment.
    pub fn resolve_max_fee(explicit: Option<u64>) -> Option<u64> {
        explicit.or_else(|| {
            env::var("SOLANA_TOOLS_LITE_MAX_FEE")
                .ok()
                .and_then(|v| v.parse().ok())
        })
    }

    /// Resolve output format with fallback to environment.
    pub fn resolve_output_format(explicit: Option<OutFmt>) -> Option<OutFmt> {
        explicit.or_else(|| {
            env::var("SOLANA_TOOLS_LITE_OUTPUT_FORMAT")
                .ok()
                .and_then(|v| match v.to_lowercase().as_str() {
                    "json" => Some(OutFmt::Json),
                    "base64" => Some(OutFmt::Base64),
                    "base58" => Some(OutFmt::Base58),
                    _ => None,
                })
        })
    }

    /// Resolve global JSON flag.
    pub fn resolve_json(explicit: bool) -> bool {
        if explicit {
            return true;
        }
        env::var("SOLANA_TOOLS_LITE_JSON")
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false)
    }

    /// Resolve global Force flag.
    pub fn resolve_force(explicit: bool) -> bool {
        if explicit {
            return true;
        }
        env::var("SOLANA_TOOLS_LITE_FORCE")
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false)
    }

    /// Resolve global Yes flag.
    pub fn resolve_yes(explicit: bool) -> bool {
        if explicit {
            return true;
        }
        env::var("SOLANA_TOOLS_LITE_YES")
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false)
    }
}
