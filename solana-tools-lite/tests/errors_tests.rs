use solana_tools_lite::errors::{AsExitCode, ExitCode, IoError, ToolError};
use std::io;

#[test]
fn io_error_not_found_maps_to_no_input() {
    let err = IoError::with_path(io::Error::new(io::ErrorKind::NotFound, "missing"), "file");
    assert_eq!(err.as_exit_code(), ExitCode::NoInput.as_i32());
}

#[test]
fn tool_error_invalid_input_maps_to_usage() {
    let err = ToolError::InvalidInput("bad".into());
    assert_eq!(err.as_exit_code(), ExitCode::Usage.as_i32());
}

#[test]
fn tool_error_io_bubbles_exit_code() {
    let err = ToolError::Io(IoError::stdio(io::Error::new(
        io::ErrorKind::PermissionDenied,
        "denied",
    )));
    assert_eq!(err.as_exit_code(), ExitCode::IoErr.as_i32());
}

#[test]
fn io_error_display_includes_path() {
    let err = IoError::with_path(io::Error::new(io::ErrorKind::NotFound, "missing"), "foo.txt");
    let text = err.to_string();
    assert!(text.contains("io(foo.txt:"));
}
