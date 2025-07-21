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
            println!("📖 Reading file: {}", p);
            
            // Check file size first
            let metadata = fs::metadata(p).map_err(|e| SignError::IoWithPath {
                source: e,
                path: Some(p.to_owned()),
            })?;
            
            let file_size = metadata.len();
            println!("📏 File size: {} bytes", file_size);
            
            if file_size > 50_000_000 { // 50MB limit
                return Err(SignError::IoWithPath {
                    source: io::Error::new(io::ErrorKind::InvalidInput, "file too large"),
                    path: Some(p.to_owned()),
                });
            }
            
            let content = fs::read_to_string(p).map_err(|e| SignError::IoWithPath {
                source: e,
                path: Some(p.to_owned()),
            })?;
            
            println!("✅ File read successfully, content length: {} chars", content.len());
            println!("🔍 First 100 chars: {:?}", &content[..content.len().min(100)]);
            
            Ok(content)
        }
        _ => {
            println!("📥 Reading from stdin...");
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf).map_err(|e| SignError::IoWithPath {
                source: e,
                path: None,
            })?;
            println!("✅ Stdin read, length: {} chars", buf.len());
            Ok(buf)
        }
    }
}

/*
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
*/