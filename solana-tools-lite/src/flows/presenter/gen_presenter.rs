use crate::flows::presenter::Presentable;
use crate::models::results::GenResult;
use crate::utils::pretty_print_json;

impl Presentable for GenResult {
    fn present(&self, json: bool, show_secret: bool) {
        match (json, show_secret) {
            (true, true) => pretty_print_json(self),
            (false, true) => println!("{}", self),
            (false, false) | (true, false) => println!("{}", self.to_public_display()),
        }
    }
}
