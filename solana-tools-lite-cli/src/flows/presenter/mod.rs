pub(crate) mod presenter_trait;
pub use presenter_trait::Presentable;

mod base58_presenter;
mod gen_presenter;
mod sign_presenter;
pub mod sign_tx_presenter; //TODO: 🟡 pub for tests!
mod verify_presenter;
mod utils;

pub use sign_tx_presenter::SignTxPresentation;
pub(crate) use utils::{emit_line, pretty_print_json};
