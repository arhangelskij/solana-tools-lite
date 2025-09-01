use std::io::{self, Read};
use std::path::Path;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

/// Reads all UTF-8 text from a file path.
pub fn read_from_file(path: &Path) -> Result<String, io::Error> {
    std::fs::read_to_string(path)
}

/// Reads all UTF-8 text from stdin.
pub fn read_from_stdin() -> Result<String, io::Error> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;
    Ok(buf)
}

/// Writes data to a file with specified permissions and force flag.
/// - If file exists and `force == false`, returns AlreadyExists error
/// - On Unix, sets permissions to `perms` (e.g. 0o600)
pub fn write_to_file(path: &Path, data: &str, perms: u32, force: bool) -> Result<(), io::Error> {
    use std::fs;

    if path.exists() && !force {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "file already exists",
        ));
    }

    fs::write(path, data)?;

    #[cfg(unix)]
    {
        if let Ok(meta) = fs::metadata(path) {
            let mut p = meta.permissions();
            p.set_mode(perms);
            let _ = fs::set_permissions(path, p);
        }
    }

    Ok(())
}
