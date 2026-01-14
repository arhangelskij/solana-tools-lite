use crate::extensions::traits::ProtocolAnalyzer;
use crate::models::extensions::{AnalysisExtensionAction, LightProtocolAction};
use crate::models::pubkey_base58::PubkeyBase58;

pub mod constants;

/// Analyzer for Light Protocol (ZK Compression).
pub struct LightProtocol;

impl ProtocolAnalyzer for LightProtocol {
    fn analyze(&self, program_id: &PubkeyBase58, data: &[u8]) -> Option<AnalysisExtensionAction> {
        // Check if program ID is one of the known Light Protocol programs
        let is_light = program_id == constants::light_system_program()
            || program_id == constants::account_compression_program()
            || program_id == constants::compressed_token_program();

        if !is_light {
            return None;
        }

        // Check data length for discriminator
        if data.len() < constants::DISCRIMINATOR_SIZE {
            return None;
        }

        let discriminator: [u8; constants::DISCRIMINATOR_SIZE] = data[0..constants::DISCRIMINATOR_SIZE].try_into().unwrap_or([0u8; constants::DISCRIMINATOR_SIZE]);
        let action = match discriminator {
            constants::DISCRIMINATOR_CREATE_MINT => LightProtocolAction::CreateMint,
            constants::DISCRIMINATOR_MINT_TO => LightProtocolAction::MintTo,
            constants::DISCRIMINATOR_TRANSFER => LightProtocolAction::Transfer,
            constants::DISCRIMINATOR_COMPRESS_SOL => {
                let lamports = if data.len() >= constants::DISCRIMINATOR_SIZE + constants::U64_SIZE {
                    Some(u64::from_le_bytes(data[constants::DISCRIMINATOR_SIZE..constants::DISCRIMINATOR_SIZE + constants::U64_SIZE].try_into().unwrap_or([0u8; 8])))
                } else {
                    None
                };
                LightProtocolAction::CompressSol { lamports }
            }
            constants::DISCRIMINATOR_COMPRESS_TOKEN => {
                let amount = if data.len() >= constants::DISCRIMINATOR_SIZE + constants::U64_SIZE {
                    Some(u64::from_le_bytes(data[constants::DISCRIMINATOR_SIZE..constants::DISCRIMINATOR_SIZE + constants::U64_SIZE].try_into().unwrap_or([0u8; 8])))
                } else {
                    None
                };
                LightProtocolAction::CompressToken { amount }
            }
            constants::DISCRIMINATOR_DECOMPRESS => LightProtocolAction::Decompress,
            constants::DISCRIMINATOR_STATE_UPDATE => LightProtocolAction::StateUpdate,
            constants::DISCRIMINATOR_CLOSE_ACCOUNT => LightProtocolAction::CloseAccount,
            _ => LightProtocolAction::Unknown {
                discriminator,
            },
        };

        Some(AnalysisExtensionAction::LightProtocol(action))
    }
}
