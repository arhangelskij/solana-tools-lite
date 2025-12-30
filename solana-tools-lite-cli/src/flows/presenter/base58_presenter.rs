//! Presentation rules for Base58 encode/decode results.

use crate::flows::presenter::{emit_line, pretty_print_json, Presentable};
use crate::shell::error::CliError;
use solana_tools_lite::models::results::Base58Result;

impl Presentable for Base58Result {
    fn present(
        &self,
        json: bool,
        _show_secret: bool,
        to_stderr: bool,
    ) -> Result<(), CliError> {
        if json {
            pretty_print_json(self, to_stderr)?;
        } else {
            emit_line(&self.output, to_stderr);
        }
        Ok(())
    }
}
