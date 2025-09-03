use crate::flows::presenter::Presentable;
use crate::models::results::VerifyResult;
use crate::utils::pretty_print_json;

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
