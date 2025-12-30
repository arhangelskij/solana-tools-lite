//! Presentation rules for message signing results.

use crate::flows::presenter::{emit_line, pretty_print_json, Presentable};
use crate::shell::error::CliError;
use solana_tools_lite::models::results::SignResult;

impl Presentable for SignResult {
    fn present(
        &self,
        json: bool,
        _show_secret: bool,
        to_stderr: bool,
    ) -> Result<(), CliError> {
        if json {
            pretty_print_json(self, to_stderr)?;
        } else {
            emit_line(&self.signature_base58, to_stderr);
        }
        Ok(())
    }
}
