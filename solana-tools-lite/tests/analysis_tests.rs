use solana_tools_lite::handlers::analysis::analyze_transaction;
use solana_tools_lite::models::analysis::{AnalysisWarning, TokenProgramKind};
use solana_tools_lite::models::instruction::Instruction;
use solana_tools_lite::models::message::{Message, MessageAddressTableLookup, MessageHeader, MessageLegacy, MessageV0};
use solana_tools_lite::models::pubkey_base58::PubkeyBase58;
use solana_tools_lite::models::hash_base58::HashBase58;
use solana_tools_lite::constants::programs;

#[test]
fn analyze_legacy_system_transfer() {
    let signer = PubkeyBase58::from([1u8; 32]);
    let recipient = PubkeyBase58::from([2u8; 32]);
    let system_program = PubkeyBase58::try_from(programs::SYSTEM_PROGRAM_ID).unwrap();
    let blockhash = HashBase58([0u8; 32]);

    let mut data = Vec::new();
    data.extend_from_slice(&2u32.to_le_bytes()); // SystemProgram::Transfer
    data.extend_from_slice(&1_500u64.to_le_bytes());

    let instr = Instruction {
        program_id_index: 2,
        accounts: vec![0, 1],
        data,
    };

    let msg = Message::Legacy(MessageLegacy {
        header: MessageHeader {
            num_required_signatures: 1,
            num_readonly_signed_accounts: 0,
            num_readonly_unsigned_accounts: 1,
        },
        account_keys: vec![signer.clone(), recipient, system_program],
        recent_blockhash: blockhash,
        instructions: vec![instr],
    });

    let analysis = analyze_transaction(&msg, &signer, None);

    assert_eq!(analysis.transfers.len(), 1);
    assert_eq!(analysis.total_sol_send_by_signer, 1_500);
    assert_eq!(analysis.base_fee_lamports, 5000);
    assert!(analysis.is_fee_payer, "signer should be fee payer");
    assert!(analysis.priority_fee_lamports.is_none());
    assert!(analysis.warnings.is_empty());
}

#[test]
fn analyze_v0_missing_lookup_tables_warns() {
    let signer = PubkeyBase58::from([3u8; 32]);
    let blockhash = HashBase58([0u8; 32]);
    let lookup_key = PubkeyBase58::from([4u8; 32]);

    let msg = Message::V0(MessageV0 {
        header: MessageHeader {
            num_required_signatures: 1,
            num_readonly_signed_accounts: 0,
            num_readonly_unsigned_accounts: 0,
        },
        account_keys: vec![signer.clone()],
        recent_blockhash: blockhash,
        instructions: Vec::new(),
        address_table_lookups: vec![MessageAddressTableLookup {
            account_key: lookup_key,
            writable_indexes: vec![0],
            readonly_indexes: vec![1],
        }],
    });

    let analysis = analyze_transaction(&msg, &signer, None);
    assert!(analysis.warnings.iter().any(|w| matches!(w, AnalysisWarning::LookupTablesNotProvided)));
}

#[test]
fn analyze_compute_budget_sets_priority_fee() {
    let signer = PubkeyBase58::from([9u8; 32]);
    let program = PubkeyBase58::try_from(programs::COMPUTE_BUDGET_ID).unwrap();
    let other = PubkeyBase58::from([8u8; 32]);

    let mut data_limit = vec![2u8];
    data_limit.extend_from_slice(&300_000u32.to_le_bytes());

    let mut data_price = vec![3u8];
    data_price.extend_from_slice(&10_000u64.to_le_bytes());

    let msg = Message::Legacy(MessageLegacy {
        header: MessageHeader {
            num_required_signatures: 1,
            num_readonly_signed_accounts: 0,
            num_readonly_unsigned_accounts: 1,
        },
        account_keys: vec![signer.clone(), other, program],
        recent_blockhash: HashBase58([7u8; 32]),
        instructions: vec![
            Instruction {
                program_id_index: 2,
                accounts: vec![],
                data: data_limit,
            },
            Instruction {
                program_id_index: 2,
                accounts: vec![],
                data: data_price,
            },
        ],
    });

    let analysis = analyze_transaction(&msg, &signer, None);
    let (fee, estimated) = analysis.priority_fee_lamports.expect("priority fee expected");
    assert!(!estimated, "limit provided, fee should not be estimated");
    assert!(fee > 0, "priority fee should be positive");
}

#[test]
fn analyze_v0_missing_lookup_table_key_warns() {
    let signer = PubkeyBase58::from([5u8; 32]);
    let blockhash = HashBase58([1u8; 32]);
    let lookup_key = PubkeyBase58::from([6u8; 32]);

    let msg = Message::V0(MessageV0 {
        header: MessageHeader {
            num_required_signatures: 1,
            num_readonly_signed_accounts: 0,
            num_readonly_unsigned_accounts: 0,
        },
        account_keys: vec![signer.clone()],
        recent_blockhash: blockhash,
        instructions: Vec::new(),
        address_table_lookups: vec![MessageAddressTableLookup {
            account_key: lookup_key.clone(),
            writable_indexes: vec![0],
            readonly_indexes: vec![1],
        }],
    });

    let tables = crate::serde::LookupTableEntry {
        writable: vec![],
        readonly: vec![],
    };
    let analysis = analyze_transaction(&msg, &signer, Some(&tables));

    assert!(analysis
        .warnings
        .iter()
        .any(|w| matches!(w, AnalysisWarning::LookupTableMissing(key) if *key == lookup_key)));
}

#[test]
fn analyze_token_and_unknown_programs_warn() {
    let signer = PubkeyBase58::from([10u8; 32]);
    let other = PubkeyBase58::from([11u8; 32]);
    let token_program = PubkeyBase58::try_from(programs::TOKEN_PROGRAM_ID).unwrap();
    let token_2022_program = PubkeyBase58::try_from(programs::TOKEN_2022_PROGRAM_ID).unwrap();
    let unknown_program = PubkeyBase58::from([12u8; 32]);

    let msg = Message::Legacy(MessageLegacy {
        header: MessageHeader {
            num_required_signatures: 1,
            num_readonly_signed_accounts: 0,
            num_readonly_unsigned_accounts: 1,
        },
        account_keys: vec![
            signer.clone(),
            other,
            token_program,
            token_2022_program,
            unknown_program.clone(),
        ],
        recent_blockhash: HashBase58([3u8; 32]),
        instructions: vec![
            Instruction {
                program_id_index: 2,
                accounts: vec![],
                data: vec![],
            },
            Instruction {
                program_id_index: 3,
                accounts: vec![],
                data: vec![],
            },
            Instruction {
                program_id_index: 4,
                accounts: vec![],
                data: vec![],
            },
        ],
    });

    let analysis = analyze_transaction(&msg, &signer, None);

    assert!(analysis.warnings.iter().any(|w| {
        matches!(
            w,
            AnalysisWarning::TokenTransferDetected(TokenProgramKind::SplToken)
        )
    }));
    assert!(analysis.warnings.iter().any(|w| {
        matches!(
            w,
            AnalysisWarning::TokenTransferDetected(TokenProgramKind::Token2022)
        )
    }));
    assert!(analysis.warnings.iter().any(|w| {
        matches!(w, AnalysisWarning::UnknownProgram { program_id } if *program_id == unknown_program)
    }));
}
