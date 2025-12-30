use solana_tools_lite::handlers::sign_tx::sign_transaction_by_key;
use solana_tools_lite::models::pubkey_base58::PubkeyBase58;

#[test]
#[ignore]
fn benchmark_signing_realistic_transactions_bin() {
    use solana_tools_lite::{
        crypto::ed25519,
        models::{
            instruction::Instruction,
            message::{Message, MessageHeader, MessageLegacy},
            transaction::Transaction,
        },
    };
    use std::time::Instant;

    let seed = [1u8; 32];
    let keypair = ed25519::keypair_from_seed(&seed).unwrap();

    const N: usize = 1_000;
    let start = Instant::now();

    use ed25519_dalek::Signature;
    use solana_tools_lite::models::hash_base58::HashBase58;

    for _ in 0..N {
        let tx = Transaction {
            signatures: vec![Signature::from_bytes(&[0u8; 64])],
            message: Message::Legacy(MessageLegacy {
                header: MessageHeader {
                    num_required_signatures: 1,
                    num_readonly_signed_accounts: 0,
                    num_readonly_unsigned_accounts: 1,
                },
                account_keys: vec![
                    PubkeyBase58([
                        138, 136, 227, 221, 116, 9, 241, 149, 253, 82, 219, 45, 60, 186, 93, 114,
                        202, 103, 9, 191, 29, 148, 18, 27, 243, 116, 136, 1, 180, 15, 111, 92,
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
        };

        let mut tx = tx;
        sign_transaction_by_key(&mut tx, &keypair).unwrap();
    }

    let elapsed = start.elapsed();
    println!(
        "Real tx signing: {} txs in {:.2?} ({:.2} µs per tx)",
        N,
        elapsed,
        elapsed.as_micros() as f64 / N as f64
    );
}

#[test]
#[ignore]
fn benchmark_e2e_parallel_build_and_sign() {
    use ed25519_dalek::Signature;
    use rayon::prelude::*;
    use solana_tools_lite::{
        crypto::ed25519,
        models::hash_base58::HashBase58,
        models::instruction::Instruction,
        models::message::{Message, MessageHeader, MessageLegacy},
        models::transaction::Transaction,
    };
    use std::time::Instant;

    let seed = [1u8; 32];
    let keypair = ed25519::keypair_from_seed(&seed).unwrap();

    const N: usize = 1_000_000;
    let start = Instant::now();

    (0..N)
        .into_par_iter()
        .for_each(|_| {
            let mut tx = Transaction {
                signatures: vec![Signature::from_bytes(&[0u8; 64])],
                message: Message::Legacy(MessageLegacy {
                    header: MessageHeader {
                        num_required_signatures: 1,
                        num_readonly_signed_accounts: 0,
                        num_readonly_unsigned_accounts: 1,
                    },
                    account_keys: vec![
                        PubkeyBase58([
                            138, 136, 227, 221, 116, 9, 241, 149, 253, 82, 219, 45, 60, 186, 93,
                            114, 202, 103, 9, 191, 29, 148, 18, 27, 243, 116, 136, 1, 180, 15,
                            111, 92,
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
            };
            sign_transaction_by_key(&mut tx, &keypair).unwrap();
        });

    let elapsed = start.elapsed();
    println!(
        "Parallel E2E tx build + sign: {} txs in {:.2?} ({:.2} µs per tx)",
        N,
        elapsed,
        elapsed.as_micros() as f64 / N as f64
    );
}
