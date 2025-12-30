//! Presentation rules for key generation results.

use crate::flows::presenter::{emit_line, pretty_print_json, Presentable};
use crate::shell::error::CliError;
use solana_tools_lite::models::results::GenResult;
use std::fmt;

impl Presentable for GenResult {
    fn present(
        &self,
        json: bool,
        show_secret: bool,
        to_stderr: bool,
    ) -> Result<(), CliError> {
        let display = GenDisplay {
            result: self,
            show_secret,
        };
        match (json, show_secret) {
            (true, true) => pretty_print_json(self, to_stderr)?,
            (false, true) | (false, false) | (true, false) => {
                emit_line(&display.to_string(), to_stderr)
            }
        }
        Ok(())
    }
}

struct GenDisplay<'a> {
    result: &'a GenResult,
    show_secret: bool,
}

impl<'a> fmt::Display for GenDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.show_secret {
            write!(
                f,
                "Mnemonic: {}\nPublic Key: {}\nSecret Key: {}\nSeed Hex: {}",
                self.result.mnemonic,
                self.result.public_key,
                self.result.secret_key,
                self.result.seed_hex
            )
        } else {
            write!(f, "Public Key: {}", self.result.public_key)
        }
    }
}
