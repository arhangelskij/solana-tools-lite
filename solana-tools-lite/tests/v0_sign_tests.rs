use bs58;
use ed25519_dalek::SigningKey;
use solana_tools_lite::codec::serialize_message;
use solana_tools_lite::crypto::ed25519;
use solana_tools_lite::handlers::sign_tx::sign_transaction_by_key;
use solana_tools_lite::models::input_transaction::{
    InputTransaction, UiAddressTableLookup, UiCompiledInstruction, UiRawMessage, UiRawMessageV0,
    UiTransaction,
};
use solana_tools_lite::models::message::MessageHeader;
use solana_tools_lite::models::transaction::Transaction;

/// Sign a minimal v0 transaction (no lookups) and verify signature.
#[test]
fn sign_v0_transaction_single_signer() {
    // signer keypair
    let seed = [1u8; 32];
    let signing_key = SigningKey::from_bytes(&seed);
    let verifying_key = signing_key.verifying_key();
    let signer_pk = bs58::encode(verifying_key.to_bytes()).into_string();

    // v0 message with static accounts only (no lookups)
    let header = MessageHeader {
        num_required_signatures: 1,
        num_readonly_signed_accounts: 0,
        num_readonly_unsigned_accounts: 1,
    };
    let program_id = "11111111111111111111111111111111"; // system program placeholder
    let blockhash = bs58::encode([9u8; 32]).into_string();

    let ui_tx = UiTransaction {
        signatures: vec![bs58::encode([0u8; 64]).into_string()],
        message: UiRawMessage::V0(UiRawMessageV0 {
            header,
            account_keys: vec![signer_pk.clone(), program_id.to_string()],
            recent_blockhash: blockhash,
            instructions: vec![UiCompiledInstruction {
                program_id_index: 1,
                accounts: vec![0],
                data: bs58::encode(&[0xaa, 0xbb]).into_string(),
            }],
            address_table_lookups: Vec::<UiAddressTableLookup>::new(),
        }),
    };

    let mut tx: Transaction = InputTransaction::Json(ui_tx).try_into().expect("v0 parse");
    sign_transaction_by_key(&mut tx, &signing_key).expect("sign v0");

    assert_eq!(tx.signatures.len(), 1);
    let sig = tx.signatures[0];
    let msg_bytes = serialize_message(&tx.message);
    assert!(
        ed25519::verify_signature(&verifying_key, &msg_bytes, &sig),
        "signature must verify for v0 message"
    );
}
