use crate::errors::{Result, SignError};

pub fn write_output(path: Option<&str>, data: &str) -> Result<(), SignError> {
    use std::fs;
    use std::io::{self, Write};

    match path {
        Some(p) if p != "-" => {
            fs::write(p, data).map_err(|e| SignError::IoWithPath {
                source: e,
                path: Some(p.to_owned()),
            })
        }
        _ => {
            let mut stdout = io::stdout();
            stdout.write_all(data.as_bytes()).map_err(|e| SignError::IoWithPath {
                source: e,
                path: None,
            })
        }
    }
}

pub fn read_input(path: Option<&str>) -> Result<String, SignError> {
    use std::fs;
    use std::io::{self, Read};

    match path {
        Some(p) if p != "-" => {
            fs::read_to_string(p).map_err(|e| SignError::IoWithPath {
                source: e,
                path: Some(p.to_owned()),
            })
        }
        _ => {
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf).map_err(|e| SignError::IoWithPath {
                source: e,
                path: None,
            })?;
            Ok(buf)
        }
    }
}