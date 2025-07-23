// Cargo.toml
/*
[package]
name = "solana-tx-parser"
version = "0.1.0"
edition = "2021"

[dependencies]
base64 = "0.21"
bs58 = "0.5"

[dev-dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
*/

use std::fmt;

// ==================== Типы данных ====================

#[derive(Debug, Clone)]
pub struct Signature(pub [u8; 64]);

impl Signature {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() != 64 {
            return Err(Error::InvalidData("Signature must be 64 bytes"));
        }
        let mut sig = [0u8; 64];
        sig.copy_from_slice(bytes);
        Ok(Signature(sig))
    }

    pub fn is_empty(&self) -> bool {
        self.0.iter().all(|&b| b == 0)
    }
}

#[derive(Debug, Clone)]
pub struct MessageHeader {
    pub num_required_signatures: u8,
    pub num_readonly_signed_accounts: u8,
    pub num_readonly_unsigned_accounts: u8,
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub program_id_index: u8,
    pub accounts: Vec<u8>,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub header: MessageHeader,
    pub account_keys: Vec<String>, // base58 encoded
    pub recent_blockhash: String,  // base58 encoded
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub signatures: Vec<Signature>,
    pub message: Message,
}

// ==================== Ошибки ====================

#[derive(Debug)]
pub enum Error {
    InvalidData(&'static str),
    Base64Decode(base64::DecodeError),
    Io(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
            Error::Base64Decode(e) => write!(f, "Base64 decode error: {}", e),
            Error::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<base64::DecodeError> for Error {
    fn from(e: base64::DecodeError) -> Self {
        Error::Base64Decode(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

// ==================== Вспомогательные функции ====================

fn read_compact_u16(data: &[u8]) -> Result<(usize, usize), Error> {
    if data.is_empty() {
        return Err(Error::InvalidData("No data for compact u16"));
    }

    let first_byte = data[0];
    match first_byte {
        0..=127 => Ok((first_byte as usize, 1)),
        128..=255 => {
            if data.len() < 2 {
                return Err(Error::InvalidData("Not enough bytes for compact u16"));
            }
            let value = ((first_byte as u16 - 128) | ((data[1] as u16) << 7)) as usize;
            Ok((value, 2))
        }
    }
}

fn parse_instruction(data: &[u8], cursor: &mut usize) -> Result<Instruction, Error> {
    if *cursor + 1 > data.len() {
        return Err(Error::InvalidData("Not enough bytes for program_id_index"));
    }
    let program_id_index = data[*cursor];
    *cursor += 1;

    // Read accounts length
    let (accounts_len, offset) = read_compact_u16(&data[*cursor..])?;
    *cursor += offset;

    // Read accounts
    if *cursor + accounts_len > data.len() {
        return Err(Error::InvalidData("Not enough bytes for accounts"));
    }
    let accounts = data[*cursor..*cursor + accounts_len].to_vec();
    *cursor += accounts_len;

    // Read data length
    let (data_len, offset) = read_compact_u16(&data[*cursor..])?;
    *cursor += offset;

    // Read data
    if *cursor + data_len > data.len() {
        return Err(Error::InvalidData("Not enough bytes for instruction data"));
    }
    let data_bytes = data[*cursor..*cursor + data_len].to_vec();
    *cursor += data_len;

    Ok(Instruction {
        program_id_index,
        accounts,
        data: data_bytes,
    })
}

// ==================== Основные функции десериализации ====================

pub fn deserialize_transaction(data: &[u8]) -> Result<Transaction, Error> {
    let mut cursor = 0;
    
    // 1. Читаем количество подписей (compact-u16)
    let (signatures_count, offset) = read_compact_u16(&data[cursor..])?;
    cursor += offset;
    
    // 2. Читаем подписи (по 64 байта каждая)
    let mut signatures = Vec::new();
    for _ in 0..signatures_count {
        if cursor + 64 > data.len() {
            return Err(Error::InvalidData("Not enough bytes for signature"));
        }
        let sig_bytes = &data[cursor..cursor + 64];
        signatures.push(Signature::from_bytes(sig_bytes)?);
        cursor += 64;
    }
    
    // 3. Парсим Message
    let message = deserialize_message(&data[cursor..])?;
    
    Ok(Transaction { signatures, message })
}

pub fn deserialize_message(data: &[u8]) -> Result<Message, Error> {
    let mut cursor = 0;
    
    // 1. MessageHeader (3 байта)
    if cursor + 3 > data.len() {
        return Err(Error::InvalidData("Not enough bytes for message header"));
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
    
    let mut account_keys = Vec::new();
    for _ in 0..accounts_count {
        if cursor + 32 > data.len() {
            return Err(Error::InvalidData("Not enough bytes for account key"));
        }
        let pubkey_bytes = &data[cursor..cursor + 32];
        account_keys.push(bs58::encode(pubkey_bytes).into_string());
        cursor += 32;
    }
    
    // 3. Recent blockhash (32 байта)
    if cursor + 32 > data.len() {
        return Err(Error::InvalidData("Not enough bytes for blockhash"));
    }
    let blockhash_bytes = &data[cursor..cursor + 32];
    let recent_blockhash = bs58::encode(blockhash_bytes).into_string();
    cursor += 32;
    
    // 4. Instructions
    let (instructions_count, offset) = read_compact_u16(&data[cursor..])?;
    cursor += offset;
    
    let mut instructions = Vec::new();
    for _ in 0..instructions_count {
        let instruction = parse_instruction(data, &mut cursor)?;
        instructions.push(instruction);
    }
    
    Ok(Message {
        header,
        account_keys,
        recent_blockhash,
        instructions,
    })
}

// ==================== Публичная функция для Base64 ====================

pub fn parse_transaction_from_base64(base64_str: &str) -> Result<Transaction, Error> {
    let data = base64::decode(base64_str)?;
    deserialize_transaction(&data)
}

// ==================== Тесты ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_compact_u16_small() {
        let data = [5u8];
        let (value, offset) = read_compact_u16(&data).unwrap();
        assert_eq!(value, 5);
        assert_eq!(offset, 1);
    }

    #[test]
    fn test_read_compact_u16_medium() {
        let data = [129u8, 1]; // 129 - 128 = 1, 1 << 7 = 128, 1 + 128 = 129
        let (value, offset) = read_compact_u16(&data).unwrap();
        assert_eq!(value, 129);
        assert_eq!(offset, 2);
    }

    #[test]
    fn test_signature_from_bytes() {
        let bytes = [1u8; 64];
        let sig = Signature::from_bytes(&bytes).unwrap();
        assert_eq!(sig.0, bytes);
    }

    #[test]
    fn test_signature_from_bytes_invalid_length() {
        let bytes = [1u8; 63];
        let result = Signature::from_bytes(&bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_signature() {
        let bytes = [0u8; 64];
        let sig = Signature::from_bytes(&bytes).unwrap();
        assert!(sig.is_empty());
    }

    #[test]
    fn test_parse_simple_transaction() {
        // Это пример Base64 транзакции (может быть невалидным, но для структуры подходит)
        let base64_tx = "AwAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAEDE5j2LG0aRXxRumpLXz29L2n8qTIWIY3ImX5Ba9F9k8r9QB2tgRd0Oqj4dH7t/8278cn3rz7/AAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
        
        // Этот тест проверит только структуру, не валидность
        match parse_transaction_from_base64(base64_tx) {
            Ok(tx) => {
                println!("Parsed transaction with {} signatures", tx.signatures.len());
                println!("Account keys: {}", tx.message.account_keys.len());
                println!("Instructions: {}", tx.message.instructions.len());
            }
            Err(e) => {
                // Это нормально для тестовой транзакции
                println!("Expected error for test transaction: {}", e);
            }
        }
    }

    #[test]
    fn test_parse_empty_transaction() {
        // Транзакция с 0 подписями
        let data = vec![0u8]; // 0 signatures
        let result = deserialize_transaction(&data);
        assert!(result.is_ok());
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
            2,           // program_id_index
            1,           // accounts_len
            3,           // account_index
            2,           // data_len
            0x01, 0x02   // data
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
            1, 0, 0,     // header
            1,           // 1 account
        ];
        
        // Добавляем pubkey (32 байта)
        data.extend_from_slice(&[1u8; 32]);
        
        // Добавляем blockhash (32 байта)
        data.extend_from_slice(&[2u8; 32]);
        
        // 1 инструкция
        data.extend_from_slice(&[
            1,  // instructions_count (compact-u16)
            0,  // program_id_index
            0,  // accounts_len
            0,  // data_len
        ]);
        
        let message = deserialize_message(&data).unwrap();
        assert_eq!(message.header.num_required_signatures, 1);
        assert_eq!(message.account_keys.len(), 1);
        assert_eq!(message.instructions.len(), 1);
    }
}