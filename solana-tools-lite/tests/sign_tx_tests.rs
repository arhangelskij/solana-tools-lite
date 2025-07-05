mod tests {
    use serde_json;
    use solana_tools_lite::crypto::ed25519;
    use solana_tools_lite::models::transaction::Transaction;

    #[test]
    fn test_parse_and_sign_transaction_json() {
        // Example minimal transaction JSON (could be moved to a separate testdata/tx.json file)
        // For production, load from disk or receive as CLI input.
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

        // Parse the transaction JSON into our Transaction struct
        let tx: Transaction = serde_json::from_str(tx_json).expect("Parse tx JSON");

        // At this point, you can access all fields for signing, validation, etc.
        assert_eq!(tx.message.account_keys.len(), 3);
        assert_eq!(tx.signatures.len(), 1);

        // (Signing logic would go here: extract message, serialize, sign, update signatures)
        // For now, just print out the message structure for debugging:
        println!("{:#?}", tx);

        // Example: check that the system program is present as expected
        assert!(
            tx.message
                .account_keys
                .contains(&"11111111111111111111111111111111".parse().unwrap())
        );
    }

    #[test]
    fn test_transaction_json_roundtrip() {
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

        let tx: Transaction = serde_json::from_str(tx_json).expect("parse");
        let new_json = serde_json::to_string_pretty(&tx).expect("serialize");
        let tx2: Transaction = serde_json::from_str(&new_json).expect("parse 2");
        assert_eq!(tx.message.account_keys, tx2.message.account_keys);
        assert_eq!(tx.signatures, tx2.signatures);
    }

    #[test]
    fn test_add_fake_signature() {
use ed25519_dalek::Signature;


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

        // "Sign": serialize message, hash and input in a fake sign
        let message_bytes = serde_json::to_vec(&tx.message).unwrap();
        // Fake! Here should be a real ed25519 sign
        let fake_signature = bs58::encode(&message_bytes[0..8]).into_string();
        tx.signatures[0] = fake_signature;

        assert!(!tx.signatures[0].is_empty());
    }

    #[test]
    fn test_sign_and_verify_transaction_message() {
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

        // Use a fixed 32-byte test seed
        let test_seed = [1u8; 32];
        let signing_key = ed25519::keypair_from_seed(&test_seed).expect("Create keypair");
        let verifying_key = signing_key.verifying_key();

        let message_bytes = serde_json::to_vec(&tx.message).unwrap();

        // Sign using wrapper
        let signature = ed25519::sign_message(&signing_key, &message_bytes);

        tx.signatures[0] = signature;

        // For verification: decode back
        let signature_decoded = bs58::decode(&tx.signatures[0])
            .into_vec()
            .expect("decode b58");
        let signature = ed25519::signature_from_bytes(&signature_decoded.try_into().unwrap());

        // Verify using wrapper
        let is_valid = ed25519::verify_signature(&verifying_key, &message_bytes, &signature);
        assert!(is_valid);

        // Tampered negative case
        let mut tampered = message_bytes.clone();
        tampered[0] ^= 0xFF;
        let is_invalid = !ed25519::verify_signature(&verifying_key, &tampered, &signature);
        assert!(is_invalid);
    }

    #[test]
    fn test_sign_and_verify_valid_tx() {
        // TODO: Parse valid json, sign message, verify signature == true
        // assert!(...)
    }

    #[test]
    fn test_cold_signer_empty_signatures_handling() {
        // This test checks that a transaction with an empty signatures array
        // (i.e., before any signing happened) is parsed and handled correctly.

        let tx_json = r#"
        {
            "signatures": [],
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

        // Try parsing transaction with empty signatures array
        let tx: Transaction = serde_json::from_str(tx_json).expect("parse");
        // In cold-signer mode, empty signatures are valid and expected
        assert_eq!(tx.signatures.len(), 0);

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

    // To chek getting of a custom error
    #[test]
    fn test_fail_on_serialize_unsupported_type() {
        use solana_tools_lite::{errors::ToolError, utils::serialize};
        struct BadSerialize;

        impl serde::Serialize for BadSerialize {
            fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                Err(serde::ser::Error::custom(
                    "Intentional serialization failure",
                ))
            }
        }

        #[derive(serde::Serialize)]
        struct Bad {
            field: BadSerialize,
        }

        let data = Bad {
            field: BadSerialize,
        };

        let result = serialize(&data);
        match result {
            Err(ToolError::Bincode(_)) => { /* ok */ }
            Err(e) => panic!("Wrong error type: {:?}", e),
            Ok(_) => panic!("Expected error, got Ok"),
        }
    }

    #[test]
    fn test_end_to_end_sign_and_save() {
        // Full roundtrip: parse, sign, save, reload, verify
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

        // Sign message
        let message_bytes = serde_json::to_vec(&tx.message).unwrap();
        let signature = ed25519::sign_message(&signing_key, &message_bytes);
        tx.signatures[0] = bs58::encode(signature.to_bytes()).into_string();

        // Serialize to json string (simulate saving to file)
        let saved = serde_json::to_string(&tx).unwrap();

        // Reload and verify signature
        let tx2: Transaction = serde_json::from_str(&saved).unwrap();
        let sig_decoded = bs58::decode(&tx2.signatures[0]).into_vec().unwrap();
        let signature = ed25519::signature_from_bytes(&sig_decoded.try_into().unwrap());
        let msg_bytes = serde_json::to_vec(&tx2.message).unwrap();
        let is_valid = ed25519::verify_signature(&verifying_key, &msg_bytes, &signature);
        assert!(is_valid);
    }

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

        let message_bytes = serde_json::to_vec(&tx.message).unwrap();
        let signature = ed25519::sign_message(&signing_key, &message_bytes);
        tx.signatures[0] = bs58::encode(signature.to_bytes()).into_string();

        // Tamper with the message
        let mut tampered_msg = tx.message; // перемещаем, не копируем
        tampered_msg.account_keys[0] = "TamperedKey".to_string();

        let tampered_bytes = serde_json::to_vec(&tampered_msg).unwrap();
        let sig_decoded = bs58::decode(&tx.signatures[0]).into_vec().unwrap();
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

        let message_bytes = serde_json::to_vec(&tx.message).unwrap();
        let signature = ed25519::sign_message(&signing_key, &message_bytes);
        tx.signatures[0] = bs58::encode(signature.to_bytes()).into_string();

        // Use another keypair for verification
        let other_seed = [2u8; 32];
        let other_signing_key = ed25519::keypair_from_seed(&other_seed).unwrap();
        let other_verifying_key = other_signing_key.verifying_key();

        let sig_decoded = bs58::decode(&tx.signatures[0]).into_vec().unwrap();
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
