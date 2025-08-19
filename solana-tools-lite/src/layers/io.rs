use crate::errors::{Result, SignError, ToolError};
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

/// Use it only in apapters
pub fn write_output(path: Option<&str>, data: &str) -> Result<(), SignError> {
    use std::fs;
    use std::io::{self, Write};

    match path {
        Some(p) if p != "-" => fs::write(p, data).map_err(|e| SignError::IoWithPath {
            source: e,
            path: Some(p.to_owned()),
        }),
        _ => {
            let mut stdout = io::stdout();
            stdout
                .write_all(data.as_bytes())
                .map_err(|e| SignError::IoWithPath {
                    source: e,
                    path: None,
                })
        }
    }
}

// TODO: 🟡🟡 think about SignError mb separate type for IO
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

            // // TODO: remove debug things
            // if file_size > 50_000_000 {
            //     // 50MB limit
            //     return Err(SignError::IoWithPath {
            //         source: io::Error::new(io::ErrorKind::InvalidInput, "file too large"),
            //         path: Some(p.to_owned()),
            //     });
            // }

            let content = fs::read_to_string(p).map_err(|e| SignError::IoWithPath {
                source: e,
                path: Some(p.to_owned()),
            })?;

            println!(
                "✅ File read successfully, content length: {} chars",
                content.len()
            );

            Ok(content)
        }
        _ => {
            println!("📥 Reading from stdin...");
            let mut buf = String::new();
            io::stdin()
                .read_to_string(&mut buf)
                .map_err(|e| SignError::IoWithPath {
                    source: e,
                    path: None,
                })?;
            println!("✅ Stdin read, length: {} chars", buf.len());
            Ok(buf)
        }
    }
}

/// Resolve text either from an inline value or from a file/stdin ("-").
/// Returns raw text exactly as read (no trimming applied).
/// Caller is responsible for trimming when appropriate (e.g. Base58/Base64 inputs).
///
/// Contract:
/// - Exactly one of `inline` or `file` must be `Some`.
/// - If `file == Some("-")`: reads from stdin when `allow_stdin == true`, otherwise returns an error.
/// - If `file == Some(path)`: reads the whole file as UTF-8 text via `read_input(Some(path))`.
/// - If `inline == Some(s)`: returns `s` as-owned `String`.
pub fn read_text_source(
    inline: Option<&str>,
    file: Option<&str>,
    allow_stdin: bool) -> Result<String> {
    match (inline, file) {
        (Some(s), None) => Ok(s.to_owned()),
        (None, Some("-")) => {
            if !allow_stdin {
                return Err(ToolError::InvalidInput("reading from stdin is disabled".to_string()));
            }
            read_input(None).map_err(ToolError::from)
        }
        (None, Some(path)) => read_input(Some(path)).map_err(ToolError::from),
        (Some(_), Some(_)) => Err(ToolError::InvalidInput(
            "provide either inline value or --from-file (not both)".to_string(),
        )),
        (None, None) => Err(ToolError::InvalidInput(
            "missing input: pass inline value or --from-file".to_string(),
        ))
    }
}

/// Write secret material to a file path, never to stdout.
/// - `path` must be a filesystem path ("-" is rejected)
/// - if the file exists and `force == false`, returns an AlreadyExists error
/// - on Unix, sets permissions to 0o600 (rw-------)
pub fn write_secret_file(path: &Path, data: &str, force: bool) -> Result<(), SignError> {
    use std::fs;
    use std::io;

    // Refuse to write secrets to stdout
    if path == Path::new("-") {
        return Err(SignError::IoWithPath {
            source: io::Error::new(
                io::ErrorKind::InvalidInput, //TODO: 🔴 refactoring?
                "refusing to write secrets to stdout (-)",
            ),
            path: Some(path.display().to_string()),
        });
    }

    let target = path;

    // If file already exists and not forced, fail fast
    if target.exists() && !force {
        return Err(SignError::IoWithPath {
            source: io::Error::new(io::ErrorKind::AlreadyExists, "file already exists"),
            path: Some(target.display().to_string()),
        });
    }

    // Write content (parent directory must exist; fs::write will error otherwise)
    fs::write(target, data).map_err(|e| SignError::IoWithPath {
        source: e,
        path: Some(target.display().to_string()),
    })?;

    // Restrict permissions on Unix
    #[cfg(unix)]
    {
        if let Ok(meta) = fs::metadata(target) {
            let mut perms = meta.permissions();
            perms.set_mode(0o600); // rw-------
            let _ = fs::set_permissions(target, perms);
        }
    }

    Ok(())
}