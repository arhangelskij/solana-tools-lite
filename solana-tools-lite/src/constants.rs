/// Permission constants
pub mod permission {
    pub const FILE_PERMS_PUBLIC: u32 = 0o644;
    pub const FILE_PERMS_SECRET: u32 = 0o600;
}

pub const LAMPORTS_PER_SOL: f64 = 1_000_000_000.0;

/// Common crypto constants
pub mod crypto {
    /// Ed25519 signature length in bytes.
    pub const SIG_LEN: usize = 64;
    /// Solana public key length in bytes.
    pub const PUBKEY_LEN: usize = 32;
    /// Seed length in bytes for keypair derivation input.
    pub const SEED_LEN: usize = 32;
}

pub mod compute_budget {
    pub const DEFAULT_COMPUTE_UNIT_LIMIT: u32 = 200_000;
}

pub mod programs {
    use crate::models::pubkey_base58::PubkeyBase58;
    use std::sync::OnceLock;

    pub const SYSTEM_PROGRAM_ID: &str = "11111111111111111111111111111111";
    pub const COMPUTE_BUDGET_ID: &str = "ComputeBudget111111111111111111111111111111";
    pub const TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
    pub const TOKEN_2022_PROGRAM_ID: &str = "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb";
}
