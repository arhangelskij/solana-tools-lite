use solana_tools_lite::crypto::ed25519;
use solana_tools_lite::handlers::sign_tx::sign_transaction;
use solana_tools_lite::models::transaction::{Message, Transaction};
use std::time::Instant;

// #[test]
// fn benchmark_signing_performance() {
//     let tx_template = Transaction {
//         signatures: vec!["".to_string()],
//         message: Message {
//             account_keys: vec![
//                 "SenderPubKeyBase58Here".to_string(),
//                 "RecipientPubKeyBase58Here".to_string(),
//                 "11111111111111111111111111111111".to_string(),
//             ],
//             recent_blockhash: "SomeRecentBlockhashBase58".to_string(),
//             instructions: vec![/* fill with dummy Instruction */],
//         },
//     };

//     let seed = [1u8; 32];
//     let keypair = ed25519::keypair_from_seed(&seed).unwrap();

//     const N: usize = 100_000;
//     let start = Instant::now();

//     for _ in 0..N {
//         let msg_bytes = serde_json::to_vec(&tx_template.message).unwrap();
//         let _sig = ed25519::sign_message(&keypair, &msg_bytes);
//     }

//     let elapsed = start.elapsed();
//     println!("Signed {} txs in {:.2?} ({:.2} µs per tx)",
//         N,
//         elapsed,
//         elapsed.as_micros() as f64 / N as f64);
// }
///////////////////////////////////////////////////
 #[test]
fn benchmark_signing_realistic_transactions() {
    use solana_tools_lite::{
        crypto::ed25519,
        handlers::sign_tx::sign_transaction,
        models::transaction::{Instruction, Message, Transaction},
    };
    use std::time::Instant;

    let seed = [1u8; 32];
    let keypair = ed25519::keypair_from_seed(&seed).unwrap();

    const N: usize = 100_000;
    let start = Instant::now();

    for _ in 0..N {
        let tx = Transaction {
            signatures: vec!["".to_string()],
            message: Message {
                account_keys: vec![
                    "SenderPubKeyBase58Here".to_string(),
                    "RecipientPubKeyBase58Here".to_string(),
                    "11111111111111111111111111111111".to_string(),
                ],
                recent_blockhash: "SomeRecentBlockhashBase58".to_string(),
                instructions: vec![
                    Instruction {
                        program_id_index: 2,
                        accounts: vec![0, 1],
                        data: "test".to_string()//vec![1, 2, 3], //TODO: uncomment after
                    }
                ],
            },
        };

        let mut tx = tx;
        sign_transaction(&mut tx, &keypair).unwrap();
    }

    let elapsed = start.elapsed();
    println!(
        "Real tx signing: {} txs in {:.2?} ({:.2} µs per tx)",
        N,
        elapsed,
        elapsed.as_micros() as f64 / N as f64
    );
}
/////////////////////////////////////////////////////////////////////////////////////////////
// #[test]
// fn benchmark_signing_parallel_transactions() {
//     use rayon::prelude::*;
//     use solana_tools_lite::{
//         crypto::ed25519,
//         handlers::sign_tx::sign_transaction,
//         models::transaction::{Instruction, Message, Transaction},
//     };
//     use std::time::Instant;

//     let seed = [1u8; 32];
//     let keypair = ed25519::keypair_from_seed(&seed).unwrap();

//     const N: usize = 10_000_000;
//     let txs: Vec<_> = (0..N)
//         .map(|_| Transaction {
//             signatures: vec!["".to_string()],
//             message: Message {
//                 account_keys: vec![
//                     "SenderPubKeyBase58Here".to_string(),
//                     "RecipientPubKeyBase58Here".to_string(),
//                     "11111111111111111111111111111111".to_string(),
//                 ],
//                 recent_blockhash: "SomeRecentBlockhashBase58".to_string(),
//                 instructions: vec![Instruction {
//                     program_id_index: 2,
//                     accounts: vec![0, 1],
//                     data: "test".to_string(),
//                 }],
//             },
//         })
//         .collect();

//     let start = Instant::now();

//     let _results: Vec<_> = txs
//         .into_par_iter()
//         .map(|mut tx| sign_transaction(&mut tx, &keypair).unwrap())
//         .collect();

//     let elapsed = start.elapsed();
//     println!(
//         "Parallel tx signing: {} txs in {:.2?} ({:.2} µs per tx)",
//         N,
//         elapsed,
//         elapsed.as_micros() as f64 / N as f64
//     );
// }