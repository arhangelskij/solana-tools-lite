//! Trait for presenting flow results to stdout/stderr.

/// Presentable: implement to control how a result is shown.
/// - `json`: pretty JSON when true, otherwise plain text
/// - `show_secret`: allow printing secrets when applicable (may be ignored)
/// - `to_stderr`: emit output to stderr instead of stdout
pub trait Presentable {
    fn present(
        &self,
        json: bool,
        show_secret: bool,
        to_stderr: bool,
    ) -> Result<(), crate::shell::error::CliError>;
}
