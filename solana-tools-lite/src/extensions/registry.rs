use crate::extensions::light_protocol::LightProtocol;
use crate::extensions::traits::ProtocolAnalyzer;

/// Returns a list of all active protocol analyzers.
///
/// This registry acts as the single source of truth for enabled extensions.
/// The core analysis loop calls this function to get the analyzers, remaining
/// agnostic to the specific types being instantiated.
pub fn get_all_analyzers() -> Vec<Box<dyn ProtocolAnalyzer>> {
    vec![
        Box::new(LightProtocol),
        // Future extensions (e.g., Arcium) will be added here.
    ]
}
