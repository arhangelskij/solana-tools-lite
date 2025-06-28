mod tests {
    use serde_json;
    use solana_tools_lite::crypto::ed25519;
    use solana_tools_lite::models::transaction::{Instruction, Message, Transaction};

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
                .contains(&"11111111111111111111111111111111".to_string())
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

        // Encode to base58 and save to tx
        tx.signatures[0] = bs58::encode(signature.to_bytes()).into_string();

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
    fn test_signature_invalid_on_message_tamper() {
        // TODO: Change message after sign, verify == false
        // assert!(!...)
    }

    #[test]
    fn test_signature_invalid_on_key_tamper() {
        // TODO: Use another pubkey for verify, must fail
        // assert!(!...)
    }

    #[test]
    fn test_invalid_signature_handling() {
        // TODO: Try verifying a random or empty signature
        // assert!(!...)
    }

    #[test]
    fn test_end_to_end_sign_and_save() {
        // TODO: Parse, sign, save new json, reload, verify
        // assert!(...)
    }

    #[test]
    fn test_fail_on_invalid_json() {
        // TODO: Broken json string should return error, not panic
        // assert!(serde_json::from_str::<Transaction>(bad_json).is_err());
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
}
