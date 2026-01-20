//! Integration tests for presenter output with multiple protocols.
//!
//! Tests verify that extension actions and notices are correctly displayed
//! when multiple protocols are present in a single transaction.

use solana_tools_lite::handlers::analysis::analyze_transaction;
use solana_tools_lite::models::pubkey_base58::PubkeyBase58;
use solana_tools_lite::models::message::{Message, MessageHeader, MessageLegacy};
use solana_tools_lite::models::instruction::Instruction;
use solana_tools_lite::models::hash_base58::HashBase58;
use extensions::analysis::light_protocol::constants::DISCRIMINATOR_COMPRESS_SOL;
use solana_tools_lite_cli::flows::presenter::Presentable;
use solana_tools_lite_cli::flows::presenter::sign_tx_presenter::SignTxPresentation;

fn build_light_compress_message(signer: &PubkeyBase58, amount_lamports: u64) -> Message {
    let light_system_program = PubkeyBase58::try_from("Lighton6oQpVkeewmo2mcPTQQp7kYHr4fWpAgJyEmDX").unwrap();
    
    // CompressSol instruction data: discriminator + amount
    let mut data = DISCRIMINATOR_COMPRESS_SOL.to_vec();
    data.extend_from_slice(&amount_lamports.to_le_bytes());
    
    let instr = Instruction {
        program_id_index: 1, // Light System Program at index 1
        accounts: vec![0, 2, 3], // signer + 2 accounts
        data,
    };
    
    Message::Legacy(MessageLegacy {
        header: MessageHeader {
            num_required_signatures: 1,
            num_readonly_signed_accounts: 0,
            num_readonly_unsigned_accounts: 2,
        },
        account_keys: vec![
            signer.clone(),
            light_system_program,
            PubkeyBase58([2u8; 32]),
            PubkeyBase58([3u8; 32]),
        ],
        recent_blockhash: HashBase58([4u8; 32]),
        instructions: vec![instr],
    })
}

#[test]
fn test_light_protocol_single_instruction_presenter_output() {
    // Initialize extensions
    extensions::init();
    
    let signer = PubkeyBase58::try_from("7ZD7xmv1Ccvoqj28aPKwpJmzSBafkwXNAV3aGhBo5nSi").unwrap();
    let message = build_light_compress_message(&signer, 5_000_000_000); // 5 SOL
    
    // Analyze transaction
    let analysis = analyze_transaction(&message, &signer, None);
    
    // Use real presenter
    let presentation = SignTxPresentation {
        analysis: Some(&analysis),
        summary_payload: None,
    };
    
    eprintln!("\n=== SINGLE LIGHT PROTOCOL INSTRUCTION ===");
    let _ = presentation.present(false, false, true);
    
    // Verify extension actions
    assert_eq!(analysis.extension_actions.len(), 1, "Should have 1 Light Protocol action");
    assert_eq!(
        analysis.extension_actions[0].protocol_name(),
        "Light Protocol",
        "Protocol name should be 'Light Protocol'"
    );
    assert!(
        analysis.extension_actions[0].description().contains("Compress"),
        "Action should be Compress"
    );
    
    // Verify extension notices
    assert_eq!(analysis.extension_notices.len(), 1, "Should have 1 notice");
    assert!(
        analysis.extension_notices[0].contains("ZK COMPRESSION"),
        "Notice should mention ZK Compression"
    );
}

#[test]
fn test_light_protocol_multiple_instructions_presenter_output() {
    // Initialize extensions
    extensions::init();
    
    let signer = PubkeyBase58::try_from("7ZD7xmv1Ccvoqj28aPKwpJmzSBafkwXNAV3aGhBo5nSi").unwrap();
    let light_system_program = PubkeyBase58::try_from("Lighton6oQpVkeewmo2mcPTQQp7kYHr4fWpAgJyEmDX").unwrap();
    
    // Two CompressSol instructions: 3 SOL + 2 SOL
    let mut data1 = DISCRIMINATOR_COMPRESS_SOL.to_vec();
    data1.extend_from_slice(&3_000_000_000u64.to_le_bytes());
    
    let mut data2 = DISCRIMINATOR_COMPRESS_SOL.to_vec();
    data2.extend_from_slice(&2_000_000_000u64.to_le_bytes());
    
    let instr1 = Instruction {
        program_id_index: 1,
        accounts: vec![0, 2, 3],
        data: data1,
    };
    
    let instr2 = Instruction {
        program_id_index: 1,
        accounts: vec![0, 2, 3],
        data: data2,
    };
    
    let message = Message::Legacy(MessageLegacy {
        header: MessageHeader {
            num_required_signatures: 1,
            num_readonly_signed_accounts: 0,
            num_readonly_unsigned_accounts: 2,
        },
        account_keys: vec![
            signer.clone(),
            light_system_program,
            PubkeyBase58([2u8; 32]),
            PubkeyBase58([3u8; 32]),
        ],
        recent_blockhash: HashBase58([4u8; 32]),
        instructions: vec![instr1, instr2],
    });
    
    // Analyze transaction
    let analysis = analyze_transaction(&message, &signer, None);
    
    // Use real presenter
    let presentation = SignTxPresentation {
        analysis: Some(&analysis),
        summary_payload: None,
    };
    
    eprintln!("\n=== MULTIPLE LIGHT PROTOCOL INSTRUCTIONS ===");
    let _ = presentation.present(false, false, true);
    
    // Verify extension actions - should have 2 actions
    assert_eq!(analysis.extension_actions.len(), 2, "Should have 2 Light Protocol actions");
}

#[test]
fn test_unknown_program_presenter_output() {
    // Initialize extensions
    extensions::init();
    
    let signer = PubkeyBase58::try_from("7ZD7xmv1Ccvoqj28aPKwpJmzSBafkwXNAV3aGhBo5nSi").unwrap();
    let unknown_program = PubkeyBase58::try_from("JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG").unwrap();
    
    let instr = Instruction {
        program_id_index: 1,
        accounts: vec![0],
        data: vec![1, 2, 3, 4],
    };
    
    let message = Message::Legacy(MessageLegacy {
        header: MessageHeader {
            num_required_signatures: 1,
            num_readonly_signed_accounts: 0,
            num_readonly_unsigned_accounts: 0,
        },
        account_keys: vec![signer.clone(), unknown_program],
        recent_blockhash: HashBase58([255u8; 32]),
        instructions: vec![instr],
    });
    
    // Analyze transaction
    let analysis = analyze_transaction(&message, &signer, None);
    
    // Use real presenter
    let presentation = SignTxPresentation {
        analysis: Some(&analysis),
        summary_payload: None,
    };
    
    eprintln!("\n=== UNKNOWN PROGRAM TRANSACTION ===");
    let _ = presentation.present(false, false, true);
    
    // Verify no extension actions
    assert_eq!(analysis.extension_actions.len(), 0, "Should have no extension actions");
    
    // Verify unknown program warning exists
    assert!(
        analysis.warnings.iter().any(|w| matches!(
            w,
            solana_tools_lite::models::analysis::AnalysisWarning::UnknownProgram { .. }
        )),
        "Should have unknown program warning"
    );
}

#[test]
fn test_light_protocol_and_unknown_program_presenter_output() {
    // Initialize extensions
    extensions::init();
    
    let signer = PubkeyBase58::try_from("7ZD7xmv1Ccvoqj28aPKwpJmzSBafkwXNAV3aGhBo5nSi").unwrap();
    let light_system_program = PubkeyBase58::try_from("Lighton6oQpVkeewmo2mcPTQQp7kYHr4fWpAgJyEmDX").unwrap();
    let unknown_program = PubkeyBase58::try_from("JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG").unwrap();
    
    // Light Protocol CompressSol instruction
    let mut light_data = DISCRIMINATOR_COMPRESS_SOL.to_vec();
    light_data.extend_from_slice(&5_000_000_000u64.to_le_bytes());
    
    let light_instr = Instruction {
        program_id_index: 1,
        accounts: vec![0, 2, 3],
        data: light_data,
    };
    
    // Unknown program instruction
    let unknown_instr = Instruction {
        program_id_index: 4,
        accounts: vec![0],
        data: vec![1, 2, 3, 4],
    };
    
    let message = Message::Legacy(MessageLegacy {
        header: MessageHeader {
            num_required_signatures: 1,
            num_readonly_signed_accounts: 0,
            num_readonly_unsigned_accounts: 3,
        },
        account_keys: vec![
            signer.clone(),
            light_system_program,
            PubkeyBase58([2u8; 32]),
            PubkeyBase58([3u8; 32]),
            unknown_program,
        ],
        recent_blockhash: HashBase58([4u8; 32]),
        instructions: vec![light_instr, unknown_instr],
    });
    
    // Analyze transaction
    let analysis = analyze_transaction(&message, &signer, None);
    
    // Use real presenter
    let presentation = SignTxPresentation {
        analysis: Some(&analysis),
        summary_payload: None,
    };
    
    eprintln!("\n=== LIGHT PROTOCOL + UNKNOWN PROGRAM ===");
    let _ = presentation.present(false, false, true);
    
    // Verify Light Protocol action
    assert_eq!(analysis.extension_actions.len(), 1, "Should have 1 Light Protocol action");
    
    // Verify unknown program warning still exists
    assert!(
        analysis.warnings.iter().any(|w| matches!(
            w,
            solana_tools_lite::models::analysis::AnalysisWarning::UnknownProgram { .. }
        )),
        "Should have unknown program warning"
    );
}

#[test]
fn test_light_protocol_mixed_operations_presenter_output() {
    // Initialize extensions
    extensions::init();
    
    let signer = PubkeyBase58::try_from("7ZD7xmv1Ccvoqj28aPKwpJmzSBafkwXNAV3aGhBo5nSi").unwrap();
    let light_system_program = PubkeyBase58::try_from("Lighton6oQpVkeewmo2mcPTQQp7kYHr4fWpAgJyEmDX").unwrap();
    
    // CompressSol instruction (storage operation)
    let mut compress_data = DISCRIMINATOR_COMPRESS_SOL.to_vec();
    compress_data.extend_from_slice(&3_000_000_000u64.to_le_bytes());
    
    // MintTo instruction (confidential operation)
    // DISCRIMINATOR_MINT_TO = [241, 34, 48, 186, 37, 179, 123, 192]
    let mut mint_data = vec![241, 34, 48, 186, 37, 179, 123, 192];
    mint_data.extend_from_slice(&1_000_000_000u64.to_le_bytes());
    
    let compress_instr = Instruction {
        program_id_index: 1,
        accounts: vec![0, 2, 3],
        data: compress_data,
    };
    
    let mint_instr = Instruction {
        program_id_index: 1,
        accounts: vec![0, 2, 3],
        data: mint_data,
    };
    
    let message = Message::Legacy(MessageLegacy {
        header: MessageHeader {
            num_required_signatures: 1,
            num_readonly_signed_accounts: 0,
            num_readonly_unsigned_accounts: 2,
        },
        account_keys: vec![
            signer.clone(),
            light_system_program,
            PubkeyBase58([2u8; 32]),
            PubkeyBase58([3u8; 32]),
        ],
        recent_blockhash: HashBase58([4u8; 32]),
        instructions: vec![compress_instr, mint_instr],
    });
    
    // Analyze transaction
    let analysis = analyze_transaction(&message, &signer, None);
    
    // Use real presenter
    let presentation = SignTxPresentation {
        analysis: Some(&analysis),
        summary_payload: None,
    };
    
    eprintln!("\n=== LIGHT PROTOCOL MIXED OPERATIONS (Compress + MintTo) ===");
    let _ = presentation.present(false, false, true);
    
    // Verify both actions detected
    assert_eq!(analysis.extension_actions.len(), 2, "Should have 2 Light Protocol actions");
    
    // Verify action types
    let descriptions: Vec<String> = analysis.extension_actions.iter()
        .map(|a| a.description())
        .collect();
    
    assert!(descriptions.iter().any(|d| d.contains("Compress")), "Should have Compress action");
    assert!(descriptions.iter().any(|d| d.contains("Mint")), "Should have Mint action");
}


#[test]
fn test_light_protocol_mixed_and_unknown_program_presenter_output() {
    // Initialize extensions
    extensions::init();
    
    let signer = PubkeyBase58::try_from("7ZD7xmv1Ccvoqj28aPKwpJmzSBafkwXNAV3aGhBo5nSi").unwrap();
    let light_system_program = PubkeyBase58::try_from("Lighton6oQpVkeewmo2mcPTQQp7kYHr4fWpAgJyEmDX").unwrap();
    let unknown_program = PubkeyBase58::try_from("JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG").unwrap();
    
    // CompressSol instruction (storage operation)
    let mut compress_data = DISCRIMINATOR_COMPRESS_SOL.to_vec();
    compress_data.extend_from_slice(&3_000_000_000u64.to_le_bytes());
    
    // MintTo instruction (confidential operation)
    let mut mint_data = vec![241, 34, 48, 186, 37, 179, 123, 192];
    mint_data.extend_from_slice(&1_000_000_000u64.to_le_bytes());
    
    // Unknown program instruction
    let unknown_instr_data = vec![1, 2, 3, 4];
    
    let compress_instr = Instruction {
        program_id_index: 1,
        accounts: vec![0, 2, 3],
        data: compress_data,
    };
    
    let mint_instr = Instruction {
        program_id_index: 1,
        accounts: vec![0, 2, 3],
        data: mint_data,
    };
    
    let unknown_instr = Instruction {
        program_id_index: 4,
        accounts: vec![0],
        data: unknown_instr_data,
    };
    
    let message = Message::Legacy(MessageLegacy {
        header: MessageHeader {
            num_required_signatures: 1,
            num_readonly_signed_accounts: 0,
            num_readonly_unsigned_accounts: 3,
        },
        account_keys: vec![
            signer.clone(),
            light_system_program,
            PubkeyBase58([2u8; 32]),
            PubkeyBase58([3u8; 32]),
            unknown_program,
        ],
        recent_blockhash: HashBase58([4u8; 32]),
        instructions: vec![compress_instr, mint_instr, unknown_instr],
    });
    
    // Analyze transaction
    let analysis = analyze_transaction(&message, &signer, None);
    
    // Use real presenter
    let presentation = SignTxPresentation {
        analysis: Some(&analysis),
        summary_payload: None,
    };
    
    eprintln!("\n=== LIGHT PROTOCOL MIXED + UNKNOWN PROGRAM ===");
    let _ = presentation.present(false, false, true);
    
    // Verify Light Protocol actions
    assert_eq!(analysis.extension_actions.len(), 2, "Should have 2 Light Protocol actions");
    
    // Verify action types
    let descriptions: Vec<String> = analysis.extension_actions.iter()
        .map(|a| a.description())
        .collect();
    
    assert!(descriptions.iter().any(|d| d.contains("Compress")), "Should have Compress action");
    assert!(descriptions.iter().any(|d| d.contains("Mint")), "Should have Mint action");
    
    // Verify unknown program warning still exists
    assert!(
        analysis.warnings.iter().any(|w| matches!(
            w,
            solana_tools_lite::models::analysis::AnalysisWarning::UnknownProgram { .. }
        )),
        "Should have unknown program warning"
    );
}


#[test]
fn test_mock_protocol_single_instruction_presenter_output() {
    // Initialize extensions
    extensions::init();
    
    let signer = PubkeyBase58::try_from("7ZD7xmv1Ccvoqj28aPKwpJmzSBafkwXNAV3aGhBo5nSi").unwrap();
    let mock_program = PubkeyBase58::try_from("Arcj82pX7HxYKLR92qvgZUAd7vGS1k4hQvAFcPATFdEQ").unwrap();
    
    let instr = Instruction {
        program_id_index: 1,
        accounts: vec![0],
        data: vec![42, 42, 42, 42, 42, 42, 42, 42],
    };
    
    let message = Message::Legacy(MessageLegacy {
        header: MessageHeader {
            num_required_signatures: 1,
            num_readonly_signed_accounts: 0,
            num_readonly_unsigned_accounts: 0,
        },
        account_keys: vec![signer.clone(), mock_program],
        recent_blockhash: HashBase58([5u8; 32]),
        instructions: vec![instr],
    });
    
    // Analyze transaction
    let analysis = analyze_transaction(&message, &signer, None);
    
    // Use real presenter
    let presentation = SignTxPresentation {
        analysis: Some(&analysis),
        summary_payload: None,
    };
    
    eprintln!("\n=== MOCK PROTOCOL SINGLE INSTRUCTION ===");
    let _ = presentation.present(false, false, true);
    
    // Verify Mock Protocol action
    assert_eq!(analysis.extension_actions.len(), 1, "Should have 1 Mock Protocol action");
    assert_eq!(
        analysis.extension_actions[0].protocol_name(),
        "Mock Protocol",
        "Protocol name should be 'Mock Protocol'"
    );
    
    // Verify Mock Protocol notice
    assert_eq!(analysis.extension_notices.len(), 1, "Should have 1 notice");
    assert!(
        analysis.extension_notices[0].contains("MOCK PROTOCOL"),
        "Notice should mention Mock Protocol"
    );
}

#[test]
fn test_light_and_mock_protocol_presenter_output() {
    // Initialize extensions
    extensions::init();
    
    let signer = PubkeyBase58::try_from("7ZD7xmv1Ccvoqj28aPKwpJmzSBafkwXNAV3aGhBo5nSi").unwrap();
    let light_system_program = PubkeyBase58::try_from("Lighton6oQpVkeewmo2mcPTQQp7kYHr4fWpAgJyEmDX").unwrap();
    let mock_program = PubkeyBase58::try_from("Arcj82pX7HxYKLR92qvgZUAd7vGS1k4hQvAFcPATFdEQ").unwrap();
    
    // Light Protocol CompressSol instruction
    let mut light_data = DISCRIMINATOR_COMPRESS_SOL.to_vec();
    light_data.extend_from_slice(&2_000_000_000u64.to_le_bytes());
    
    // Mock Protocol instruction
    let mock_data = vec![42, 42, 42, 42, 42, 42, 42, 42];
    
    let light_instr = Instruction {
        program_id_index: 1,
        accounts: vec![0, 2, 3],
        data: light_data,
    };
    
    let mock_instr = Instruction {
        program_id_index: 4,
        accounts: vec![0],
        data: mock_data,
    };
    
    let message = Message::Legacy(MessageLegacy {
        header: MessageHeader {
            num_required_signatures: 1,
            num_readonly_signed_accounts: 0,
            num_readonly_unsigned_accounts: 3,
        },
        account_keys: vec![
            signer.clone(),
            light_system_program,
            PubkeyBase58([2u8; 32]),
            PubkeyBase58([3u8; 32]),
            mock_program,
        ],
        recent_blockhash: HashBase58([6u8; 32]),
        instructions: vec![light_instr, mock_instr],
    });
    
    // Analyze transaction
    let analysis = analyze_transaction(&message, &signer, None);
    
    // Use real presenter
    let presentation = SignTxPresentation {
        analysis: Some(&analysis),
        summary_payload: None,
    };
    
    eprintln!("\n=== LIGHT PROTOCOL + MOCK PROTOCOL ===");
    let _ = presentation.present(false, false, true);
    
    // Verify both protocols detected
    assert_eq!(analysis.extension_actions.len(), 2, "Should have 2 protocol actions");
    
    // Verify action types
    let protocol_names: Vec<&str> = analysis.extension_actions.iter()
        .map(|a| a.protocol_name())
        .collect();
    
    assert!(protocol_names.contains(&"Light Protocol"), "Should have Light Protocol");
    assert!(protocol_names.contains(&"Mock Protocol"), "Should have Mock Protocol");
    
    // Verify both notices
    assert_eq!(analysis.extension_notices.len(), 2, "Should have 2 notices");
    assert!(
        analysis.extension_notices.iter().any(|n| n.contains("ZK COMPRESSION")),
        "Should have Light Protocol notice"
    );
    assert!(
        analysis.extension_notices.iter().any(|n| n.contains("MOCK PROTOCOL")),
        "Should have Mock Protocol notice"
    );
}
