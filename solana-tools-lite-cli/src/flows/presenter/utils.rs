use serde::Serialize;
use crate::shell::error::CliError;

/// Pretty-prints any serializable struct as JSON.
pub(crate) fn pretty_print_json<T: Serialize>(
    value: &T,
    to_stderr: bool,
) -> Result<(), CliError> {
    let output = serde_json::to_string_pretty(value)
        .map_err(|err| CliError::PresentationEncode(err.to_string()))?;
    emit_line(&output, to_stderr);
    
    Ok(())
}

pub(crate) fn emit_line(line: &str, to_stderr: bool) {
    if to_stderr {
        eprintln!("{line}");
    } else {
        println!("{line}");
    }
}
