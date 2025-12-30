use crate::codec::short_vec::write_shortvec_len;
use crate::models::instruction::Instruction;
use crate::models::message::{Message, MessageLegacy, MessageV0};
use crate::models::{HashBase58, PubkeyBase58, Transaction};

const VERSION_PREFIX: u8 = 0x80;
const VERSION_0: u8 = 0;

/// Serialize a `Transaction` into Solana wire-format bytes.
///
/// Layout
/// - Signatures: shortvec length followed by each 64-byte signature.
/// - Message: bytes produced by `serialize_message(&tx.message)`.
///
/// Validation
/// - The pair `deserialize_transaction`/`serialize_transaction` is validated by
///   `tests/deserializer_tests.rs::test_roundtrip_serde_base64_tx` to ensure exact
///   equality with the fixture wire bytes before signing.
pub fn serialize_transaction(tx: &Transaction) -> Vec<u8> {
    let mut buf = Vec::new();

    write_shortvec_len(tx.signatures.len(), &mut buf);

    for sig in &tx.signatures {
        buf.extend_from_slice(&sig.to_bytes());
    }

    buf.extend_from_slice(&serialize_message(&tx.message));
    buf
}

/// Serialize a `Message` into Solana wire-format bytes.
///
/// Format details
/// - Header: 3 bytes (`num_required_signatures`, `num_readonly_signed_accounts`, `num_readonly_unsigned_accounts`).
/// - Account keys: shortvec length + N public keys (32 bytes each) in order.
/// - Recent blockhash: 32 bytes.
/// - Instructions: shortvec length + each instruction encoded as
///   `program_id_index (u8)` + `accounts (shortvec<u8>)` + `data (shortvec<u8>)`.
///
/// Round-trip checks
/// - Together with `deserialize_message`, this encoding is covered by
///   `tests/deserializer_tests.rs::test_roundtrip_serde_base64_tx` (via Transaction),
///   to catch regressions in expected serialization.
pub fn serialize_message(msg: &Message) -> Vec<u8> {
    match msg {
        Message::Legacy(legacy_msg) => serialize_message_legacy(legacy_msg),
        Message::V0(v0_msg) => serialize_message_v0(v0_msg),
    }
}

/// Serialize a legacy message (pre-versioned format).
pub fn serialize_message_legacy(msg: &MessageLegacy) -> Vec<u8> {
    let mut buf = Vec::new();

    buf.push(msg.header.num_required_signatures);
    buf.push(msg.header.num_readonly_signed_accounts);
    buf.push(msg.header.num_readonly_unsigned_accounts);

    write_shortvec_len(msg.account_keys.len(), &mut buf);
    for PubkeyBase58(pk) in &msg.account_keys {
        buf.extend_from_slice(pk);
    }

    let HashBase58(bh) = &msg.recent_blockhash;
    buf.extend_from_slice(bh);

    write_shortvec_len(msg.instructions.len(), &mut buf);
    for instr in &msg.instructions {
        buf.extend_from_slice(&serialize_instruction(instr));
    }
    buf
}

/// Serialize a versioned v0 message (header + lookups).
pub fn serialize_message_v0(msg: &MessageV0) -> Vec<u8> {
    let mut buf = Vec::new();

    buf.push(VERSION_PREFIX | VERSION_0);

    buf.push(msg.header.num_required_signatures);
    buf.push(msg.header.num_readonly_signed_accounts);
    buf.push(msg.header.num_readonly_unsigned_accounts);

    write_shortvec_len(msg.account_keys.len(), &mut buf);
    for PubkeyBase58(pk) in &msg.account_keys {
        buf.extend_from_slice(pk);
    }

    let HashBase58(bh) = &msg.recent_blockhash;
    buf.extend_from_slice(bh);

    write_shortvec_len(msg.instructions.len(), &mut buf);
    for instr in &msg.instructions {
        buf.extend_from_slice(&serialize_instruction(instr));
    }

    write_shortvec_len(msg.address_table_lookups.len(), &mut buf);
    for lut in &msg.address_table_lookups {
        buf.extend_from_slice(&lut.account_key.0);
        write_shortvec_len(lut.writable_indexes.len(), &mut buf);
        buf.extend_from_slice(&lut.writable_indexes);
        write_shortvec_len(lut.readonly_indexes.len(), &mut buf);
        buf.extend_from_slice(&lut.readonly_indexes);
    }

    buf
}

/// Serialize an Instruction into wire-format bytes.
pub fn serialize_instruction(instr: &Instruction) -> Vec<u8> {
    let mut buf = Vec::new();

    buf.push(instr.program_id_index);

    write_shortvec_len(instr.accounts.len(), &mut buf);
    buf.extend_from_slice(&instr.accounts);

    write_shortvec_len(instr.data.len(), &mut buf);
    buf.extend_from_slice(&instr.data);
    buf
}
