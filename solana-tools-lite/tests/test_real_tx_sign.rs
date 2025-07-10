use solana_tools_lite::models::hash_base58::HashBase58;
use solana_tools_lite::{
    crypto::ed25519,
    handlers::sign_tx::sign_transaction,
    models::pubkey_base58::PubkeyBase58,
    models::transaction::{Instruction, Message, MessageHeader, Transaction}
};

#[test]
fn test_real_tx_signature_base58() {
    let seed = [1u8; 32];
    let keypair = ed25519::keypair_from_seed(&seed).unwrap();

    use solana_tools_lite::utils::serialize;

    let recent_blockhash = HashBase58(
        bs58::decode("cGfHiC6Kgg3FpFZvgwGcswsCRtp4aBP2fzuXRQPizuN")
            .into_vec()
            .expect("invalid b58")
            .try_into()
            .unwrap(),
    );

    let raw: [u8; 32] = recent_blockhash.0;
    println!("🟢---------- Serialize blockhash: {:?}", raw);

    let msg = Message {
        header: MessageHeader {
            num_required_signatures: 1,
            num_readonly_signed_accounts: 0,
            num_readonly_unsigned_accounts: 1,
        },
        account_keys: vec![
            PubkeyBase58([1u8; 32]),
            PubkeyBase58([2u8; 32]),
            PubkeyBase58([3u8; 32]),
        ],
        recent_blockhash: recent_blockhash,
        instructions: vec![Instruction {
            program_id_index: 2,
            accounts: vec![0, 1],
            data: vec![1, 2, 3],
        }],
    };

    let bytes = serialize(&msg);
    println!("--------------- Msg to bytes: {:?}", bytes);

    let mut tx = Transaction {
        signatures: vec![],
        message: msg,
    };

    sign_transaction(&mut tx, &keypair).unwrap();

    let sig_bytes = bs58::encode(tx.signatures[0].to_bytes()).into_string();
    println!("Signature (base58): {}", sig_bytes);

    use solana_tools_lite::utils;

    let test = TestStruct {
        a: 1,
        b: 2,
        c: "3".to_string(),
    };
    let bytes = utils::serialize(&test);

    println!("🔴---------------- Bytes: {:?}", bytes);
}

use serde::Serialize;
#[derive(Serialize)]
struct TestStruct {
    a: u32,
    b: u64,
    c: String,
}
