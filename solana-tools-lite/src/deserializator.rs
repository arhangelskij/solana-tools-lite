use crate::errors::DeserializeError;
use crate::errors::Result;
use crate::models::transaction::{Instruction, Message, MessageHeader, Transaction};
use ed25519_dalek::Signature;

fn deserialize_transaction(data: &[u8]) -> Result<Transaction, DeserializeError> {
    let mut cursor = 0;
    
    // 1. Читаем количество подписей (compact-u16)
    let (signatures_count, offset) = read_compact_u16(&data[cursor..])?;
    cursor += offset;
    
    // 2. Читаем подписи (по 64 байта каждая)
    let mut signatures = Vec::new();
    for _ in 0..signatures_count {
        if cursor + 64 > data.len() {
            return Err(DeserializeError::Deserialization("Not enough bytes for signature".to_string()));
        }

        let sig_bytes: [u8; 64] = data[cursor..cursor + 64].try_into().map_err(|_| DeserializeError::Deserialization("Invalid signature length".to_string()))?;
        signatures.push(Signature::from_bytes(&sig_bytes));
        cursor += 64;
    }
    
    // 3. Парсим Message
    let message = deserialize_message(&data[cursor..])?;

    Ok(Transaction {signatures, message})
}

fn deserialize_message(data: &[u8]) -> Result<Message, DeserializeError> {
    let mut cursor = 0;
    
    // 1. MessageHeader (3 байта)
    let header = MessageHeader {
        num_required_signatures: data[cursor],
        num_readonly_signed_accounts: data[cursor + 1],
        num_readonly_unsigned_accounts: data[cursor + 2],
    };
    cursor += 3;
    
    // 2. Account keys
    let (accounts_count, offset) = read_compact_u16(&data[cursor..])?;
    cursor += offset;
    
    let mut account_keys = Vec::new();
    for _ in 0..accounts_count {
        let pubkey_bytes = &data[cursor..cursor + 32];
        account_keys.push(bs58::encode(pubkey_bytes).into_string());
        cursor += 32;
    }
    
    // 3. Recent blockhash (32 байта)
    let blockhash_bytes = &data[cursor..cursor + 32];
    let recent_blockhash = bs58::encode(blockhash_bytes).into_string();
    cursor += 32;
    
    // 4. Instructions
    let (instructions_count, offset) = read_compact_u16(&data[cursor..])?;
    cursor += offset;
    
    let mut instructions = Vec::new();
    for _ in 0..instructions_count {
        let instruction = parse_instruction(&data[cursor..], &mut cursor)?;
        instructions.push(instruction);
    }
    
    Ok(Message {
        header,
        account_keys,
        recent_blockhash,
        instructions,
    })
}

/// Helpers

fn read_compact_u16(data: &[u8]) -> Result<(usize, usize), DeserializeError> {
    if data.is_empty() {
        return Err(DeserializeError::Deserialization("data is empty".to_string()));
    }

    let first_byte = data[0];
    match first_byte {
        0..=127 => Ok((first_byte as usize, 1)),
        128..=255 => {
            if data.len() < 2 {
                return Err(DeserializeError::Deserialization("Not enough bytes for compact u16".to_string()));
            }
            //TODO: add comments
            let value = ((first_byte as u16 - 128) | ((data[1] as u16) << 7)) as usize;
            Ok((value, 2))
        }
    }
}

fn parse_instruction(data: &[u8], cursor: &mut usize) -> Result<Instruction, DeserializeError> {
    if *cursor + 1 > data.len() {
        return  Err(DeserializeError::Deserialization("Not enough bytes for program_id_index".to_string()));
    }
    let program_id_index = data[*cursor];
    *cursor += 1;

    // Read accounts length
    let (accounts_len, offset) = read_compact_u16(&data[*cursor..])?;
    *cursor += offset;

    // Read accounts
    if *cursor + accounts_len > data.len() {
        return Err(DeserializeError::Deserialization("Not enough bytes for accounts".to_string()));
    }
    let accounts = data[*cursor..*cursor + accounts_len].to_vec();
    *cursor += accounts_len;

    // Read data length
    let (data_len, offset) = read_compact_u16(&data[*cursor..])?;
    *cursor += offset;

    // Read data
    if *cursor + data_len > data.len() {
        return Err(DeserializeError::Deserialization("Not enough bytes for instruction data".to_string()));
    }
    let data_bytes = data[*cursor..*cursor + data_len].to_vec();
    *cursor += data_len;

    Ok(Instruction {
        program_id_index,
        accounts,
        data: data_bytes,
    })
}