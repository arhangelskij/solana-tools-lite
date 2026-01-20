/// Mock Protocol constants for testing multiple protocol support.
use solana_tools_lite::{ToolError, models::pubkey_base58::PubkeyBase58};

/// Mock Protocol program ID (using Arcium v0.6.3 program ID for testing).
pub const MOCK_PROTOCOL_PROGRAM_ID: &str = "Arcj82pX7HxYKLR92qvgZUAd7vGS1k4hQvAFcPATFdEQ";

/// Mock Protocol instruction discriminator.
pub const DISCRIMINATOR_MOCK_ACTION: [u8; 8] = [42, 42, 42, 42, 42, 42, 42, 42];

/// Returns the Mock Protocol program ID.
pub fn supported_programs() -> Result<&'static [PubkeyBase58], ToolError> {
    use std::sync::OnceLock;
    
    static PROGRAMS: OnceLock<Result<Vec<PubkeyBase58>, ToolError>> = OnceLock::new();
    
    let result = PROGRAMS.get_or_init(|| {
        let mock_program = PubkeyBase58::try_from(MOCK_PROTOCOL_PROGRAM_ID)
            .map_err(|_| ToolError::ConfigurationError("MOCK_PROTOCOL_PROGRAM_ID".to_string()))?;
        
        Ok(vec![mock_program])
    });
    
    match result {
        Ok(vec) => Ok(vec.as_slice()),
        Err(e) => Err(ToolError::ConfigurationError(format!("Failed to initialize Mock Protocol programs: {}", e))),
    }
}
