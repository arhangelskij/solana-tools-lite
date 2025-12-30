use crate::codec::short_vec::read_shortvec_len;
use crate::errors::DeserializeError;
use crate::models::{HashBase58, PubkeyBase58, Transaction};
use crate::models::instruction::Instruction;
use crate::models::message::{
    Message, MessageAddressTableLookup, MessageHeader, MessageLegacy, MessageV0,
};
use crate::Result;
use ed25519_dalek::Signature;
use std::convert::TryFrom;

const PUBKEY_LEN: usize = 32;
const BLOCKHASH_LEN: usize = 32;
const VERSION_PREFIX: u8 = 0x80;
const VERSION_0: u8 = 0;

/// Deserialize a full Solana transaction from raw bytes.
///
/// Layout:
/// 1. Signatures count (short_vec)
/// 2. Signatures (64 bytes each)
/// 3. Message (Legacy or V0)
pub fn deserialize_transaction(data: &[u8]) -> Result<Transaction, DeserializeError> {
    let mut cursor = 0;

    // 1. Read signature count (compact-u16).
    let (signatures_count, offset) = read_shortvec_len(&data[cursor..])?;
    cursor += offset;

    // 2. Read signatures (64 bytes each).
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
        let sig = Signature::try_from(&sig_bytes[..]).map_err(|_| {
            DeserializeError::Deserialization("Invalid signature bytes".to_string())
        })?;
        signatures.push(sig);
        cursor += 64;
    }

    // 3. Parse Message: Legacy or V0 by prefix
    let message = if data
        .get(cursor)
        .map(|b| b & VERSION_PREFIX != 0)
        .unwrap_or(false)
    {
        let (msg_v0, _) = deserialize_message_v0(&data[cursor..])?;
        Message::V0(msg_v0)
    } else {
        let (msg_legacy, _) = deserialize_message_legacy(&data[cursor..])?;
        Message::Legacy(msg_legacy)
    };

    Ok(Transaction {
        signatures,
        message,
    })
}

/// Deserialize a legacy message (non-versioned).
/// Returns parsed message and bytes consumed.
pub fn deserialize_message_legacy(data: &[u8]) -> Result<(MessageLegacy, usize), DeserializeError> {
    let mut cursor = 0;

    // Header (3 bytes)
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

    // Account keys (short_vec)
    let (accounts_count, offset) = read_shortvec_len(&data[cursor..])?;
    cursor += offset;

    let mut account_keys: Vec<PubkeyBase58> = Vec::with_capacity(accounts_count);
    for _ in 0..accounts_count {
        // Pubkey is always 32 bytes
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

    // Recent blockhash (32 bytes)
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

    // Instructions (short_vec)
    let (instructions_count, offset) = read_shortvec_len(&data[cursor..])?;
    cursor += offset;

    let mut instructions = Vec::with_capacity(instructions_count);
    for _ in 0..instructions_count {
        let instruction = parse_instruction(data, &mut cursor)?;
        instructions.push(instruction);
    }

    // Basic internal consistency checks:
    // Program ID and account indices must refer to existing keys in message.account_keys
    for instr in &instructions {
        if instr.program_id_index as usize >= account_keys.len() {
            return Err(DeserializeError::Deserialization(
                "program_id_index out of bounds".to_string(),
            ));
        }
        if instr
            .accounts
            .iter()
            .any(|&i| i as usize >= account_keys.len())
        {
            return Err(DeserializeError::Deserialization(
                "account index out of bounds".to_string(),
            ));
        }
    }

    Ok((
        MessageLegacy {
            header,
            account_keys,
            recent_blockhash,
            instructions,
        },
        cursor,
    ))
}

/// Deserialize a v0 message (with version prefix byte).
/// Returns parsed message and bytes consumed.
pub fn deserialize_message_v0(data: &[u8]) -> Result<(MessageV0, usize), DeserializeError> {
    let mut cursor = 0;

    // Check version prefix (highest bit set)
    let version = *data
        .get(cursor)
        .ok_or_else(|| DeserializeError::Deserialization("Missing version byte".to_string()))?;
    cursor += 1;

    // Extract version (lower 7 bits)
    if version & VERSION_PREFIX == 0 {
        return Err(DeserializeError::Deserialization(
            "Expected versioned message prefix".to_string(),
        ));
    }
    let ver = version & !VERSION_PREFIX;
    if ver != VERSION_0 {
        // Current Solana only supports v0 (0x80 prefix)
        return Err(DeserializeError::Deserialization(format!(
            "Unsupported message version: {}",
            ver
        )));
    }

    // Header (3 bytes)
    if data.len() < cursor + 3 {
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

    // Account keys (short_vec)
    let (accounts_count, offset) = read_shortvec_len(&data[cursor..])?;
    cursor += offset;

    let mut account_keys: Vec<PubkeyBase58> = Vec::with_capacity(accounts_count);
    for _ in 0..accounts_count {
        // Pubkey is always 32 bytes
        if cursor + PUBKEY_LEN > data.len() {
            return Err(DeserializeError::Deserialization(
                "Not enough bytes for pubkey".to_string(),
            ));
        }
        let pubkey_bytes: [u8; PUBKEY_LEN] = data[cursor..cursor + PUBKEY_LEN]
            .try_into()
            .map_err(|_| DeserializeError::Deserialization("Invalid pubkey length".to_string()))?;
        account_keys.push(PubkeyBase58(pubkey_bytes));
        cursor += PUBKEY_LEN;
    }

    // Recent blockhash (32 bytes)
    if cursor + BLOCKHASH_LEN > data.len() {
        return Err(DeserializeError::Deserialization(
            "Not enough bytes for recent blockhash".to_string(),
        ));
    }
    let blockhash_bytes: [u8; BLOCKHASH_LEN] = data[cursor..cursor + BLOCKHASH_LEN]
        .try_into()
        .map_err(|_| DeserializeError::Deserialization("Invalid blockhash length".to_string()))?;
    let recent_blockhash = HashBase58(blockhash_bytes);
    cursor += BLOCKHASH_LEN;

    // Instructions (short_vec)
    let (instructions_count, offset) = read_shortvec_len(&data[cursor..])?;
    cursor += offset;

    let mut instructions = Vec::with_capacity(instructions_count);
    for _ in 0..instructions_count {
        let instruction = parse_instruction(data, &mut cursor)?;
        instructions.push(instruction);
    }

    // Address Table Lookups (short_vec)
    let (lookups_count, offset) = read_shortvec_len(&data[cursor..])?;
    cursor += offset;

    let mut address_table_lookups = Vec::with_capacity(lookups_count);
    for _ in 0..lookups_count {
        // Lookup account key (32 bytes)
        if cursor + PUBKEY_LEN > data.len() {
            return Err(DeserializeError::Deserialization(
                "Not enough bytes for lookup account key".to_string(),
            ));
        }
        let account_key_bytes: [u8; PUBKEY_LEN] =
            data[cursor..cursor + PUBKEY_LEN].try_into().map_err(|_| {
                DeserializeError::Deserialization("Invalid lookup account key length".to_string())
            })?;
        cursor += PUBKEY_LEN;

        let (writable_len, offset) = read_shortvec_len(&data[cursor..])?;
        cursor += offset;
        if cursor + writable_len > data.len() {
            return Err(DeserializeError::Deserialization(
                "Not enough bytes for writable indexes".to_string(),
            ));
        }
        let writable_indexes = data[cursor..cursor + writable_len].to_vec();
        cursor += writable_len;

        let (readonly_len, offset) = read_shortvec_len(&data[cursor..])?;
        cursor += offset;
        if cursor + readonly_len > data.len() {
            return Err(DeserializeError::Deserialization(
                "Not enough bytes for readonly indexes".to_string(),
            ));
        }
        let readonly_indexes = data[cursor..cursor + readonly_len].to_vec();
        cursor += readonly_len;

        address_table_lookups.push(MessageAddressTableLookup {
            account_key: PubkeyBase58(account_key_bytes),
            writable_indexes,
            readonly_indexes,
        });
    }

    // Basic internal consistency checks for V0
    // Total accounts = static + writable lookups + readonly lookups
    let loaded_keys = address_table_lookups
        .iter()
        .map(|lut| lut.writable_indexes.len() + lut.readonly_indexes.len())
        .sum::<usize>();
    let total_keys = account_keys.len() + loaded_keys;
    for instr in &instructions {
        if instr.program_id_index as usize >= total_keys {
            return Err(DeserializeError::Deserialization(
                "program_id_index out of bounds".to_string(),
            ));
        }
        if instr.accounts.iter().any(|&i| i as usize >= total_keys) {
            return Err(DeserializeError::Deserialization(
                "account index out of bounds".to_string(),
            ));
        }
    }

    Ok((
        MessageV0 {
            header,
            account_keys,
            recent_blockhash,
            instructions,
            address_table_lookups,
        },
        cursor,
    ))
}

/// Parse a single compiled instruction from wire bytes.
pub fn parse_instruction(data: &[u8], cursor: &mut usize) -> Result<Instruction, DeserializeError> {
    if *cursor + 1 > data.len() {
        return Err(DeserializeError::Deserialization(
            "Not enough bytes for program_id_index".to_string(),
        ));
    }
    let program_id_index = data[*cursor];
    *cursor += 1;

    // Accounts indices (short_vec)
    let (accounts_len, offset) = read_shortvec_len(&data[*cursor..])?;
    *cursor += offset;

    if *cursor + accounts_len > data.len() {
        return Err(DeserializeError::Deserialization(
            "Not enough bytes for accounts".to_string(),
        ));
    }
    let accounts = data[*cursor..*cursor + accounts_len].to_vec();
    *cursor += accounts_len;

    // Opaque data (short_vec)
    let (data_len, offset) = read_shortvec_len(&data[*cursor..])?;
    *cursor += offset;

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
