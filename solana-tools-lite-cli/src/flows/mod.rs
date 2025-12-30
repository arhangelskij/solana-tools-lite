pub mod base58;
pub mod generation;
pub mod presenter;
pub mod sign;
pub mod sign_tx;
pub mod verify;
// Re-export only the trait for a cleaner public API.
pub use presenter::Presentable;
