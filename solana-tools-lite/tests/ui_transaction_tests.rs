mod common;

use bs58;
use common::{deterministic_base58, deterministic_pubkey, deterministic_signature};
use solana_tools_lite::errors::TransactionParseError;
use solana_tools_lite::models::input_transaction::{
    UiCompiledInstruction, UiRawMessage, UiRawMessageLegacy, UiTransaction,
};
use solana_tools_lite::models::instruction::Instruction;
use solana_tools_lite::models::message::{
    Message, MessageAddressTableLookup, MessageHeader, MessageLegacy, MessageV0,
};
use solana_tools_lite::models::{hash_base58::HashBase58, transaction::Transaction};
use std::convert::TryFrom;

#[test]
fn legacy_transaction_to_ui_preserves_all_fields() {
    let required_signatures = 1;
    let header = MessageHeader {
        num_required_signatures: required_signatures,
        num_readonly_signed_accounts: 0,
        num_readonly_unsigned_accounts: 1,
    };

    let account_keys = vec![
        deterministic_pubkey(1),
        deterministic_pubkey(2),
        deterministic_pubkey(3),
    ];
    let account_key_strings: Vec<String> = account_keys.iter().map(|k| k.to_string()).collect();

    let blockhash_str = deterministic_base58(9, 32);
    let blockhash = HashBase58::try_from(blockhash_str.as_str()).unwrap();

    let instruction_bytes = vec![7u8, 8, 9, 10];
    let instruction_base58 = bs58::encode(&instruction_bytes).into_string();
    let instructions = vec![Instruction {
        program_id_index: 1,
        accounts: vec![0, 2],
        data: instruction_bytes.clone(),
    }];

    let tx = Transaction {
        signatures: vec![deterministic_signature(5)],
        message: Message::Legacy(MessageLegacy {
            header,
            account_keys: account_keys.clone(),
            recent_blockhash: blockhash,
            instructions: instructions.clone(),
        }),
    };

    let ui = UiTransaction::from(&tx);
    assert_eq!(ui.signatures.len(), 1);
    assert_eq!(
        ui.signatures[0],
        bs58::encode(deterministic_signature(5).to_bytes()).into_string()
    );

    match ui.message {
        UiRawMessage::Legacy(ref legacy) => {
            assert_eq!(legacy.header.num_required_signatures, required_signatures);
            assert_eq!(legacy.account_keys, account_key_strings);
            assert_eq!(legacy.recent_blockhash, blockhash_str);
            assert_eq!(legacy.instructions.len(), instructions.len());
            assert_eq!(legacy.instructions[0].program_id_index, 1);
            assert_eq!(legacy.instructions[0].accounts, vec![0, 2]);
            assert_eq!(legacy.instructions[0].data, instruction_base58);
        }
        UiRawMessage::V0(_) => panic!("expected legacy message"),
    }
}

#[test]
fn v0_transaction_roundtrip_via_ui_transaction() {
    let required_signatures = 2;
    let header = MessageHeader {
        num_required_signatures: required_signatures,
        num_readonly_signed_accounts: 0,
        num_readonly_unsigned_accounts: 1,
    };

    let account_keys = vec![
        deterministic_pubkey(11),
        deterministic_pubkey(12),
        deterministic_pubkey(13),
    ];
    let account_key_strings: Vec<String> = account_keys.iter().map(|k| k.to_string()).collect();

    let blockhash_str = deterministic_base58(7, 32);
    let blockhash = HashBase58::try_from(blockhash_str.as_str()).unwrap();

    let instructions = vec![
        Instruction {
            program_id_index: 1,
            accounts: vec![0, 1],
            data: vec![42, 43, 44],
        },
        Instruction {
            program_id_index: 2,
            accounts: vec![2],
            data: vec![1, 2, 3, 4],
        },
    ];

    let lookups = vec![MessageAddressTableLookup {
        account_key: deterministic_pubkey(31),
        writable_indexes: vec![0, 2],
        readonly_indexes: vec![3],
    }];

    let tx = Transaction {
        signatures: vec![deterministic_signature(8), deterministic_signature(9)],
        message: Message::V0(MessageV0 {
            header,
            account_keys: account_keys.clone(),
            recent_blockhash: blockhash,
            instructions: instructions.clone(),
            address_table_lookups: lookups.clone(),
        }),
    };

    let ui = UiTransaction::from(&tx);
    assert!(
        matches!(ui.message, UiRawMessage::V0(_)),
        "expected UiRawMessage::V0"
    );

    let roundtrip = Transaction::try_from(ui).expect("roundtrip conversion");
    assert_eq!(roundtrip.signatures.len(), 2);
    assert_eq!(
        roundtrip.signatures[0].to_bytes(),
        deterministic_signature(8).to_bytes()
    );
    assert_eq!(
        roundtrip.signatures[1].to_bytes(),
        deterministic_signature(9).to_bytes()
    );

    match roundtrip.message {
        Message::V0(msg) => {
            assert_eq!(msg.header.num_required_signatures, required_signatures);
            assert_eq!(
                msg.account_keys.iter().map(|k| k.to_string()).collect::<Vec<_>>(),
                account_key_strings
            );
            assert_eq!(msg.recent_blockhash.to_string(), blockhash_str);
            assert_eq!(msg.instructions.len(), instructions.len());
            for (ix_roundtrip, ix_expected) in msg.instructions.iter().zip(instructions.iter()) {
                assert_eq!(ix_roundtrip.program_id_index, ix_expected.program_id_index);
                assert_eq!(ix_roundtrip.accounts, ix_expected.accounts);
                assert_eq!(ix_roundtrip.data, ix_expected.data);
            }
            assert_eq!(msg.address_table_lookups.len(), lookups.len());
            for (lut_roundtrip, lut_expected) in msg
                .address_table_lookups
                .iter()
                .zip(lookups.iter())
            {
                assert_eq!(lut_roundtrip.account_key.to_string(), lut_expected.account_key.to_string());
                assert_eq!(lut_roundtrip.writable_indexes, lut_expected.writable_indexes);
                assert_eq!(lut_roundtrip.readonly_indexes, lut_expected.readonly_indexes);
            }
        }
        Message::Legacy(_) => panic!("expected v0 message"),
    }
}

#[test]
fn ui_transaction_rejects_short_signature() {
    let header = MessageHeader {
        num_required_signatures: 1,
        num_readonly_signed_accounts: 0,
        num_readonly_unsigned_accounts: 1,
    };

    let account_keys = vec![deterministic_base58(55, 32), deterministic_base58(56, 32)];
    let blockhash = deterministic_base58(10, 32);
    let instruction_data = bs58::encode(&[1u8, 2, 3]).into_string();

    let ui_tx = UiTransaction {
        signatures: vec![bs58::encode(&[5u8; 10]).into_string()],
        message: UiRawMessage::Legacy(UiRawMessageLegacy {
            header,
            account_keys,
            recent_blockhash: blockhash,
            instructions: vec![UiCompiledInstruction {
                program_id_index: 1,
                accounts: vec![0, 1],
                data: instruction_data,
            }],
        }),
    };

    let err = Transaction::try_from(ui_tx).expect_err("should fail due to short signature");
    match err {
        TransactionParseError::InvalidFormat(msg) => {
            assert!(
                msg.to_lowercase().contains("signature"),
                "unexpected message: {msg}"
            );
        }
        other => panic!("expected InvalidFormat error, got {other:?}"),
    }
}
