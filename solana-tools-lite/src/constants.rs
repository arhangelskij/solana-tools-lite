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

    const SYSTEM_PROGRAM_ID: &str = "11111111111111111111111111111111";
    const COMPUTE_BUDGET_ID: &str = "ComputeBudget111111111111111111111111111111";
    const TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
    const TOKEN_2022_PROGRAM_ID: &str = "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb";

    static SYSTEM_PROGRAM: OnceLock<PubkeyBase58> = OnceLock::new();
    static COMPUTE_BUDGET_PROGRAM: OnceLock<PubkeyBase58> = OnceLock::new();
    static TOKEN_PROGRAM: OnceLock<PubkeyBase58> = OnceLock::new();
    static TOKEN_2022_PROGRAM: OnceLock<PubkeyBase58> = OnceLock::new();

    pub fn system_program() -> &'static PubkeyBase58 {
        SYSTEM_PROGRAM.get_or_init(|| {
            PubkeyBase58::try_from(SYSTEM_PROGRAM_ID)
                // Safe to unwrap: SYSTEM_PROGRAM_ID is a compile-time constant known to be valid base58.
                // If this panics, it indicates a critical bug in the constant definition.
                .expect("SYSTEM_PROGRAM_ID must be valid base58 pubkey")
        })
    }

    pub fn compute_budget_program() -> &'static PubkeyBase58 {
        COMPUTE_BUDGET_PROGRAM.get_or_init(|| {
            PubkeyBase58::try_from(COMPUTE_BUDGET_ID)
                .expect("COMPUTE_BUDGET_ID must be valid base58 pubkey")
        })
    }

    pub fn token_program() -> &'static PubkeyBase58 {
        TOKEN_PROGRAM.get_or_init(|| {
            PubkeyBase58::try_from(TOKEN_PROGRAM_ID)
                .expect("TOKEN_PROGRAM_ID must be valid base58 pubkey")
        })
    }

    pub fn token_2022_program() -> &'static PubkeyBase58 {
        TOKEN_2022_PROGRAM.get_or_init(|| {
            PubkeyBase58::try_from(TOKEN_2022_PROGRAM_ID)
                .expect("TOKEN_2022_PROGRAM_ID must be valid base58 pubkey")
        })
    }
}
