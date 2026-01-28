//! Extensions for solana-tools-lite analysis
//!
//! This crate provides protocol-specific analyzers for transaction analysis.
//! Currently includes Light Protocol (ZK Compression) support.

pub mod analysis;

/// Initialize all protocol analyzers.
/// 
/// Call this once at application startup to register all available analyzers
/// with the core registry. This enables enhanced transaction analysis for
/// supported protocols (Light Protocol, etc.).
/// 
/// # Example
/// ```no_run
/// // Initialize protocol extensions at startup
/// extensions::init();
/// 
/// // Now the analysis engine will detect Light Protocol instructions
/// ```
/// 
/// # Note
/// 
/// This function can be called multiple times safely - only the first call
/// will register the analyzers (subsequent calls are no-ops).
pub fn init() {
    use std::sync::Arc;
    
    let analyzers: Vec<Arc<dyn solana_tools_lite::extensions::traits::ProtocolAnalyzer>> = vec![
        Arc::new(analysis::light_protocol::LightProtocol),
        // Future protocols can be added here:
        // Arc::new(analysis::arcium::Arcium),
    ];
    
    solana_tools_lite::extensions::registry::register(analyzers);
}

pub use analysis::ProgramId;
pub use analysis::LightProtocolAction;
