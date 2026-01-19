use thiserror::Error;

/// Errors specific to Light Protocol analysis.
#[derive(Debug, Error)]
pub enum LightError { //TODO: think about 
    #[error("Instruction data too short: expected {0}, got {1}")]
    InstructionTooShort(usize, usize),

    #[error("Invalid instruction discriminator")]
    InvalidDiscriminator,

    #[error("Failed to parse instruction data: {0}")]
    ParseFailed(String),

    #[error("Invalid Program ID: {0}")]
    InvalidProgramId(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),
}
