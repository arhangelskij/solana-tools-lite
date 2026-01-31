use crate::analysis::light_protocol::{LightProtocol, constants};
use solana_tools_lite::extensions::ProtocolAnalyzer;
use solana_tools_lite::models::analysis::{AnalysisWarning, PrivacyLevel, TxAnalysis};
use solana_tools_lite::models::hash_base58::HashBase58;
use solana_tools_lite::models::instruction::Instruction;
use solana_tools_lite::models::message::{Message, MessageHeader, MessageLegacy};
use solana_tools_lite::models::pubkey_base58::PubkeyBase58;

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
fn test_malformed_instruction_too_short() {
    let analyzer = LightProtocol;
    let program_id = PubkeyBase58::try_from(constants::COMPRESSED_TOKEN_PROGRAM_ID).unwrap();
    let signer = PubkeyBase58::try_from("54pMAtV1S7S9B6V95eU7x6fA5Fz5xY6gR8H9N7V1p2A3").unwrap();
    
    let instr = Instruction {
        program_id_index: 1,
        accounts: vec![0],
        data: vec![], // Empty data
    };
    
    let message = Message::Legacy(MessageLegacy {
        header: MessageHeader {
            num_required_signatures: 1,
            num_readonly_signed_accounts: 0,
            num_readonly_unsigned_accounts: 0,
        },
        account_keys: vec![signer.clone(), program_id],
        recent_blockhash: HashBase58([0u8; 32]),
        instructions: vec![instr],
    });
    
    let mut analysis = empty_analysis();
    analyzer.analyze(&message, &message.account_keys(), &signer, &mut analysis);
    
    // Should add malformed instruction warning
    assert!(analysis.warnings.iter().any(|w| matches!(w, AnalysisWarning::MalformedInstruction)));
    // Should not add any extension actions
    assert!(analysis.extension_actions.is_empty());
    // Should not increment counts
    assert_eq!(analysis.confidential_ops_count, 0);
    assert_eq!(analysis.storage_ops_count, 0);
}

#[test]
fn test_invalid_discriminator() {
    let analyzer = LightProtocol;
    let program_id = PubkeyBase58::try_from(constants::COMPRESSED_TOKEN_PROGRAM_ID).unwrap();
    let signer = PubkeyBase58::try_from("54pMAtV1S7S9B6V95eU7x6fA5Fz5xY6gR8H9N7V1p2A3").unwrap();
    
    let instr = Instruction {
        program_id_index: 1,
        accounts: vec![0],
        data: vec![255], // Invalid 1-byte discriminator
    };
    
    let message = Message::Legacy(MessageLegacy {
        header: MessageHeader {
            num_required_signatures: 1,
            num_readonly_signed_accounts: 0,
            num_readonly_unsigned_accounts: 0,
        },
        account_keys: vec![signer.clone(), program_id],
        recent_blockhash: HashBase58([0u8; 32]),
        instructions: vec![instr],
    });
    
    let mut analysis = empty_analysis();
    analyzer.analyze(&message, &message.account_keys(), &signer, &mut analysis);
    
    // Should create Unknown action
    assert_eq!(analysis.extension_actions.len(), 1);
    // Should not increment counts for unknown actions
    assert_eq!(analysis.confidential_ops_count, 0);
    assert_eq!(analysis.storage_ops_count, 0);
}

#[test]
fn test_multiple_instructions_counting() {
    let analyzer = LightProtocol;
    let program_id = PubkeyBase58::try_from(constants::COMPRESSED_TOKEN_PROGRAM_ID).unwrap();
    let signer = PubkeyBase58::try_from("54pMAtV1S7S9B6V95eU7x6fA5Fz5xY6gR8H9N7V1p2A3").unwrap();
    
    let instructions = vec![
        Instruction {
            program_id_index: 1,
            accounts: vec![0],
            data: vec![constants::DISCRIMINATOR_CTOKEN_TRANSFER],
        },
        Instruction {
            program_id_index: 1,
            accounts: vec![0],
            data: vec![constants::DISCRIMINATOR_CTOKEN_MINT_TO],
        },
        Instruction {
            program_id_index: 1,
            accounts: vec![0],
            data: vec![constants::DISCRIMINATOR_CREATE_TOKEN_ACCOUNT],
        },
    ];
    
    let message = Message::Legacy(MessageLegacy {
        header: MessageHeader {
            num_required_signatures: 1,
            num_readonly_signed_accounts: 0,
            num_readonly_unsigned_accounts: 0,
        },
        account_keys: vec![signer.clone(), program_id],
        recent_blockhash: HashBase58([0u8; 32]),
        instructions,
    });
    
    let mut analysis = empty_analysis();
    analyzer.analyze(&message, &message.account_keys(), &signer, &mut analysis);
    
    // Should process all three instructions
    assert_eq!(analysis.extension_actions.len(), 3);
    // Should count: 2 confidential (CTokenTransfer, CTokenMintTo) + 1 storage (CreateTokenAccount)
    assert_eq!(analysis.confidential_ops_count, 2);
    assert_eq!(analysis.storage_ops_count, 1);
    
    // Privacy level should reflect confidential operations
    assert!(analysis.confidential_ops_count > 0, "Should have confidential operations");
}

#[test]
fn test_signer_not_in_accounts() {
    let analyzer = LightProtocol;
    let program_id = PubkeyBase58::try_from(constants::COMPRESSED_TOKEN_PROGRAM_ID).unwrap();
    let signer = PubkeyBase58::try_from("54pMAtV1S7S9B6V95eU7x6fA5Fz5xY6gR8H9N7V1p2A3").unwrap();
    let other_account = PubkeyBase58::try_from("11111111111111111111111111111111").unwrap();
    
    let instr = Instruction {
        program_id_index: 1,
        accounts: vec![1], // Points to other_account, not signer
        data: vec![constants::DISCRIMINATOR_CTOKEN_TRANSFER],
    };
    
    let message = Message::Legacy(MessageLegacy {
        header: MessageHeader {
            num_required_signatures: 1,
            num_readonly_signed_accounts: 0,
            num_readonly_unsigned_accounts: 0,
        },
        account_keys: vec![signer.clone(), program_id.clone(), other_account],
        recent_blockhash: HashBase58([0u8; 32]),
        instructions: vec![instr],
    });
    
    let mut analysis = empty_analysis();
    analyzer.analyze(&message, &message.account_keys(), &signer, &mut analysis);
    
    // Should add extension action and count it (privacy level depends on tx content, not signer role)
    assert_eq!(analysis.extension_actions.len(), 1);
    assert_eq!(analysis.confidential_ops_count, 1);
    assert_eq!(analysis.storage_ops_count, 0);
}

#[test]
fn test_8byte_discriminator_in_compressed_token_program() {
    let analyzer = LightProtocol;
    let program_id = PubkeyBase58::try_from(constants::COMPRESSED_TOKEN_PROGRAM_ID).unwrap();
    let signer = PubkeyBase58::try_from("54pMAtV1S7S9B6V95eU7x6fA5Fz5xY6gR8H9N7V1p2A3").unwrap();
    
    // Token Interface Transfer (8-byte discriminator)
    let instr = Instruction {
        program_id_index: 1,
        accounts: vec![0],
        data: constants::DISCRIMINATOR_TOKEN_INTERFACE_TRANSFER.to_vec(),
    };
    
    let message = Message::Legacy(MessageLegacy {
        header: MessageHeader {
            num_required_signatures: 1,
            num_readonly_signed_accounts: 0,
            num_readonly_unsigned_accounts: 0,
        },
        account_keys: vec![signer.clone(), program_id],
        recent_blockhash: HashBase58([0u8; 32]),
        instructions: vec![instr],
    });
    
    let mut analysis = empty_analysis();
    analyzer.analyze(&message, &message.account_keys(), &signer, &mut analysis);
    
    // Should recognize Token Interface Transfer as confidential
    assert_eq!(analysis.extension_actions.len(), 1);
    assert_eq!(analysis.confidential_ops_count, 1);
    assert_eq!(analysis.storage_ops_count, 0);
}
