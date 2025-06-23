use serde::Serialize;
use std::io::{self, Read};

/// Reads from stdin if input is "-", otherwise returns the argument as a string.
pub fn read_stdin_or_arg(arg: &str) -> io::Result<String> {
    if arg == "-" {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        Ok(buffer)
    } else {
        Ok(arg.to_string())
    }
}

/// Pretty-prints any serializable struct as JSON.
pub fn pretty_print_json<T: Serialize>(value: &T) {
    let output = serde_json::to_string_pretty(value)
        .unwrap_or_else(|_| "{\"error\":\"Serialization error\"}".to_string());
    println!("{output}");
}

/// Prints error to stderr and exits with code 1.
pub fn exit_with_error(msg: &str) -> ! {
    eprintln!("Error: {msg}");
    std::process::exit(1)
}

/// HEX encode
pub fn hex_encode(data: &[u8]) -> String {
    hex::encode(data)
}