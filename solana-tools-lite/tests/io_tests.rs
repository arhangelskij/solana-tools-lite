
use std::fs;
use solana_tools_lite::adapters::io_adapter::{read_input, write_output};

#[test]
fn test_read_input_from_file() -> Result<(), Box<dyn std::error::Error>> {
    // Prepare a temporary file with known content
    let path = "test_read.txt";
    let data = "Hello, IO!";
    fs::write(path, data)?;

    // Read using read_input
    let content = read_input(Some(path))?;
    assert_eq!(content, data);

    // Clean up
    fs::remove_file(path)?;
    Ok(())
}

#[test]
fn test_read_input_invalid_path() {
    // Attempt to read a nonexistent file, should return an error
    let result = read_input(Some("nonexistent_file.txt"));
    assert!(result.is_err());
}

#[test]
fn test_write_output_to_file() -> Result<(), Box<dyn std::error::Error>> {
    // Write data to a temporary file
    let path = "test_write.txt";
    let data = "Output Data";
    write_output(Some(path), data)?;

    // Verify the file content
    let content = fs::read_to_string(path)?;
    assert_eq!(content, data);

    // Clean up
    fs::remove_file(path)?;
    Ok(())
}
