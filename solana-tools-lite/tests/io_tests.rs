
use std::fs;
use std::path::Path;
use solana_tools_lite::layers::io::{read_from_file, write_to_file};

#[test]
fn test_read_from_file_ok() -> Result<(), Box<dyn std::error::Error>> {
    // Prepare a temporary file with known content
    let path = "test_read.txt";
    let data = "Hello, IO!";
    fs::write(path, data)?;

    // Read using low-level IO layer
    let content = read_from_file(Path::new(path))?;
    assert_eq!(content, data);

    // Clean up
    fs::remove_file(path)?;
    Ok(())
}

#[test]
fn test_read_input_invalid_path() {
    // Attempt to read a nonexistent file via IO layer, should return an error
    let result = read_from_file(Path::new("nonexistent_file.txt"));
    assert!(result.is_err());
}

#[test]
fn test_write_to_file_ok() -> Result<(), Box<dyn std::error::Error>> {
    // Write data to a temporary file
    let path = "test_write.txt";
    let data = "Output Data";
    write_to_file(Path::new(path), data, 0o644, true)?;

    // Verify the file content
    let content = fs::read_to_string(path)?;
    assert_eq!(content, data);

    // Clean up
    fs::remove_file(path)?;
    Ok(())
}

#[test]
fn test_write_to_file_no_force_should_fail_on_existing() -> Result<(), Box<dyn std::error::Error>> {
    let path = "test_write_exists.txt";
    fs::write(path, "initial")?;

    let err = write_to_file(Path::new(path), "new", 0o644, false)
        .expect_err("expected AlreadyExists error when force=false");
    assert_eq!(err.kind(), std::io::ErrorKind::AlreadyExists);

    fs::remove_file(path)?;
    Ok(())
}

#[test]
fn test_read_from_file_directory_should_error() -> Result<(), Box<dyn std::error::Error>> {
    let dir = "test_io_dir_read";
    fs::create_dir(dir)?;
    let res = read_from_file(Path::new(dir));
    
    assert!(res.is_err());
    fs::remove_dir(dir)?;
    Ok(())
}

#[test]
fn test_write_to_file_directory_should_error() -> Result<(), Box<dyn std::error::Error>> {
    let dir = "test_io_dir_write";
    fs::create_dir(dir)?;
    
    let res = write_to_file(Path::new(dir), "data", 0o644, true);
    assert!(res.is_err());
    
    fs::remove_dir(dir)?;
    Ok(())
}

#[cfg(unix)]
#[test]
fn test_write_to_file_sets_permissions_unix() -> Result<(), Box<dyn std::error::Error>> {
    use std::os::unix::fs::PermissionsExt;
    let path = "test_io_perms.txt";
    write_to_file(Path::new(path), "data", 0o600, true)?;

    let meta = fs::metadata(path)?;
    let mode = meta.permissions().mode() & 0o777;
    assert_eq!(mode, 0o600);

    fs::remove_file(path)?;
    Ok(())
}
