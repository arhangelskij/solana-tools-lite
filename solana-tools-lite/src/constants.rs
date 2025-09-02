/// Permission constants
pub mod permission {
    pub const FILE_PERMS_PUBLIC: u32 = 0o644;
    pub const FILE_PERMS_SECRET: u32 = 0o600;
}

/// Default filename for generated wallet files
pub const DEFAULT_WALLET_FILENAME: &str = "wallet.json";

/// Common crypto constants
pub mod crypto {
    pub const SIG_LEN: usize = 64;
    pub const PUBKEY_LEN: usize = 32;
}
