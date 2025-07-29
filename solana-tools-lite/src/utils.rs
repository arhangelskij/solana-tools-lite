use serde::Serialize;
use crate::errors::{ToolError, Result};

/// Pretty-prints any serializable struct as JSON.
pub fn pretty_print_json<T: Serialize>(value: &T) {
    let output = serde_json::to_string_pretty(value)
        .unwrap_or_else(|_| "{\"error\":\"Serialization error\"}".to_string());
    println!("{output}");
}

/// HEX encode
pub fn hex_encode(data: &[u8]) -> String {
    hex::encode(data)
}

// utils/codec.rs
use bincode::{
    config::{standard},
    serde::encode_to_vec};

pub fn serialize<T: serde::Serialize>(value: &T) -> Result<Vec<u8>> {
    // canonical LE + fixed-int
    let config = standard().with_fixed_int_encoding();
    encode_to_vec(value, config).map_err(ToolError::Bincode)
}