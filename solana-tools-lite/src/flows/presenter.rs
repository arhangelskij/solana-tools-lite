//! Flow result presentation via a simple trait implemented per result type.

use crate::models::results::{GenResult, SignResult, VerifyResult};
use crate::utils::pretty_print_json;

/// Presentable defines how a flow result prints itself.
/// Flows keep control over side effects (saving files, stderr routing, etc.).
pub trait Presentable {
    /// Print to stdout/stderr according to the flags.
    /// - `json`: pretty JSON when true, otherwise plain text
    /// - `show_secret`: allow printing secrets when applicable (may be ignored)
    fn present(&self, json: bool, show_secret: bool);
}

impl Presentable for VerifyResult {
    fn present(&self, json: bool, _show_secret: bool) {
        if json {
            pretty_print_json(self);
        } else if self.valid {
            println!("[✓] Signature is valid");
        } else {
            let err = self.error.as_deref().unwrap_or("unknown error");
            eprintln!("[✗] Signature is invalid: {}", err);
        }
    }
}

impl Presentable for SignResult {
    fn present(&self, json: bool, _show_secret: bool) {
        if json {
            pretty_print_json(self);
        } else {
            println!("{}", self.signature_base58);
        }
    }
}

impl Presentable for GenResult {
    fn present(&self, json: bool, show_secret: bool) {
        match (json, show_secret) {
            (true, true) => pretty_print_json(self),
            (false, true) => println!("{}", self),
            (false, false) | (true, false) => println!("{}", self.to_public_display()),
        }
    }
}
