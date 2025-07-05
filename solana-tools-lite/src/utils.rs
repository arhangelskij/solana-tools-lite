use serde::Serialize;
use std::io::{self, Read};
use crate::errors::{ToolError, SignError, Result};

/// Reads from stdin if input is "-", otherwise returns the argument as a string.
//TODO: looks like not used – delete if not needed
/* 
pub fn read_stdin_or_arg(arg: &str) -> io::Result<String> {
    if arg == "-" {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        Ok(buffer)
    } else {
        Ok(arg.to_string())
    }
}
    */

pub fn write_file(path: &str, data: &str) -> Result<(), SignError> {
    use std::{fs};

    fs::write(path, data).map_err(|e| SignError::IoWithPath {
        source: e,
        path: Some(path.to_owned())
    })
}

pub fn read_stdin_or_file(path: &Option<String>) -> Result<String, SignError> {
    use std::fs;
    
    let mut buf = String::new();
    match path {
        Some(p) if p != "-" => fs::read_to_string(p).map_err(|e| SignError::IoWithPath {
            source: e,
            path: Some(p.clone()),
        }),
        _ => {
            io::stdin().read_to_string(&mut buf).map_err(|e| SignError::IoWithPath {
                source: e,
                path: path.to_owned(),
            })?;
            Ok(buf)
        }
    }
}

/// Pretty-prints any serializable struct as JSON.
pub fn pretty_print_json<T: Serialize>(value: &T) {
    let output = serde_json::to_string_pretty(value)
        .unwrap_or_else(|_| "{\"error\":\"Serialization error\"}".to_string());
    println!("{output}");
}

//TODO: not used 
/// Prints error to stderr and exits with code 1.
pub fn exit_with_error(msg: &str) -> ! {
    eprintln!("Error: {msg}");
    std::process::exit(1)
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