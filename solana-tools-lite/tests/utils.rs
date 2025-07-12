use bs58;
use rand::RngCore;
use rand::rng;

/// Generates a valid 32-byte Base58 public key
pub fn generate_mock_pubkey() -> String {
    let mut bytes = [0u8; 32];
    let mut rng = rng();
    rng.fill_bytes(&mut bytes);
    bs58::encode(bytes).into_string()
}

/// Generates a valid 64-byte Base58 signature
pub fn generate_mock_signature() -> String {
    //TODO:check it
    let mut bytes = [0u8; 64];
    let mut rng = rng();
    rng.fill_bytes(&mut bytes);
    bs58::encode(bytes).into_string()
}

use solana_tools_lite::models::input_transaction::{
    InputTransaction, UiCompiledInstruction, UiRawMessage, UiTransaction,
};
use solana_tools_lite::models::transaction::MessageHeader;

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
        data: instruction_data.to_string(), //TODO: check
    }];

    let message = UiRawMessage {
        header,
        account_keys: account_keys.into_iter().map(String::from).collect(),
        recent_blockhash: blockhash.to_string(),
        instructions,
    };
//TODO: check it if correct setup when need empty signs
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
