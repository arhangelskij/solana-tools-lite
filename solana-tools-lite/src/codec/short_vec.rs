use crate::errors::{DeserializeError, Result};
use solana_short_vec::decode_shortu16_len;

/// Read a Solana-compatible shortvec length (1-3 bytes) from data.
pub fn read_shortvec_len(data: &[u8]) -> Result<(usize, usize), DeserializeError> {
    decode_shortu16_len(data)
        .map(|(len, consumed)| (len as usize, consumed))
        .map_err(|_| DeserializeError::Deserialization("invalid short_vec length".to_string()))
}

/// Write a usize as a Solana-compatible shortvec (1-3 bytes) into `buf`.
pub fn write_shortvec_len(value: usize, buf: &mut Vec<u8>) {
    if value <= 0x7F {
        buf.push(value as u8);
    } else if value <= 0x3FFF {
        let lo = (value as u8 & 0x7F) | 0x80;
        let hi = (value >> 7) as u8;
        buf.push(lo);
        buf.push(hi);
    } else {
        let lo = (value as u8 & 0x7F) | 0x80;
        let hi = (((value >> 7) as u8) & 0x7F) | 0x80;
        let third = (value >> 14) as u8;
        buf.push(lo);
        buf.push(hi);
        buf.push(third);
    }
}
