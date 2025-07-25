#[cfg(test)]
mod tests {
    use ed25519_dalek::Signature;
    use solana_tools_lite::deserializator::*;
    use std::convert::TryFrom;

    #[test]
    fn test_read_shortvec_len_small() {
        let data = [5u8];
        let (value, offset) = read_shortvec_len(&data).unwrap();
        assert_eq!(value, 5);
        assert_eq!(offset, 1);
    }

    #[test]
    fn test_read_shortvec_len_two_bytes_128() {
        // short-vec two-byte encoding for 128 is 0x80 0x01
        let data = [0x80u8, 0x01];
        let (value, offset) = read_shortvec_len(&data).unwrap();
        assert_eq!(value, 128);
        assert_eq!(offset, 2);
    }

    #[test]
    fn test_signature_from_bytes() {
        let bytes = [1u8; 64];
        let sig = Signature::try_from(&bytes[..]).expect("valid signature bytes must parse");
        assert_eq!(sig.to_bytes(), bytes);
    }

    #[test]
    fn test_signature_from_bytes_invalid_length() {
        let bytes = [1u8; 63];
        let result = Signature::try_from(bytes.as_slice());
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_signature() {
        let bytes = [0u8; 64];
        let sig = Signature::try_from(&bytes[..])
            .expect("zeroed signature bytes should still parse as a struct");
        assert_eq!(sig.to_bytes(), bytes);
    }

    #[test]
    fn test_insufficient_data() {
        let data = vec![1u8]; // 1 signature, но нет самих байтов
        let result = deserialize_transaction(&data);
        assert!(result.is_err());
    }

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

    ///////
    ///

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

    #[test]
    fn test_shortvec_not_enough_bytes() {
        let res = read_shortvec_len(&[0x80]);
        assert!(res.is_err(), "expected error for truncated shortvec length");
    }

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
    #[test]
    fn test_shortvec_too_long_encoding_err() {
        // 4-byte continuation should be rejected by solana-short-vec (u16 max, up to 3 bytes)
        let res = read_shortvec_len(&[0x80, 0x80, 0x80, 0x01]);
        assert!(res.is_err(), "expected error for length encoded with >3 bytes");
    }

    // -------------------------------
    // Section: Transaction / Message – positive and negative
    // -------------------------------
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
}
