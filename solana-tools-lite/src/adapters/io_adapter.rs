use crate::errors::{Result, SignError, TransactionParseError, ToolError};
use crate::layers::io;
use crate::models::input_transaction::{InputTransaction, UiTransaction};
use serde_json;
use std::io as std_io;
use std::path::Path;
use data_encoding::BASE64;

pub enum InputFormat {
    Json,
    Base64,
    Base58,
}

pub enum OutputFormat {
    Json { pretty: bool },
    Base64,
    Base58,
}
/// Read from a file or stdin ("-") based on `path`.
fn read_input(path: Option<&str>) -> std::result::Result<String, SignError> {
    match path {
        Some(p) if p != "-" => io::read_from_file(Path::new(p)).map_err(|e| SignError::IoWithPath {
            source: e,
            path: Some(p.to_string()),
        }),
        _ => io::read_from_stdin().map_err(|e| SignError::IoWithPath { source: e, path: None }),
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
    allow_stdin: bool,
) -> Result<String> {
    match (inline, file) {
        (Some(s), None) => Ok(s.to_owned()),

        //TODO: check comments
        // Stdin 
        (None, Some("-")) => {
            if !allow_stdin {
                return Err(ToolError::InvalidInput("reading from stdin is disabled".to_string()));
            }
            read_input(None).map_err(ToolError::from)
        }
        // From path
        (None, Some(path)) => read_input(Some(path)).map_err(ToolError::from),
        
        // Errors
        (Some(_), Some(_)) => Err(ToolError::InvalidInput(
            "provide either inline value or --from-file (not both)".to_string(),
        )),
        
        (None, None) => Err(ToolError::InvalidInput(
            "missing input: pass inline value or --from-file".to_string(),
        )),
    }
}

//TODO: 1/09 🔴(tomorrow first) mb make it private too
/// Write data to a file or stdout; stdout is written as-is; file uses 0o644 perms and overwrites.
pub fn write_output(path: Option<&str>, data: &str) -> std::result::Result<(), SignError> {
    // Public output: stdout allowed, 0644 permissions, always overwrite
    let target = match path {
        Some(p) if p != "-" => OutputTarget::File(Path::new(p)),
        _ => OutputTarget::Stdout,
    };
    write_bytes_with_opts(target, data.as_bytes(), 0o644, true)
}

pub fn read_input_transaction(input: Option<&str>) -> Result<InputTransaction> {
    // Read raw text via IO layer first (file or stdin), then detect format
    let raw = read_input(input)?;
    crate::serde::input_tx::parse_input_transaction(Some(&raw)).map_err(ToolError::from)
}

pub fn read_secret_key_file(path: &str) -> std::result::Result<String, SignError> {
    // For security reasons, reading secret keys from stdin is disabled.
    if path == "-" {
        return Err(SignError::IoWithPath {
            source: std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "reading secret key from stdin is disabled",
            ),
            path: Some("-".to_string()),
        });
    }

    let p = Path::new(path);

    if !p.exists() {
        return Err(SignError::IoWithPath {
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "secret key file not found"),
            path: Some(path.to_string()),
        });
    }

    if !p.is_file() {
        return Err(SignError::IoWithPath {
            source: std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "secret key path is not a file",
            ),
            path: Some(path.to_string()),
        });
    }

    let s = read_input(Some(path))?;

    Ok(s.trim().to_string())
}

/// Serialize a `UiTransaction` into the specified `OutputFormat` and write it out.
pub fn write_output_transaction(
    transaction: &UiTransaction,
    format: OutputFormat,
    output: Option<&str>,
) -> Result<()> {
    // First, serialize to JSON (for both JSON and encoded formats)
    let json_str = serde_json::to_string(transaction)
        .map_err(|e| TransactionParseError::Serialization(e.to_string()))?;

    // Build the output string based on desired format
    let out_str = match format {
        OutputFormat::Json { pretty } => {
            if pretty {
                serde_json::to_string_pretty(transaction)
                    .map_err(|e| TransactionParseError::Serialization(e.to_string()))?
            } else {
                json_str.clone()
            }
        }
        OutputFormat::Base64 => BASE64.encode(&json_str.as_bytes()),
        OutputFormat::Base58 => bs58::encode(&json_str).into_string(),
    };

    // Write to file or stdout
    write_output(output, &out_str).map_err(|e| e)?;

    Ok(())
}

/// Mnemonic

/// Read mnemonic from file or stdin (`-`) and normalize whitespace.
pub fn read_mnemonic(input: &str) -> Result<String, SignError> {
    let raw = read_input(if input == "-" { None } else { Some(input) })?;
    Ok(raw.split_whitespace().collect::<Vec<_>>().join(" "))
}

/// Write secret material to a file path, never to stdout.
/// - `path` must be a filesystem path ("-" is rejected)
/// - if the file exists and `force == false`, returns an AlreadyExists error
/// - on Unix, sets permissions to 0o600 (rw-------)
pub fn write_secret_file(path: &Path, data: &str, force: bool) -> std::result::Result<(), SignError> {
    // Secrets: stdout forbidden, 0600, respect force
    if path == Path::new("-") {
        return Err(SignError::IoWithPath {
            source: std_io::Error::new(
                std_io::ErrorKind::InvalidInput,
                "refusing to write secrets to stdout (-)",
            ),
            path: Some(path.display().to_string()),
        });
    }
//TODO: 🟡 1sept use const for perms
    write_bytes_file_with_opts(path, data.as_bytes(), 0o600, force)
}

/// Write non-secret public artifact to a file path, respecting `force` and using 0o644 perms.
/// Stdout is not allowed here (use `write_output` for stdout writes).
pub fn write_public_file(path: &Path, data: &str, force: bool) -> std::result::Result<(), SignError> {
    write_bytes_file_with_opts(path, data.as_bytes(), 0o644, force)
}

//TODO: 🟡 unused?
/// Read a single-line secret-like text (file or stdin), trimmed.
// pub fn read_text(input: &str) -> Result<String, SignError> {
//     let raw = read_input(if input == "-" { None } else { Some(input) })?;
//     Ok(raw.trim().to_string())
// }

//TODO: 27 aug 🔴 new write func

use std::fs::OpenOptions;
use std::io::Write;
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;

/// Output target for low-level writers.
///
/// - `Stdout`: write bytes to standard output (no permission or overwrite semantics apply).
/// - `File(&Path)`: write bytes to the given filesystem path.
enum OutputTarget<'a> {
    Stdout,
    File(&'a Path),
}

/// Low-level writer: writes to either stdout or a file depending on `target`.
///
/// - For `OutputTarget::Stdout`, writes bytes as-is to stdout and flushes.
/// - For `OutputTarget::File`, delegates to `write_bytes_file_with_opts` with the provided
///   permissions (`perms`) and overwrite policy (`force`).
///
/// This helper centralizes the “stdout vs file” branching so upper layers can express intent
/// clearly by constructing the appropriate `OutputTarget`.
fn write_bytes_with_opts(
    target: OutputTarget,
    bytes: &[u8],
    perms: u32,
    force: bool,
) -> std::result::Result<(), SignError> {
    match target {
        OutputTarget::Stdout => {
            let mut stdout = std_io::stdout();
            stdout.write_all(bytes).map_err(|e| SignError::IoWithPath {
                source: e,
                path: None,
            })?;
            stdout.flush().map_err(|e| SignError::IoWithPath {
                source: e,
                path: None,
            })?;
            Ok(())
        }
        OutputTarget::File(p) => write_bytes_file_with_opts(p, bytes, perms, force),
    }
}

/// File-only writer: safely writes bytes to a filesystem path.
///
/// - Honors `force`: when `false`, uses `create_new(true)` to atomically fail if the file exists;
///   when `true`, truncates or creates the file.
/// - On Unix, sets the file mode to `perms` (e.g., 0o600 for secrets, 0o644 for public data).
/// - Never writes to stdout — use `write_bytes_with_opts(OutputTarget::Stdout, ...)` instead.
fn write_bytes_file_with_opts(
    path: &Path,
    bytes: &[u8],
    perms: u32,
    force: bool,
) -> std::result::Result<(), SignError> {
    let mut opts = OpenOptions::new();
    opts.write(true);
    
    if force {
        opts.create(true).truncate(true);
    } else {
        opts.create_new(true);
    }

    #[cfg(unix)]
    {
        opts.mode(perms);
    }

    let mut file = opts.open(path).map_err(|e| SignError::IoWithPath {
        source: e,
        path: Some(path.display().to_string()),
    })?;

    file.write_all(bytes).map_err(|e| SignError::IoWithPath {
        source: e,
        path: Some(path.display().to_string()),
    })?;

    Ok(())
}