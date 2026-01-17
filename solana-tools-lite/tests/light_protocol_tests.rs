use solana_tools_lite::constants::programs;
use solana_tools_lite::extensions::light_protocol::constants::{
    self, DISCRIMINATOR_COMPRESS_SOL, DISCRIMINATOR_CREATE_MINT, DISCRIMINATOR_MINT_TO,
    DISCRIMINATOR_TRANSFER,
};
use solana_tools_lite::extensions::light_protocol::LightProtocol;
use solana_tools_lite::extensions::ProtocolAnalyzer;
use solana_tools_lite::models::analysis::{PrivacyLevel, TxAnalysis};
use solana_tools_lite::models::hash_base58::HashBase58;
use solana_tools_lite::models::instruction::Instruction;
use solana_tools_lite::models::message::{Message, MessageHeader, MessageLegacy};
use solana_tools_lite::models::pubkey_base58::PubkeyBase58;

fn mock_message(program_id: &PubkeyBase58, data: Vec<u8>, signer: &PubkeyBase58) -> Message {
    let instr = Instruction {
        program_id_index: 1, // program_id is at index 1
        accounts: vec![0],  // signer is at index 0
        data,
    };
    Message::Legacy(MessageLegacy {
        header: MessageHeader {
            num_required_signatures: 1,
            num_readonly_signed_accounts: 0,
            num_readonly_unsigned_accounts: 0,
        },
        account_keys: vec![signer.clone(), program_id.clone()],
        recent_blockhash: HashBase58([0u8; 32]),
        instructions: vec![instr],
    })
}

fn empty_analysis() -> TxAnalysis {
    TxAnalysis {
        transfers: vec![],
        base_fee_lamports: 0,
        priority_fee_lamports: None,
        total_fee_lamports: 0,
        total_sol_send_by_signer: 0,
        compute_unit_limit: None,
        compute_unit_price_micro: None,
        warnings: vec![],
        message_version: "legacy",
        privacy_level: PrivacyLevel::Public,
        extension_actions: vec![],
        extension_notices: vec![],
        confidential_ops_count: 0,
        storage_ops_count: 0,
        is_fee_payer: false,
        has_non_sol_assets: false,
    }
}

#[test]
fn test_detect_create_mint() {
    let analyzer = LightProtocol;
    let program_id = PubkeyBase58::try_from(constants::COMPRESSED_TOKEN_PROGRAM_ID).unwrap();
    let data = DISCRIMINATOR_CREATE_MINT.to_vec();
    let signer = PubkeyBase58::try_from("54pMAtV1S7S9B6V95eU7x6fA5Fz5xY6gR8H9N7V1p2A3").unwrap();
    let message = mock_message(&program_id, data, &signer);
    let mut analysis = empty_analysis();

    analyzer.analyze(&message, &message.account_keys(), &signer, &mut analysis);
    
    assert_eq!(analysis.storage_ops_count, 1);
    assert!(!analysis.extension_actions.is_empty());
}

#[test]
fn test_detect_mint_to() {
    let analyzer = LightProtocol;
    let program_id = PubkeyBase58::try_from(constants::COMPRESSED_TOKEN_PROGRAM_ID).unwrap();
    let data = DISCRIMINATOR_MINT_TO.to_vec();
    let signer = PubkeyBase58::try_from("54pMAtV1S7S9B6V95eU7x6fA5Fz5xY6gR8H9N7V1p2A3").unwrap();
    let message = mock_message(&program_id, data, &signer);
    let mut analysis = empty_analysis();

    analyzer.analyze(&message, &message.account_keys(), &signer, &mut analysis);
    
    assert_eq!(analysis.confidential_ops_count, 1);
}

#[test]
fn test_detect_transfer() {
    let analyzer = LightProtocol;
    let program_id = PubkeyBase58::try_from(constants::COMPRESSED_TOKEN_PROGRAM_ID).unwrap();
    let data = DISCRIMINATOR_TRANSFER.to_vec();
    let signer = PubkeyBase58::try_from("54pMAtV1S7S9B6V95eU7x6fA5Fz5xY6gR8H9N7V1p2A3").unwrap();
    let message = mock_message(&program_id, data, &signer);
    let mut analysis = empty_analysis();

    analyzer.analyze(&message, &message.account_keys(), &signer, &mut analysis);
    
    assert_eq!(analysis.confidential_ops_count, 1);
}

#[test]
fn test_detect_compress_sol() {
    let analyzer = LightProtocol;
    let program_id = PubkeyBase58::try_from(constants::LIGHT_SYSTEM_PROGRAM_ID).unwrap();
    
    let lamports: u64 = 1_000_000_000;
    let mut data = DISCRIMINATOR_COMPRESS_SOL.to_vec();
    data.extend_from_slice(&lamports.to_le_bytes());

    let signer = PubkeyBase58::try_from("54pMAtV1S7S9B6V95eU7x6fA5Fz5xY6gR8H9N7V1p2A3").unwrap();
    let message = mock_message(&program_id, data, &signer);
    let mut analysis = empty_analysis();

    analyzer.analyze(&message, &message.account_keys(), &signer, &mut analysis);
    
    assert_eq!(analysis.storage_ops_count, 1);
}

#[test]
fn test_ignore_system_program() {
    let analyzer = LightProtocol;
    let program_id = programs::system_program();
    let data = vec![0u8; 8]; 
    let signer = PubkeyBase58::try_from("54pMAtV1S7S9B6V95eU7x6fA5Fz5xY6gR8H9N7V1p2A3").unwrap();
    let message = mock_message(&program_id, data, &signer);
    let mut analysis = empty_analysis();

    analyzer.analyze(&message, &message.account_keys(), &signer, &mut analysis);
    
    assert_eq!(analysis.confidential_ops_count, 0);
    assert_eq!(analysis.storage_ops_count, 0);
}

#[test]
fn test_unknown_light_instruction() {
    let analyzer = LightProtocol;
    let program_id = PubkeyBase58::try_from(constants::COMPRESSED_TOKEN_PROGRAM_ID).unwrap();
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8]; 
    let signer = PubkeyBase58::try_from("54pMAtV1S7S9B6V95eU7x6fA5Fz5xY6gR8H9N7V1p2A3").unwrap();
    let message = mock_message(&program_id, data, &signer);
    let mut analysis = empty_analysis();

    analyzer.analyze(&message, &message.account_keys(), &signer, &mut analysis);
    
    // Unknown instructions don't increment counts but should add an extension action
    assert_eq!(analysis.confidential_ops_count, 0);
    assert!(!analysis.extension_actions.is_empty());
}

#[test]
fn test_signer_not_involved() {
    let analyzer = LightProtocol;
    let program_id = PubkeyBase58::try_from(constants::COMPRESSED_TOKEN_PROGRAM_ID).unwrap();
    let data = DISCRIMINATOR_TRANSFER.to_vec();
    
    // Signer is NOT in accounts
    let instr = Instruction {
        program_id_index: 0,
        accounts: vec![], // No accounts
        data,
    };
    let message = Message::Legacy(MessageLegacy {
        header: MessageHeader {
            num_required_signatures: 1,
            num_readonly_signed_accounts: 0,
            num_readonly_unsigned_accounts: 0,
        },
        account_keys: vec![program_id.clone()],
        recent_blockhash: HashBase58([0u8; 32]),
        instructions: vec![instr],
    });
    
    let mut analysis = empty_analysis();
    let signer = PubkeyBase58::try_from("54pMAtV1S7S9B6V95eU7x6fA5Fz5xY6gR8H9N7V1p2A3").unwrap();

    analyzer.analyze(&message, &message.account_keys(), &signer, &mut analysis);
    
    // Should NOT be counted because signer is not involved
    assert_eq!(analysis.confidential_ops_count, 0);
    // But it should still be in extension actions
    assert!(!analysis.extension_actions.is_empty());
}

#[test]
fn test_enrich_notice_dynamic() {
    let analyzer = LightProtocol;
    let mut analysis = empty_analysis();
    
    // Case 1: Only storage compression
    analysis.storage_ops_count = 1;
    let notice = analyzer.enrich_notice(&analysis).unwrap();
    assert!(!notice.contains("Valid proofs are required"));
    
    // Case 2: Confidential ops present
    analysis.confidential_ops_count = 1;
    let notice = analyzer.enrich_notice(&analysis).unwrap();
    assert!(notice.contains("Valid proofs are required"));
}
