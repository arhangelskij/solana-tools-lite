pub mod generation;
pub mod sign;
pub mod verify;
pub mod sign_tx;
pub mod presenter;
// Re-export only the trait for a cleaner public API.
pub use presenter::Presentable;
