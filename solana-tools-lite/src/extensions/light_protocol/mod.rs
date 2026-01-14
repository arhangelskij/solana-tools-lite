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
        if data.len() < 8 {
            return None;
        }

        let discriminator = &data[0..8];
        let action = if discriminator == constants::DISCRIMINATOR_CREATE_MINT {
            LightProtocolAction::CreateMint
        } else if discriminator == constants::DISCRIMINATOR_MINT_TO {
            LightProtocolAction::MintTo
        } else if discriminator == constants::DISCRIMINATOR_TRANSFER {
            LightProtocolAction::Transfer
        } else if discriminator == constants::DISCRIMINATOR_COMPRESS_SOL {
            let lamports = if data.len() >= 16 {
                Some(u64::from_le_bytes(data[8..16].try_into().unwrap_or([0u8; 8])))
            } else {
                None
            };
            LightProtocolAction::CompressSol { lamports }
        } else if discriminator == constants::DISCRIMINATOR_COMPRESS_TOKEN {
            let amount = if data.len() >= 16 {
                Some(u64::from_le_bytes(data[8..16].try_into().unwrap_or([0u8; 8])))
            } else {
                None
            };
            LightProtocolAction::CompressToken { amount }
        } else if discriminator == constants::DISCRIMINATOR_DECOMPRESS {
            LightProtocolAction::Decompress
        } else if discriminator == constants::DISCRIMINATOR_STATE_UPDATE {
            LightProtocolAction::StateUpdate
        } else if discriminator == constants::DISCRIMINATOR_CLOSE_ACCOUNT {
            LightProtocolAction::CloseAccount
        } else {
            let mut d = [0u8; 8];
            d.copy_from_slice(discriminator);
            LightProtocolAction::Unknown { discriminator: d }
        };

        Some(AnalysisExtensionAction::LightProtocol(action))
    }
}
