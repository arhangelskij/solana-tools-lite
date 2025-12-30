use criterion::{Criterion, criterion_group, criterion_main};
use ed25519_dalek::Signature;
use solana_tools_lite::handlers::sign_tx::sign_transaction_by_key;
use solana_tools_lite::models::hash_base58::HashBase58;
use solana_tools_lite::models::pubkey_base58::PubkeyBase58;
use solana_tools_lite::models::{
    instruction::Instruction,
    message::{Message, MessageHeader, MessageLegacy},
    transaction::Transaction,
};
use std::hint::black_box;

use solana_tools_lite::crypto::ed25519;

fn dummy_tx() -> Transaction {
    Transaction {
        signatures: vec![Signature::from_bytes(&[0u8; 64])],
        message: Message::Legacy(MessageLegacy {
            header: MessageHeader {
                num_required_signatures: 1,
                num_readonly_signed_accounts: 0,
                num_readonly_unsigned_accounts: 1,
            },
            account_keys: vec![
                PubkeyBase58([
                    138, 136, 227, 221, 116, 9, 241, 149, 253, 82, 219, 45, 60, 186, 93, 114, 202,
                    103, 9, 191, 29, 148, 18, 27, 243, 116, 136, 1, 180, 15, 111, 92,
                ]),
                PubkeyBase58([2u8; 32]),
                PubkeyBase58([3u8; 32]),
            ],
            recent_blockhash: HashBase58([9u8; 32]),
            instructions: vec![Instruction {
                program_id_index: 2,
                accounts: vec![0, 1],
                data: vec![1, 2, 3],
            }],
        }),
    }
}

fn bench_sign_tx(c: &mut Criterion) {
    let seed = [1u8; 32];
    let keypair = ed25519::keypair_from_seed(&seed).unwrap();

    c.bench_function("sign_transaction_dummy", |b| {
        b.iter(|| {
            let mut tx_clone = dummy_tx();
            let _ = sign_transaction_by_key(black_box(&mut tx_clone), black_box(&keypair));
        })
    });
}

criterion_group!(benches, bench_sign_tx);
criterion_main!(benches);
