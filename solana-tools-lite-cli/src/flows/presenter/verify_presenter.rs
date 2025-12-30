//! Presentation rules for signature verification results.

use crate::flows::presenter::{Presentable, emit_line, pretty_print_json};
use crate::shell::error::CliError;
use solana_tools_lite::models::results::VerifyResult;

impl Presentable for VerifyResult {
    fn present(
        &self,
        json: bool,
        _show_secret: bool,
        to_stderr: bool,
    ) -> Result<(), CliError> {
        if json {
            return pretty_print_json(self, to_stderr);
        }
        emit_line("[✓] Signature is valid", to_stderr);

        Ok(())
    }
}
