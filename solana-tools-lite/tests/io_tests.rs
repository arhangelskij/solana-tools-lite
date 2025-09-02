
use std::fs;
use std::path::Path;
use solana_tools_lite::layers::io::{read_from_file, write_to_file};
use solana_tools_lite::constants::permission::{FILE_PERMS_PUBLIC, FILE_PERMS_SECRET};
use solana_tools_lite::errors::{Result, ToolError, IoError};

fn io<T>(r: std::io::Result<T>) -> Result<T> { r.map_err(|e| ToolError::Io(IoError::Io(e))) }

#[test]
fn test_read_from_file_ok() -> Result<()> {
    // Prepare a temporary file with known content
    let path = "test_read.txt";
    let data = "Hello, IO!";
    io(fs::write(path, data))?;

    // Read using low-level IO layer
    let content = io(read_from_file(Path::new(path)))?;
    assert_eq!(content, data);

    // Clean up
    io(fs::remove_file(path))?;
    Ok(())
}

#[test]
fn test_read_input_invalid_path() {
    // Attempt to read a nonexistent file via IO layer, should return an error
    let result = read_from_file(Path::new("nonexistent_file.txt"));
    assert!(result.is_err());
}

#[test]
fn test_write_to_file_ok() -> Result<()> {
    // Write data to a temporary file
    let path = "test_write.txt";
    let data = "Output Data";
    io(write_to_file(Path::new(path), data, FILE_PERMS_PUBLIC, true))?;

    // Verify the file content
    let content = io(fs::read_to_string(path))?;
    assert_eq!(content, data);

    // Clean up
    io(fs::remove_file(path))?;
    Ok(())
}

#[test]
fn test_write_to_file_no_force_should_fail_on_existing() -> Result<()> {
    let path = "test_write_exists.txt";
    io(fs::write(path, "initial"))?;

    let err = write_to_file(Path::new(path), "new", FILE_PERMS_PUBLIC, false)
        .expect_err("expected AlreadyExists error when force=false");
    assert_eq!(err.kind(), std::io::ErrorKind::AlreadyExists);

    io(fs::remove_file(path))?;
    Ok(())
}

#[test]
fn test_read_from_file_directory_should_error() -> Result<()> {
    let dir = "test_io_dir_read";
    io(fs::create_dir(dir))?;
    let res = io(read_from_file(Path::new(dir)));
    
    assert!(res.is_err());
    io(fs::remove_dir(dir))?;
    Ok(())
}

#[test]
fn test_write_to_file_directory_should_error() -> Result<()> {
    let dir = "test_io_dir_write";
    io(fs::create_dir(dir))?;
    
    let res = io(write_to_file(Path::new(dir), "data", FILE_PERMS_PUBLIC, true));
    assert!(res.is_err());
    
    io(fs::remove_dir(dir))?;
    Ok(())
}

#[cfg(unix)]
#[test]
fn test_write_to_file_sets_permissions_unix() -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let path = "test_io_perms.txt";
    io(write_to_file(Path::new(path), "data", FILE_PERMS_SECRET, true))?;
    
    let meta = io(fs::metadata(path))?;
    let mode = meta.permissions().mode() & 0o777;
    assert_eq!(mode, 0o600);

    io(fs::remove_file(path))?;
    Ok(())
}
