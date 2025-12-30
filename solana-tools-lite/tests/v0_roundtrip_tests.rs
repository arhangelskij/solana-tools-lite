use solana_tools_lite::codec::{
    deserialize_transaction, serialize_transaction, write_shortvec_len,
};

// Build a minimal v0 transaction (one signer, one lookup) and ensure roundtrip serialize/deserialize.
#[test]
fn v0_roundtrip_serialize_deserialize() {
    // ---- Build message bytes (v0)
    let mut msg_bytes = Vec::new();
    msg_bytes.push(0x80 | 0); // versioned message prefix, version 0

    // Header: 1 required signature, 0/1 readonly
    msg_bytes.extend_from_slice(&[1, 0, 1]);

    // Static account keys (2)
    write_shortvec_len(2, &mut msg_bytes);
    msg_bytes.extend_from_slice(&[1u8; 32]); // signer
    msg_bytes.extend_from_slice(&[2u8; 32]); // program id

    // Recent blockhash
    msg_bytes.extend_from_slice(&[9u8; 32]);

    // Instructions: 1 instruction
    write_shortvec_len(1, &mut msg_bytes);
    // instr: program_id_index = 1, accounts [0], data [0xaa, 0xbb]
    msg_bytes.push(1); // program_id_index
    write_shortvec_len(1, &mut msg_bytes); // accounts len
    msg_bytes.push(0); // account 0 (signer)
    write_shortvec_len(2, &mut msg_bytes); // data len
    msg_bytes.extend_from_slice(&[0xaa, 0xbb]);

    // Address table lookups: 1
    write_shortvec_len(1, &mut msg_bytes);
    msg_bytes.extend_from_slice(&[7u8; 32]); // lookup table account key
    write_shortvec_len(1, &mut msg_bytes); // writable indexes
    msg_bytes.push(0);
    write_shortvec_len(1, &mut msg_bytes); // readonly indexes
    msg_bytes.push(1);

    // ---- Build full transaction bytes: signatures + message
    let mut tx_bytes = Vec::new();
    write_shortvec_len(1, &mut tx_bytes); // one signature slot
    tx_bytes.extend_from_slice(&[0u8; 64]); // placeholder signature
    tx_bytes.extend_from_slice(&msg_bytes);

    // Deserialize -> serialize
    let tx = deserialize_transaction(&tx_bytes).expect("v0 tx should deserialize");
    let roundtrip = serialize_transaction(&tx);

    assert_eq!(roundtrip, tx_bytes, "v0 transaction must roundtrip");
}
