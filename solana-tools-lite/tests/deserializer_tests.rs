#[cfg(test)]
mod deserialize_tests {
    use ed25519_dalek::Signature;
    use solana_tools_lite::deserializer::*;
    use std::convert::TryFrom;
    use data_encoding::BASE64;
    use solana_tools_lite::models::input_transaction::UiTransaction;
    use solana_tools_lite::handlers::sign_tx::sign_transaction_by_key;

    use solana_tools_lite::crypto::ed25519;

    // -------------------------------
    // Section: shortvec decoding – basic cases
    // -------------------------------

    // Basic case: decode single-byte shortvec (<128)
    #[test]
    fn test_read_shortvec_len_small() {
        let data = [5u8];
        let (value, offset) = read_shortvec_len(&data).unwrap();
        assert_eq!(value, 5);
        assert_eq!(offset, 1);
    }

    // Two-byte shortvec decode: 128 encoded as [0x80,0x01]
    #[test]
    fn test_read_shortvec_len_two_bytes_128() {
        // short-vec two-byte encoding for 128 is 0x80 0x01
        let data = [0x80u8, 0x01];
        let (value, offset) = read_shortvec_len(&data).unwrap();
        assert_eq!(value, 128);
        assert_eq!(offset, 2);
    }

    // -------------------------------
    // Section: ed25519 signature parsing
    // -------------------------------

    // Parse a valid 64-byte ed25519 signature
    #[test]
    fn test_signature_from_bytes() {
        let bytes = [1u8; 64];
        let sig = Signature::try_from(&bytes[..]).expect("valid signature bytes must parse");
        assert_eq!(sig.to_bytes(), bytes);
    }

    // Error on invalid signature length (not 64 bytes)
    #[test]
    fn test_signature_from_bytes_invalid_length() {
        let bytes = [1u8; 63];
        let result = Signature::try_from(bytes.as_slice());
        assert!(result.is_err());
    }

    // Zeroed signature bytes still produce a valid Signature struct
    #[test]
    fn test_empty_signature() {
        let bytes = [0u8; 64];
        let sig = Signature::try_from(&bytes[..])
            .expect("zeroed signature bytes should still parse as a struct");
        assert_eq!(sig.to_bytes(), bytes);
    }

    // -------------------------------
    // Section: deserialize_transaction – negative cases
    // -------------------------------

    // Error when transaction byte slice is too short for declared signature count
    #[test]
    fn test_insufficient_data() {
        let data = vec![1u8]; // 1 signature, но нет самих байтов
        let result = deserialize_transaction(&data);
        assert!(result.is_err());
    }

    // -------------------------------
    // Section: parse_instruction – basic cases
    // -------------------------------

    // Simple instruction parse: one account, fixed data
    #[test]
    fn test_parse_instruction_simple() {
        // program_id_index = 2, 1 account = [3], data = [0x01, 0x02]
        let data = vec![
            2, // program_id_index
            1, // accounts_len
            3, // account_index
            2, // data_len
            0x01, 0x02, // data
        ];

        let mut cursor = 0;
        let instruction = parse_instruction(&data, &mut cursor).unwrap();

        assert_eq!(instruction.program_id_index, 2);
        assert_eq!(instruction.accounts, vec![3]);
        assert_eq!(instruction.data, vec![0x01, 0x02]);
        assert_eq!(cursor, data.len());
    }

    // -------------------------------
    // Section: deserialize_message – basic cases
    // -------------------------------

    // Basic message deserialization: one account, no instruction data
    #[test]
    fn test_full_message_parsing() {
        // Простое сообщение: 1 аккаунт, 1 инструкция
        let mut data = vec![
            1, 0, 0, // header
            1, // 1 account
        ];

        // Добавляем pubkey (32 байта)
        data.extend_from_slice(&[1u8; 32]);

        // Добавляем blockhash (32 байта)
        data.extend_from_slice(&[2u8; 32]);

        // 1 инструкция
        data.extend_from_slice(&[
            1, // instructions_count (compact-u16)
            0, // program_id_index
            0, // accounts_len
            0, // data_len
        ]);

        let message = deserialize_message(&data).unwrap();
        assert_eq!(message.header.num_required_signatures, 1);
        assert_eq!(message.account_keys.len(), 1);
        assert_eq!(message.instructions.len(), 1);
    }

    // Edge-case shortvec decoding: boundary values 0,127,128,16383,16384
    #[test]
    fn test_shortvec_edge_cases() {
        // 0
        let (val, offset) = read_shortvec_len(&[0]).unwrap();
        assert_eq!(val, 0);
        assert_eq!(offset, 1);

        // 127 (max 1-byte)
        let (val, offset) = read_shortvec_len(&[127]).unwrap();
        assert_eq!(val, 127);
        assert_eq!(offset, 1);

        // 128 (first 2-byte) -> 0x80 0x01
        let (val, offset) = read_shortvec_len(&[0x80, 0x01]).unwrap();
        assert_eq!(val, 128);
        assert_eq!(offset, 2);

        // 16_383 (max 2-byte) -> 0xFF 0x7F
        let (val, offset) = read_shortvec_len(&[0xFF, 0x7F]).unwrap();
        assert_eq!(val, 16_383);
        assert_eq!(offset, 2);

        // 16_384 (first 3-byte) -> 0x80 0x80 0x01
        let (val, offset) = read_shortvec_len(&[0x80, 0x80, 0x01]).unwrap();
        assert_eq!(val, 16_384);
        assert_eq!(offset, 3);
    }

    // Error on truncated shortvec encoding (missing continuation byte)
    #[test]
    fn test_shortvec_not_enough_bytes() {
        let res = read_shortvec_len(&[0x80]);
        assert!(res.is_err(), "expected error for truncated shortvec length");
    }

    // Error on program_id_index out of bounds in message parsing
    #[test]
    fn test_message_program_index_oob() {
        // header: 1 required sig, 0/0 readonly
        let mut data = vec![1, 0, 0];
        // accounts_count = 1
        data.push(1);
        // 1 pubkey (32 bytes)
        data.extend_from_slice(&[0u8; 32]);
        // recent_blockhash (32 bytes)
        data.extend_from_slice(&[0u8; 32]);
        // instructions_count = 1
        data.push(1);
        // instruction:
        // program_id_index = 1 (out of bounds, since only 1 account -> valid indices: 0)
        data.push(1);
        // accounts_len = 0
        data.push(0);
        // data_len = 0
        data.push(0);

        let res = deserialize_message(&data);
        assert!(res.is_err(), "expected error due to program_id_index out of bounds");
    }

    // Error on account index out of bounds within instruction
    #[test]
    fn test_message_account_index_oob() {
        // header
        let mut data = vec![1, 0, 0];
        // accounts_count = 1
        data.push(1);
        // 1 pubkey
        data.extend_from_slice(&[0u8; 32]);
        // recent_blockhash
        data.extend_from_slice(&[0u8; 32]);
        // instructions_count = 1
        data.push(1);
        // instruction:
        // program_id_index = 0 (ok)
        data.push(0);
        // accounts_len = 1
        data.push(1);
        // accounts: index 5 (out of bounds)
        data.push(5);
        // data_len = 0
        data.push(0);

        let res = deserialize_message(&data);
        assert!(res.is_err(), "expected error due to account index out of bounds");
    }

    // -------------------------------
    // Section: shortvec extra negatives / limits
    // -------------------------------
    // Error on overly long shortvec encoding (>3 bytes)
    #[test]
    fn test_shortvec_too_long_encoding_err() {
        // 4-byte continuation should be rejected by solana-short-vec (u16 max, up to 3 bytes)
        let res = read_shortvec_len(&[0x80, 0x80, 0x80, 0x01]);
        assert!(res.is_err(), "expected error for length encoded with >3 bytes");
    }

    // -------------------------------
    // Section: Transaction / Message – positive and negative
    // -------------------------------
    // Deserialize minimal multisig transaction with two signatures
    #[test]
    fn test_deserialize_transaction_multisig_minimal() {
        // signatures_count = 2
        let mut data: Vec<u8> = vec![2];
        // 2 signatures (2 * 64 bytes)
        data.extend_from_slice(&[0u8; 64]);
        data.extend_from_slice(&[0u8; 64]);

        // Message
        // header: 2 required signatures, 0/0 readonly
        data.extend_from_slice(&[2, 0, 0]);
        // accounts_count = 2
        data.push(2);
        // 2 pubkeys (2 * 32)
        data.extend_from_slice(&[1u8; 32]);
        data.extend_from_slice(&[2u8; 32]);
        // recent_blockhash (32 bytes)
        data.extend_from_slice(&[3u8; 32]);
        // instructions_count = 0
        data.push(0);

        let tx = deserialize_transaction(&data).expect("must parse minimal multisig tx");
        assert_eq!(tx.signatures.len(), 2);
        assert_eq!(tx.message.header.num_required_signatures, 2);
        assert_eq!(tx.message.account_keys.len(), 2);
        assert!(tx.message.instructions.is_empty());
    }

    // Error on truncated recent_blockhash field
    #[test]
    fn test_message_truncated_blockhash() {
        // header
        let mut data = vec![1, 0, 0];
        // accounts_count = 1
        data.push(1);
        // 1 pubkey (32 bytes)
        data.extend_from_slice(&[0u8; 32]);
        // recent_blockhash – truncate intentionally: only 10 bytes
        data.extend_from_slice(&[0u8; 10]);

        let res = deserialize_message(&data);
        assert!(res.is_err(), "expected error due to truncated blockhash");
    }

    // Error on truncated instruction data (length mismatch)
    #[test]
    fn test_message_truncated_instruction_data() {
        // header
        let mut data = vec![1, 0, 0];
        // accounts_count = 1
        data.push(1);
        // 1 pubkey
        data.extend_from_slice(&[0u8; 32]);
        // recent_blockhash
        data.extend_from_slice(&[0u8; 32]);
        // instructions_count = 1
        data.push(1);
        // instruction:
        // program_id_index = 0
        data.push(0);
        // accounts_len = 0
        data.push(0);
        // data_len = 2, but we will provide only 1 byte
        data.push(2);
        data.push(0xAA);

        let res = deserialize_message(&data);
        assert!(res.is_err(), "expected error due to truncated instruction data");
    }

    // ----------------------------------------
    // Main test: deserialize provided Base64 transaction fixture
    // ----------------------------------------
    #[test]
    fn test_deserialize_provided_base64_tx() {
        // Unsigned Tx from the Solana SDK
        let b64 = "AQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAED4viKcSXOZTHLc68aIq0QRoeJ7pPCtJIumjEv636+oX/NLoC2Z66TEsGaE4CkHQIC/XLT0yZ7mSg2EtNl+5KzsQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAgIAAQwCAAAAQEIPAAAAAAA=";
        let raw = BASE64.decode(b64.as_bytes()).expect("failed to decode base64 transaction");
        let tx = deserialize_transaction(&raw).expect("failed to deserialize transaction");
        // Sanity checks
        assert!(!tx.signatures.is_empty(), "expected at least one signature");
        assert!(!tx.message.account_keys.is_empty(), "expected at least one account key");
        assert!(!tx.message.instructions.is_empty(), "expected at least one instruction");

        println!("------- tx: {:?}", tx);

        // Additional
        let ui_tx = UiTransaction::from(&tx); 

        println!("------- 🥂 TX: {:?}", ui_tx);
       
    }
    // ----------------------------------------
    // Utility test: generate signed transaction Base64
    // ----------------------------------------
    #[test]
    fn test_roundtrip_serde_base64_tx() {
        // Derive deterministic keypair from a fixed secret (all ones)
        let seed = [1u8; 32];
        let keypair = ed25519::keypair_from_seed(&seed).unwrap();

        // Use provided Base64-encoded unsigned tx fixture
        let b64 = "AQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAEDiojj3XQJ8ZX9UtstPLpdcspnCb8dlBIb83SIAbQPb1yBOXcOqH0XX1ajVGbDTH7My42KkbTuN6Jd9g9bj8mzlAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACQkJCQkJCQkJCQkJCQkJCQkJCQkJCQkJCQkJCQkJCQkBAgIAAQwCAAAAQEIPAAAAAAA=";
        let raw = BASE64.decode(b64.as_bytes()).expect("decode unsigned tx");
        let mut tx: solana_tools_lite::models::transaction::Transaction = deserialize_transaction(&raw).expect("failed to deserialize transaction");

        let tx_raw_again = serialize_transaction(&tx);
        let bs64_back = BASE64.encode(&tx_raw_again);

        println!("😼 bs64_back: {}", bs64_back);

        assert_eq!(raw, tx_raw_again);

        sign_transaction_by_key(&mut tx, &keypair).unwrap();

        let sig_bytes = bs58::encode(tx.signatures[0].to_bytes()).into_string();
        
        println!("sig_bytes: {:?}", sig_bytes);

        assert_eq!(sig_bytes, "5uqmwQq2f3DhLAU9Mwa51GzByKR6NrKkxELeibhs1r3PU2KdiucpBTLw2Q7o43E3VxTtUod1ksXpy8oebvNrvyLb");
    }
}