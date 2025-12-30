#![allow(dead_code)]
use bs58;
use ed25519_dalek::Signature;
use rand::RngCore;
use solana_tools_lite::models::pubkey_base58::PubkeyBase58;

/// Generates a valid 32-byte Base58 public key (randomized).
pub fn generate_mock_pubkey() -> String {
    let mut bytes = [0u8; 32];
    let mut rng = rand::thread_rng();
    rng.fill_bytes(&mut bytes);
    bs58::encode(bytes).into_string()
}

/// Generates a valid 64-byte Base58 signature (randomized).
pub fn generate_mock_signature() -> String {
    let mut bytes = [0u8; 64];
    let mut rng = rand::thread_rng();
    rng.fill_bytes(&mut bytes);
    bs58::encode(bytes).into_string()
}

pub fn generate_fake_pubkey() -> PubkeyBase58 {
    let bytes = [42u8; 32];
    let encoded = bs58::encode(bytes).into_string();
    PubkeyBase58::try_from(encoded.as_str()).unwrap()
}

/// Deterministic helper: encode `len` bytes filled with `byte` into Base58.
pub fn deterministic_base58(byte: u8, len: usize) -> String {
    let data = vec![byte; len];
    bs58::encode(data).into_string()
}

/// Deterministic helper: create a PubkeyBase58 backed by repeated `byte`.
pub fn deterministic_pubkey(byte: u8) -> PubkeyBase58 {
    let encoded = deterministic_base58(byte, 32);
    PubkeyBase58::try_from(encoded.as_str()).unwrap()
}

/// Deterministic helper: create a fixed Ed25519 signature with repeated `byte`.
pub fn deterministic_signature(byte: u8) -> Signature {
    Signature::from_bytes(&[byte; 64])
}

use solana_tools_lite::models::input_transaction::{
    InputTransaction, UiCompiledInstruction, UiRawMessage, UiRawMessageLegacy, UiTransaction,
};
use solana_tools_lite::models::message::MessageHeader;

/// Build UiTransaction directly (for adapter/serde tests)
pub fn generate_ui_transaction(
    required_signatures: u8,
    account_keys: Vec<&str>,
    blockhash: &str,
    program_id_index: u8,
    instruction_accounts: Vec<u8>,
    instruction_data: &str,
) -> UiTransaction {
    let header = MessageHeader {
        num_required_signatures: required_signatures,
        num_readonly_signed_accounts: 0,
        num_readonly_unsigned_accounts: 1,
    };

    let instructions = vec![UiCompiledInstruction {
        program_id_index,
        accounts: instruction_accounts,
        data: instruction_data.to_string(),
    }];

    let message = UiRawMessage::Legacy(UiRawMessageLegacy {
        header,
        account_keys: account_keys.into_iter().map(String::from).collect(),
        recent_blockhash: blockhash.to_string(),
        instructions,
    });

    // For UI TX helper, mimic the same signature policy as generate_input_transaction
    let signatures = if required_signatures == 0 {
        vec![]
    } else {
        (0..(required_signatures - 1))
            .map(|_| generate_mock_signature())
            .collect()
    };

    UiTransaction {
        signatures,
        message,
    }
}

pub fn generate_input_transaction(
    required_signatures: u8,
    account_keys: Vec<&str>,
    blockhash: &str,
    program_id_index: u8,
    instruction_accounts: Vec<u8>,
    instruction_data: &str,
) -> InputTransaction {
    let header = MessageHeader {
        num_required_signatures: required_signatures,
        num_readonly_signed_accounts: 0,
        num_readonly_unsigned_accounts: 1,
    };

    let instructions = vec![UiCompiledInstruction {
        program_id_index,
        accounts: instruction_accounts,
        data: instruction_data.to_string(),
    }];

    let message = UiRawMessage::Legacy(UiRawMessageLegacy {
        header,
        account_keys: account_keys.into_iter().map(String::from).collect(),
        recent_blockhash: blockhash.to_string(),
        instructions,
    });

    // Empty signatures when none required, else generate dummy placeholders.
    let signatures = if required_signatures == 0 {
        vec![]
    } else {
        (0..(required_signatures - 1))
            .map(|_| generate_mock_signature())
            .collect()
    };

    let ui_tx = UiTransaction {
        signatures,
        message,
    };

    InputTransaction::Json(ui_tx)
}
