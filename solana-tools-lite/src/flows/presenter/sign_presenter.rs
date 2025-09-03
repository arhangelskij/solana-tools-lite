use crate::flows::presenter::Presentable;
use crate::models::results::SignResult;
use crate::utils::pretty_print_json;

impl Presentable for SignResult {
    fn present(&self, json: bool, _show_secret: bool) {
        if json {
            pretty_print_json(self);
        } else {
            println!("{}", self.signature_base58);
        }
    }
}