//! Trait for presenting flow results to stdout/stderr.

/// Presentable: implement to control how a result is shown.
/// - `json`: pretty JSON when true, otherwise plain text
/// - `show_secret`: allow printing secrets when applicable (may be ignored)
pub trait Presentable {
    fn present(&self, json: bool, show_secret: bool);
}
