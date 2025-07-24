use crate::errors::DeserializeError;
use crate::errors::Result;
use crate::models::pubkey_base58::PubkeyBase58;
use crate::models::transaction::{Instruction, Message, MessageHeader, Transaction};
use ed25519_dalek::Signature;
use std::convert::TryFrom;
use solana_short_vec::decode_shortu16_len;
use crate::models::hash_base58::HashBase58;

pub fn deserialize_transaction(data: &[u8]) -> Result<Transaction, DeserializeError> {
    let mut cursor = 0;

    // 1. Читаем количество подписей (compact-u16)
    let (signatures_count, offset) = read_compact_u16(&data[cursor..])?;
    cursor += offset;

    // 2. Читаем подписи (по 64 байта каждая)
    let mut signatures = Vec::with_capacity(signatures_count);
    for _ in 0..signatures_count {
        if cursor + 64 > data.len() {
            return Err(DeserializeError::Deserialization(
                "Not enough bytes for signature".to_string(),
            ));
        }

        let sig_bytes: [u8; 64] = data[cursor..cursor + 64].try_into().map_err(|_| {
            DeserializeError::Deserialization("Invalid signature length".to_string())
        })?;
        let sig = Signature::try_from(&sig_bytes[..])
            .map_err(|_| DeserializeError::Deserialization("Invalid signature bytes".to_string()))?;
        signatures.push(sig);
        cursor += 64;
    }

    // 3. Парсим Message
    let message = deserialize_message(&data[cursor..])?;

    Ok(Transaction {
        signatures,
        message,
    })
}

pub fn deserialize_message(data: &[u8]) -> Result<Message, DeserializeError> {
    let mut cursor = 0;

    // 1. MessageHeader (3 bytes)
    if data.len() < 3 {
        return Err(DeserializeError::Deserialization(
            "Not enough bytes for MessageHeader".to_string(),
        ));
    }
    let header = MessageHeader {
        num_required_signatures: data[cursor],
        num_readonly_signed_accounts: data[cursor + 1],
        num_readonly_unsigned_accounts: data[cursor + 2],
    };
    cursor += 3;

    // 2. Account keys
    let (accounts_count, offset) = read_compact_u16(&data[cursor..])?;
    cursor += offset;

    let mut account_keys: Vec<PubkeyBase58> = Vec::with_capacity(accounts_count);
    for _ in 0..accounts_count {
        if cursor + 32 > data.len() {
            return Err(DeserializeError::Deserialization(
                "Not enough bytes for pubkey".to_string(),
            ));
        }
        let pubkey_bytes: [u8; 32] = data[cursor..cursor + 32]
            .try_into()
            .map_err(|_| DeserializeError::Deserialization("Invalid pubkey length".to_string()))?;
        account_keys.push(PubkeyBase58(pubkey_bytes));
        cursor += 32;
    }

    // 3. Recent blockhash (32 bytes)
    if cursor + 32 > data.len() {
        return Err(DeserializeError::Deserialization(
            "Not enough bytes for recent blockhash".to_string(),
        ));
    }
    let blockhash_bytes: [u8; 32] = data[cursor..cursor + 32]
        .try_into()
        .map_err(|_| DeserializeError::Deserialization("Invalid blockhash length".to_string()))?;
    let recent_blockhash = HashBase58(blockhash_bytes);
    cursor += 32;

    // 4. Instructions
    let (instructions_count, offset) = read_compact_u16(&data[cursor..])?;
    cursor += offset;

    let mut instructions = Vec::with_capacity(instructions_count);
    for _ in 0..instructions_count {
        let instruction = parse_instruction(data, &mut cursor)?; // <-- передаём всё сообщение и глобальный курсор
        instructions.push(instruction);
    }

    // 5. Validate indices inside instructions
    for instr in &instructions {
        if instr.program_id_index as usize >= account_keys.len() {
            return Err(DeserializeError::Deserialization(
                "program_id_index out of bounds".to_string(),
            ));
        }
        if instr.accounts.iter().any(|&i| i as usize >= account_keys.len()) {
            return Err(DeserializeError::Deserialization(
                "account index out of bounds".to_string(),
            ));
        }
    }

    Ok(Message {
        header,
        account_keys,
        recent_blockhash,
        instructions,
    })
}

/// Helpers

pub fn read_compact_u16(data: &[u8]) -> Result<(usize, usize), DeserializeError> {
    decode_shortu16_len(data)
        .map(|(len, consumed)| (len as usize, consumed))
        .map_err(|_| DeserializeError::Deserialization("invalid short_vec length".to_string()))
}

pub fn parse_instruction(data: &[u8], cursor: &mut usize) -> Result<Instruction, DeserializeError> {
    // program_id_index (1 byte)
    if *cursor + 1 > data.len() {
        return Err(DeserializeError::Deserialization(
            "Not enough bytes for program_id_index".to_string(),
        ));
    }
    let program_id_index = data[*cursor];
    *cursor += 1;

    // accounts_len (compact-u16)
    let (accounts_len, offset) = read_compact_u16(&data[*cursor..])?;
    *cursor += offset;

    // accounts
    if *cursor + accounts_len > data.len() {
        return Err(DeserializeError::Deserialization(
            "Not enough bytes for accounts".to_string(),
        ));
    }
    let accounts = data[*cursor..*cursor + accounts_len].to_vec();
    *cursor += accounts_len;

    // data_len (compact-u16)
    let (data_len, offset) = read_compact_u16(&data[*cursor..])?;
    *cursor += offset;

    // data bytes
    if *cursor + data_len > data.len() {
        return Err(DeserializeError::Deserialization(
            "Not enough bytes for instruction data".to_string(),
        ));
    }
    let data_bytes = data[*cursor..*cursor + data_len].to_vec();
    *cursor += data_len;

    Ok(Instruction {
        program_id_index,
        accounts,
        data: data_bytes,
    })
}
