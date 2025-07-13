mod tests {
    use rayon::result;
    use serde_json;
    use solana_tools_lite::crypto::ed25519;
    use solana_tools_lite::models::{
        input_transaction::{InputTransaction, UiTransaction},
        transaction::Transaction,
    };
    use solana_tools_lite::utils;

    use solana_tools_lite::handlers::sign_tx::sign_transaction_by_key;

    use crate::utils::*;

    #[test]
    fn test_transaction_json_roundtrip() {
        let pk1 = generate_mock_pubkey();
        let pk2 = generate_mock_pubkey();
        let blockhash = generate_mock_pubkey();
        let data = bs58::encode(b"mockdata").into_string();

        let input_tx: InputTransaction = generate_input_transaction(
            1,
            vec![&pk1, &pk2, "11111111111111111111111111111111"],
            &blockhash,
            2,
            vec![0, 1],
            &data,
        );

        //input_tx = {:?}", input_tx);

        let tx: Transaction = Transaction::try_from(input_tx).expect("parse");

        let new_json = serde_json::to_string_pretty(&tx).expect("serialize");

        //println!("-------------- {}", new_json);
        let tx2: Transaction = serde_json::from_str(&new_json).expect("parse 2");

        assert_eq!(tx.message.account_keys, tx2.message.account_keys);
        assert_eq!(tx.signatures, tx2.signatures);
    }

    #[test]
    fn test_insert_signature_in_unsigned_transaction() {
        // JSON of an unsigned transaction
        let tx_json = r#"
    {
        "signatures": [""],
        "message": {
            "account_keys": [
                "SenderPubKeyBase58Here",
                "RecipientPubKeyBase58Here",
                "11111111111111111111111111111111"
            ],
            "recent_blockhash": "SomeRecentBlockhashBase58",
            "instructions": [
                {
                    "program_id_index": 2,
                    "accounts": [0, 1],
                    "data": "3Bxs4R9sW4B"
                }
            ]
        }
    }
    "#;

        // Parse transaction
        let mut tx: Transaction = serde_json::from_str(tx_json).unwrap();

        // Create a dummy signature (64 bytes filled with 9)
        let dummy_bytes = [9u8; 64];
        let signature = ed25519_dalek::Signature::from_bytes(&dummy_bytes);

        // Insert the signature into the transaction
        tx.signatures[0] = signature;

        // Roundtrip: serialize and deserialize to check consistency
        let serialized = serde_json::to_string(&tx).unwrap();
        let parsed: Transaction = serde_json::from_str(&serialized).unwrap();

        // Verify inserted signature matches original dummy signature
        let parsed_sig_bytes = parsed.signatures[0].to_bytes();
        assert_eq!(parsed_sig_bytes, dummy_bytes);
    }

    #[test]
    // fn test_sign_and_verify_transaction_message() {
    //     let tx_json = r#"
    //     {
    //         "signatures": [""],
    //         "message": {
    //             "account_keys": [
    //                 "SenderPubKeyBase58Here",
    //                 "RecipientPubKeyBase58Here",
    //                 "11111111111111111111111111111111"
    //             ],
    //             "recent_blockhash": "SomeRecentBlockhashBase58",
    //             "instructions": [
    //                 {
    //                     "program_id_index": 2,
    //                     "accounts": [0, 1],
    //                     "data": "3Bxs4R9sW4B"
    //                 }
    //             ]
    //         }
    //     }
    //     "#;

    //     let mut tx: Transaction = serde_json::from_str(tx_json).unwrap();

    //     // Use a fixed 32-byte test seed
    //     let test_seed = [1u8; 32];
    //     let signing_key = ed25519::keypair_from_seed(&test_seed).expect("Create keypair");
    //     let verifying_key = signing_key.verifying_key();

    //     let message_bytes = serde_json::to_vec(&tx.message).unwrap();

    //     // Sign using wrapper
    //     let signature = ed25519::sign_message(&signing_key, &message_bytes);
    //     tx.signatures[0] = signature;

    //     // For verification: decode back
    //     let signature_decoded = bs58::decode(&tx.signatures[0].to_bytes())
    //         .into_vec()
    //         .expect("decode b58");
    //     let signature = ed25519::signature_from_bytes(&signature_decoded.try_into().unwrap());

    //     // Verify using wrapper
    //     let is_valid = ed25519::verify_signature(&verifying_key, &message_bytes, &signature);
    //     assert!(is_valid);
    //     //TODO: 🟡 check
    //     // Tampered negative case
    //     let mut tampered = message_bytes.clone();
    //     tampered[0] ^= 0xFF;
    //     let is_invalid = !ed25519::verify_signature(&verifying_key, &tampered, &signature);
    //     assert!(is_invalid);
    // }
    #[test]
    fn test_sign_and_verify_valid_tx() {
        // TODO: Parse valid json, sign message, verify signature == true
        // assert!(...)
    }
    //TODO: 🔴 test
    #[test]
    fn test_transaction_with_zero_required_signatures_has_empty_signatures() {
        let pk1 = generate_mock_pubkey();
        let pk2 = generate_mock_pubkey();
        let blockhash = generate_mock_pubkey();
        let data = bs58::encode(b"mockdata").into_string();

        let input_tx: InputTransaction = generate_input_transaction(
            0, // num_required_signatures == 0
            vec![&pk1, &pk2, "11111111111111111111111111111111"],
            &blockhash,
            2,
            vec![0, 1],
            &data,
        );

        // Try parsing transaction with empty signatures array
        let mut tx: Transaction = Transaction::try_from(input_tx).expect("parse");
        // Ensure that when num_required_signatures == 0, the Transaction has an empty signature list
        assert_eq!(tx.signatures.len(), 0);
        //////////

        let seed = [1u8; 32];
        let keypair = ed25519::keypair_from_seed(&seed).unwrap();

        let _ = sign_transaction_by_key(&mut tx, &keypair);

        println!("------- {:?}", tx);

        // Optionally, try adding a signature (simulate signing)
        // Should not panic if you push or insert to signatures
        // (this may depend on your business logic: sometimes required signers are known in advance)
        // For example:
        // tx.signatures.push("fake_signature_here".to_string());
    }

    ///////////////////////////// New tests
    ///
    #[test]
    fn test_fail_on_invalid_json_parse() {
        // Broken JSON (missing comma)
        let bad_json = r#"{"signatures": [""], "message": { "account_keys": ["a"], "recent_blockhash": "h" "instructions": [] } }"#;
        let err = serde_json::from_str::<Transaction>(bad_json);
        assert!(err.is_err(), "Should fail to parse broken JSON");
    }

    // 🟢 main test
    #[test]
    fn test_end_to_end_sign_and_save() {
        // Full roundtrip: parse, sign, save, reload, verify

        // Step 1: generate keypair
        let seed = [42u8; 32];
        let keypair = ed25519::keypair_from_seed(&seed).unwrap();
        let verifying_key = keypair.verifying_key();

        // Step 2: generate pk and use fixed program_id
        let pk = bs58::encode(verifying_key.to_bytes()).into_string();
        let program_id = "11111111111111111111111111111111";
        let blockhash = generate_mock_pubkey();
        let data = bs58::encode(b"mockdata").into_string();

        // Step 3: build transaction
        let input_tx: InputTransaction =
            generate_input_transaction(1, vec![&pk, program_id], &blockhash, 2, vec![0, 1], &data);

        let mut tx = Transaction::try_from(input_tx).unwrap();

        let result = sign_transaction_by_key(&mut tx, &keypair);
        assert!(result.is_ok());

        let ui_tx = UiTransaction::from(&tx);

        println!("Signatures in UiTransaction:");
        for sig in &ui_tx.signatures {
            println!("{}", sig);
        }

        let roundtrip_input = InputTransaction::Json(ui_tx);
        let saved_json = serde_json::to_string(&roundtrip_input).unwrap();

        // 🔁 Deserialize from same format
        let deserialized: InputTransaction = serde_json::from_str(&saved_json).unwrap();
        let tx2 = Transaction::try_from(deserialized).unwrap();

        // Reload and verify signature
        let sig_bytes = &tx2.signatures[0].to_bytes();
        let signature = ed25519::signature_from_bytes(&sig_bytes);
        let msg_bytes = utils::serialize(&tx2.message).unwrap();

        let is_valid = ed25519::verify_signature(&verifying_key, &msg_bytes, &signature);

        assert!(is_valid);
    }

    // ‼️
    #[test]
    fn test_signature_invalid_on_message_tamper() {
        // After signing, tamper with message, signature must fail
        let tx_json = r#"
        {
            "signatures": [""],
            "message": {
                "account_keys": [
                    "SenderPubKeyBase58Here",
                    "RecipientPubKeyBase58Here",
                    "11111111111111111111111111111111"
                ],
                "recent_blockhash": "SomeRecentBlockhashBase58",
                "instructions": [
                    {
                        "program_id_index": 2,
                        "accounts": [0, 1],
                        "data": "3Bxs4R9sW4B"
                    }
                ]
            }
        }
        "#;

        let mut tx: Transaction = serde_json::from_str(tx_json).unwrap();
        let test_seed = [1u8; 32];
        let signing_key = ed25519::keypair_from_seed(&test_seed).unwrap();
        let verifying_key = signing_key.verifying_key();

        // let message_bytes = serde_json::to_vec(&tx.message).unwrap();
        // let signature = ed25519::sign_message(&signing_key, &message_bytes);
        // tx.signatures[0] = bs58::encode(signature.to_bytes()).into_string();
        sign_transaction_by_key(&mut tx, &signing_key);

        // Tamper with the message
        let mut tampered_msg = tx.message;
        tampered_msg.account_keys[0] = "TamperedKey".try_into().unwrap();

        let tampered_bytes = serde_json::to_vec(&tampered_msg).unwrap();
        let sig_decoded = bs58::decode(&tx.signatures[0].to_bytes())
            .into_vec()
            .unwrap();
        let signature = ed25519::signature_from_bytes(&sig_decoded.try_into().unwrap());

        let is_valid = ed25519::verify_signature(&verifying_key, &tampered_bytes, &signature);
        assert!(!is_valid);
    }

    #[test]
    fn test_signature_invalid_on_key_tamper() {
        // Verifying with another key must fail
        let tx_json = r#"
        {
            "signatures": [""],
            "message": {
                "account_keys": [
                    "SenderPubKeyBase58Here",
                    "RecipientPubKeyBase58Here",
                    "11111111111111111111111111111111"
                ],
                "recent_blockhash": "SomeRecentBlockhashBase58",
                "instructions": [
                    {
                        "program_id_index": 2,
                        "accounts": [0, 1],
                        "data": "3Bxs4R9sW4B"
                    }
                ]
            }
        }
        "#;

        let mut tx: Transaction = serde_json::from_str(tx_json).unwrap();
        let test_seed = [1u8; 32];
        let signing_key = ed25519::keypair_from_seed(&test_seed).unwrap();

        let result = sign_transaction_by_key(&mut tx, &signing_key); //TODO: 🔴
        assert!(result.is_ok());

        let message_bytes = serde_json::to_vec(&tx.message).unwrap();
        // let signature = ed25519::sign_message(&signing_key, &message_bytes);
        // tx.signatures[0] = bs58::encode(signature.to_bytes()).into_string();

        // Use another keypair for verification
        let other_seed = [2u8; 32];
        let other_signing_key = ed25519::keypair_from_seed(&other_seed).unwrap();
        let other_verifying_key = other_signing_key.verifying_key();

        let sig_decoded = bs58::decode(&tx.signatures[0].to_bytes())
            .into_vec()
            .unwrap();
        let signature = ed25519::signature_from_bytes(&sig_decoded.try_into().unwrap());

        let is_valid = ed25519::verify_signature(&other_verifying_key, &message_bytes, &signature);
        assert!(!is_valid);
    }

    #[test]
    fn test_invalid_signature_handling() {
        // Pass random bytes or empty sig, must fail
        let tx_json = r#"
        {
            "signatures": ["C4hqXg2jWsasQZ43VUBCqTYPE1fVVJQ5C3g2PF2UJjcuseFufzLTqzbE22DTPBYtocQTACLav3mZT86KKrMzqEM"],
            "message": {
                "account_keys": [
                    "SenderPubKeyBase58Here",
                    "RecipientPubKeyBase58Here",
                    "11111111111111111111111111111111"
                ],
                "recent_blockhash": "SomeRecentBlockhashBase58",
                "instructions": [
                    {
                        "program_id_index": 2,
                        "accounts": [0, 1],
                        "data": "3Bxs4R9sW4B"
                    }
                ]
            }
        }
        "#;

        let tx: Transaction = serde_json::from_str(tx_json).unwrap();
        let bad_sig = vec![0u8; 64];
        let sig_bytes: [u8; 64] = bad_sig.try_into().unwrap();
        let fake_signature = ed25519::signature_from_bytes(&sig_bytes);

        let test_seed = [1u8; 32];
        let signing_key = ed25519::keypair_from_seed(&test_seed).unwrap();
        let verifying_key = signing_key.verifying_key();
        let message_bytes = serde_json::to_vec(&tx.message).unwrap();

        // Invalid signature must fail verify
        let is_valid = ed25519::verify_signature(&verifying_key, &message_bytes, &fake_signature);
        assert!(!is_valid);
    }
}

////////////////////////////////////////////////
///
///

#[path = "utils.rs"]
mod utils;

mod tests_signing {
    use crate::utils::*;

    use solana_tools_lite::models::transaction::Transaction;
    use solana_tools_lite::utils::serialize;

    use solana_tools_lite::crypto::ed25519;
    use solana_tools_lite::handlers::sign_tx::sign_transaction_by_key;
    use std::convert::TryFrom;

    #[test]
    fn test_sign_transaction_valid_index() {
        // Step 1: generate keypair
        let seed = [42u8; 32];
        let keypair = ed25519::keypair_from_seed(&seed).unwrap();

        // Step 2: generate pk and use fixed program_id
        let pk = bs58::encode(keypair.verifying_key().to_bytes()).into_string();
        let program_id = "11111111111111111111111111111111";
        let blockhash = generate_mock_pubkey();
        let data = bs58::encode(b"mockdata").into_string();

        // Step 3: build transaction where pk2 is signer at index 1
        let input_tx =
            generate_input_transaction(1, vec![&pk, program_id], &blockhash, 2, vec![0, 1], &data);

        let mut tx = Transaction::try_from(input_tx).unwrap();

        // Step 4: sign tx
        let res = sign_transaction_by_key(&mut tx, &keypair);
        assert!(res.is_ok(), "Signing failed: {:?}", res.unwrap_err());

        // Signature at index 0 should be updated
        println!(
            "-----------------------------tx.signatures.len() {:?}",
            tx.signatures.len()
        );
        assert_eq!(tx.signatures.len(), 1);
        assert_ne!(tx.signatures[0].to_bytes(), [0u8; 64]);

        // Validate signature
        let message_bytes = serialize(&tx.message).unwrap();
        let verifying_key = keypair.verifying_key();
        let is_valid = ed25519::verify_signature(
            &verifying_key,
            &message_bytes,
            &ed25519::signature_from_bytes(&tx.signatures[0].to_bytes()),
        );
        assert!(is_valid);
    }

    #[test]
    fn test_multi_signature_correct_index() {
        // Step 1: generate keypair for pk2
        let seed = [42u8; 32];
        let keypair = ed25519::keypair_from_seed(&seed).unwrap();
        let pk2 = bs58::encode(keypair.verifying_key().to_bytes()).into_string();

        // Step 2: generate pk1 and use fixed program_id
        let pk1 = generate_mock_pubkey();
        let program_id = "11111111111111111111111111111111";
        let blockhash = generate_mock_pubkey();
        let data = bs58::encode(b"mockdata").into_string();

        // Step 3: build transaction where pk2 is signer at index 1
        let input_tx = generate_input_transaction(
            2,
            vec![&pk1, &pk2, program_id],
            &blockhash,
            2,
            vec![0, 1],
            &data,
        );

        let mut tx = Transaction::try_from(input_tx).unwrap();

        // Save original signature at index 0
        let original_sig_0 = tx.signatures[0].to_bytes();

        // Step 4: use real keypair for pk2 (index 1)
        let res = sign_transaction_by_key(&mut tx, &keypair);
        assert!(res.is_ok(), "Signing failed: {:?}", res.unwrap_err());

        // Signature at index 1 should be updated
        assert_eq!(tx.signatures.len(), 2);
        assert_ne!(tx.signatures[1].to_bytes(), [0u8; 64]);

        // Signature at index 0 must remain unchanged
        assert_eq!(tx.signatures[0].to_bytes(), original_sig_0);

        // Validate signature at index 1
        let message_bytes = serialize(&tx.message).unwrap();
        let verifying_key = keypair.verifying_key();
        let is_valid = ed25519::verify_signature(
            &verifying_key,
            &message_bytes,
            &ed25519::signature_from_bytes(&tx.signatures[1].to_bytes()),
        );
        assert!(is_valid);
    }

    /// This test checks that signing fails when provided key does not match any required signer
    #[test]
    fn test_sign_transaction_missing_key() {
        use solana_tools_lite::errors::{SignError, ToolError};

        // Step 1: generate keypair not part of transaction
        let fake_keypair = ed25519::keypair_from_seed(&[99u8; 32]).unwrap();

        // Step 2: generate pk1 and pk2 (required signers), program ID is fixed
        let pk1 = generate_mock_pubkey();
        let pk2 = generate_mock_pubkey();
        let program_id = "11111111111111111111111111111111";
        let blockhash = generate_mock_pubkey();
        let data = bs58::encode(b"mockdata").into_string();

        // Step 3: create input transaction with pk1 and pk2 as signers
        let input_tx = generate_input_transaction(
            2,
            vec![&pk1, &pk2, program_id],
            &blockhash,
            2,
            vec![0, 1],
            &data,
        );

        let mut tx = Transaction::try_from(input_tx).unwrap();

        // Step 4: try signing with a key not matching pk1 or pk2
        let result = sign_transaction_by_key(&mut tx, &fake_keypair);
        assert!(matches!(
            result,
            Err(ToolError::Sign(SignError::SignerKeyNotFound))
        ));
    }

    #[test]
    fn test_sign_transaction_out_of_bounds() {
        use solana_tools_lite::errors::{SignError, ToolError};

        // Step 1: Generate keypair that will be at index 2 (non-signer)
        let non_signer_key = ed25519::keypair_from_seed(&[3u8; 32]).unwrap();
        let pk0 = generate_mock_pubkey();
        let pk1 = generate_mock_pubkey();
        let pk2 = bs58::encode(non_signer_key.verifying_key().to_bytes()).into_string(); // this will be at index 2

        let program_id = "11111111111111111111111111111111";
        let blockhash = generate_mock_pubkey();
        let data = bs58::encode(b"mockdata").into_string();

        // Step 2: num_required_signatures = 2, so index 2 is not required
        let input_tx = generate_input_transaction(
            2,
            vec![&pk0, &pk1, &pk2, program_id],
            &blockhash,
            3,
            vec![0, 1],
            &data,
        );

        let mut tx = Transaction::try_from(input_tx).unwrap();

        // Step 3: Attempt to sign with a non-required signer (index 2)
        let result = sign_transaction_by_key(&mut tx, &non_signer_key);

        // Step 4: Assert that signing fails with correct error
        assert!(matches!(
            result,
            Err(ToolError::Sign(SignError::SigningNotRequiredForKey))
        ));
    }
}
