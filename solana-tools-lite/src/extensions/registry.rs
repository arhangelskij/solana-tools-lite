use crate::extensions::traits::ProtocolAnalyzer;
use std::sync::{Arc, OnceLock};

static ANALYZERS: OnceLock<Vec<Arc<dyn ProtocolAnalyzer>>> = OnceLock::new();

/// Register protocol analyzers (call once at startup).
/// 
/// This should be called by the extensions crate's `init()` function.
/// Subsequent calls are ignored (OnceLock guarantees single initialization).
/// 
/// # Example
/// ```ignore
/// use std::sync::Arc;
/// 
/// // Register all protocol analyzers at startup
/// let analyzers = vec![/* Arc<dyn ProtocolAnalyzer> instances */];
/// solana_tools_lite::extensions::registry::register(analyzers);
/// ```
pub fn register(analyzers: Vec<Arc<dyn ProtocolAnalyzer>>) {
    let _ = ANALYZERS.set(analyzers); // Ignore error if already set
}

/// Returns a list of all registered protocol analyzers.
///
/// Returns an empty slice if no analyzers have been registered yet.
/// The returned reference is to a static slice, avoiding allocations.
pub fn get_all_analyzers() -> &'static [Arc<dyn ProtocolAnalyzer>] {
    ANALYZERS.get()
        .map(|v| v.as_slice())
        .unwrap_or(&[])
}
