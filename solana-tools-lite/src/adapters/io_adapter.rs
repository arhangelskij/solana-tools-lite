use crate::constants::permission::{FILE_PERMS_PUBLIC, FILE_PERMS_SECRET};
use crate::errors::{IoError, Result, ToolError};
use crate::layers::io;
use crate::models::input_transaction::{InputTransaction, UiTransaction};
use crate::serde::fmt::{self as serde_fmt, OutputFormat};
use std::io as std_io;
use std::path::{Path, PathBuf};

// Private source enum: used internally to model a single text input source
enum TextSource<'a> {
    Inline(&'a str),
    File(&'a str),
    Stdin,
}

/// Read from a file or stdin ("-") based on `path`.
/// Returns adapter-level IoError with optional path context.
fn read_input(path: Option<&str>) -> std::result::Result<String, IoError> {
    match path {
        Some(p) if p != "-" => io::read_from_file(Path::new(p)).map_err(|e| IoError::IoWithPath {
            source: e,
            path: Some(p.to_string()),
        }),
        _ => io::read_from_stdin().map_err(|e| IoError::IoWithPath {
            source: e,
            path: None,
        }),
    }
}

/// Resolve text either from an inline value or from a file/stdin ("-").
/// Returns raw text exactly as read (no trimming applied).
/// Caller is responsible for trimming when appropriate (e.g., Base58/Base64 inputs).
///
/// Prefer using higher-level helpers when possible:
/// - `read_message(...)` — preserves bytes as-is (no trim)
/// - `read_signature(...)` — trims trailing whitespace/newlines
/// - `read_pubkey(...)` — trims trailing whitespace/newlines
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
    let src = to_text_source(inline, file)?;
    resolve_text_source(src, allow_stdin)
}

/// Writes public data either to stdout or to a file.
///
/// - When `path` is `None` or `Some("-")`, writes to stdout as-is.
/// - When `path` is `Some(p)`, writes to file `p` with permissions 0o644.
/// - Always overwrites existing files (does not respect `--force`).
/// - Not intended for secrets; use `write_secret_file` for secret material.
fn write_output(path: Option<&str>, data: &str) -> std::result::Result<(), ToolError> {
    // Public output: stdout allowed, 0644 permissions, always overwrite
    let target = match path {
        Some(p) if p != "-" => OutputTarget::File(Path::new(p)),
        _ => OutputTarget::Stdout,
    };

    write_bytes_with_opts(&target, data.as_bytes(), FILE_PERMS_PUBLIC, true).map_err(|e| {
        match target {
            OutputTarget::Stdout => ToolError::Io(IoError::IoWithPath {
                source: e,
                path: None,
            }),
            OutputTarget::File(p) => ToolError::Io(IoError::IoWithPath {
                source: e,
                path: Some(p.display().to_string()),
            }),
        }
    })
}

pub fn read_input_transaction(input: Option<&str>) -> Result<InputTransaction> {
    // Read raw text via IO layer first (file or stdin), then detect format
    let raw = match input {
        Some(p) => read_input(Some(p))?,
        None => read_input(None)?,
    };
    crate::serde::input_tx::parse_input_transaction(Some(&raw)).map_err(ToolError::from)
}

pub fn read_secret_key_file(path: &str) -> std::result::Result<String, ToolError> {
    // For security reasons, reading secret keys from stdin is disabled.
    if path == "-" {
        return Err(ToolError::Io(IoError::IoWithPath {
            source: std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "reading secret key from stdin is disabled",
            ),
            path: Some("-".to_string()),
        }));
    }

    let p = Path::new(path);

    if !p.exists() {
        return Err(ToolError::Io(IoError::IoWithPath {
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "secret key file not found"),
            path: Some(path.to_string()),
        }));
    }

    if !p.is_file() {
        return Err(ToolError::Io(IoError::IoWithPath {
            source: std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "secret key path is not a file",
            ),
            path: Some(path.to_string()),
        }));
    }

    let s = read_input(Some(path)).map_err(ToolError::Io)?;

    Ok(s.trim().to_string())
}

/// Serialize a `UiTransaction` into the specified `OutputFormat` and write it out.
///
/// Behavior
/// - Uses `serde::fmt::encode_ui_transaction` to build the output string
///   (JSON pretty/plain, Base64(JSON), or Base58(JSON)).
/// - Writes to stdout when `output` is `None` or `Some("-")`.
/// - Writes to file with 0o644 and always overwrites when `output = Some(path)`.
pub fn write_output_transaction(
    transaction: &UiTransaction,
    format: OutputFormat,
    output: Option<&str>,
) -> Result<()> {
    // Encode UI Tx
    let out_str = serde_fmt::encode_ui_transaction(transaction, format)?;

    // Write to file or stdout
    write_output(output, &out_str)?;

    Ok(())
}

/// Resolve final path for a possibly-directory `output_path` by appending `default_filename`.
/// If `output_path` is a file path, returns it unchanged.
pub fn get_final_path_with_default(output_path: Option<&str>, default_filename: &str) -> PathBuf {
    match output_path {
        Some(path_str) => {
            let p = Path::new(path_str);
            if p.is_dir() {
                p.join(default_filename)
            } else {
                p.to_path_buf()
            }
        }
        None => PathBuf::from(default_filename),
    }
}

/// Save any serializable value as pretty JSON to a public file.
/// - When `out_path` is `Some`, writes to that path (directory is allowed; appends `default_filename`).
/// - When `out_path` is `None`, does nothing and returns `Ok(None)`.
/// - Respects `force` semantics and uses public file permissions (0644).
pub fn save_pretty_json<T: serde::Serialize>(
    value: &T,
    out_path: Option<&str>,
    force: bool,
    default_filename: &str,
) -> Result<Option<PathBuf>> {
    let json_str = serde_json::to_string_pretty(value)
        .map_err(|e| ToolError::InvalidInput(format!("failed to serialize JSON: {e}")))?;

    let saved = match out_path {
        Some(_) => {
            let target = get_final_path_with_default(out_path, default_filename);
            write_public_file(&target, &json_str, force)?;
            Some(target)
        }
        None => None, //TODO: 🟠 add test on this case
    };
    Ok(saved)
}

/// Read mnemonic from file or stdin (`-`) and normalize whitespace.
///
/// This helper is intended for CLI flows that accept a mnemonic from a file or stdin.
/// It collapses any whitespace (spaces, tabs, newlines) into single spaces.
pub fn read_mnemonic(input: &str) -> Result<String> {
    let path = match input {
        "-" => None,
        _ => Some(input),
    };
    let raw = read_input(path).map_err(ToolError::Io)?;
    Ok(raw.split_whitespace().collect::<Vec<_>>().join(" ")) //TODO: 🟠 magic whitespace!
}

/// Read passphrase from file or stdin ("-") without altering internal whitespace.
/// Trims only trailing newlines ("\n"/"\r\n").
pub fn read_passphrase(input: &str) -> Result<String> {
    let path = match input {
        "-" => None,
        _ => Some(input),
    };
    let raw = read_input(path).map_err(ToolError::Io)?;
    Ok(raw.trim_end_matches(['\r', '\n']).to_string())
}

/// Write secret material to a file path, never to stdout.
/// - `path` must be a filesystem path ("-" is rejected)
/// - If the file exists and `force == false`, returns AlreadyExists (atomic via create_new)
/// - On Unix, sets permissions to 0o600 (rw-------)
pub fn write_secret_file(
    path: &Path,
    data: &str,
    force: bool,
) -> std::result::Result<(), ToolError> {
    // Secrets: stdout forbidden, 0600, respect force
    if path == Path::new("-") {
        return Err(ToolError::Io(IoError::IoWithPath {
            source: std_io::Error::new(
                std_io::ErrorKind::InvalidInput,
                "refusing to write secrets to stdout (-)",
            ),
            path: Some(path.display().to_string()),
        }));
    }
    write_bytes_file_with_opts(path, data.as_bytes(), FILE_PERMS_SECRET, force).map_err(|e| {
        ToolError::Io(IoError::IoWithPath {
            source: e,
            path: Some(path.display().to_string()),
        })
    })
}

/// Write non-secret public artifact to a file path, respecting `force` and using 0o644 perms.
/// - Stdout is NOT allowed here (use `write_output` for stdout writes).
pub fn write_public_file(
    path: &Path,
    data: &str,
    force: bool,
) -> std::result::Result<(), ToolError> {
    write_bytes_file_with_opts(path, data.as_bytes(), FILE_PERMS_PUBLIC, force).map_err(|e| {
        ToolError::Io(IoError::IoWithPath {
            source: e,
            path: Some(path.display().to_string()),
        })
    })
}

use std::fs::OpenOptions;
use std::io::Write;
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;

// --- Internal helpers for source resolution ---

fn to_text_source<'a>(inline: Option<&'a str>, file: Option<&'a str>) -> Result<TextSource<'a>> {
    match (inline, file) {
        (Some(s), None) => Ok(TextSource::Inline(s)),
        (None, Some("-")) => Ok(TextSource::Stdin),
        (None, Some(path)) => Ok(TextSource::File(path)),
        (Some(_), Some(_)) => Err(ToolError::InvalidInput(
            "provide either inline value or --from-file (not both)".to_string(),
        )),
        (None, None) => Err(ToolError::InvalidInput(
            "missing input: pass inline value or --from-file".to_string(),
        )),
    }
}

fn resolve_text_source(src: TextSource<'_>, allow_stdin: bool) -> Result<String> {
    match src {
        TextSource::Inline(s) => Ok(s.to_owned()),
        TextSource::File(p) => read_input(Some(p)).map_err(ToolError::Io),
        TextSource::Stdin => {
            if !allow_stdin {
                return Err(ToolError::InvalidInput(
                    "reading from stdin is disabled".to_string(),
                ));
            }
            read_input(None).map_err(ToolError::Io)
        }
    }
}

// --- Public small helpers for flows ---

/// Read message from inline/file/stdin without trimming (preserves exact bytes)
pub fn read_message(inline: Option<&str>, file: Option<&str>) -> Result<String> {
    let src = to_text_source(inline, file)?;
    resolve_text_source(src, true)
}

/// Read signature from inline/file/stdin and trim trailing whitespace/newlines
pub fn read_signature(inline: Option<&str>, file: Option<&str>) -> Result<String> {
    let src = to_text_source(inline, file)?;
    Ok(resolve_text_source(src, true)?.trim().to_string())
}

/// Read public key from inline/file/stdin and trim trailing whitespace/newlines
pub fn read_pubkey(inline: Option<&str>, file: Option<&str>) -> Result<String> {
    let src = to_text_source(inline, file)?;
    Ok(resolve_text_source(src, true)?.trim().to_string())
}

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
    target: &OutputTarget,
    bytes: &[u8],
    perms: u32,
    force: bool,
) -> std::result::Result<(), std::io::Error> {
    match target {
        OutputTarget::Stdout => {
            let mut stdout = std_io::stdout();
            stdout.write_all(bytes)?;
            stdout.flush()?;
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
) -> std::result::Result<(), std::io::Error> {
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

    let mut file = opts.open(path)?;

    file.write_all(bytes)?;

    Ok(())
}
