use clap::{Subcommand};

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate keypair from mnemonic
    Gen {
        #[arg(short, long)]
        mnemonic: Option<String>,

        #[arg(long)]
        passphrase: Option<String>,

        #[arg(long, default_value = "false")]
        explain: bool
    },

    /// Sign a message
    Sign {
        #[arg(short, long)]
        message: String,

        /// Base58-encoded private key (32 bytes)
        #[arg(long)]
        secret_key: String
    },

    /// Verify a signature
    Verify {
        #[arg(short, long)]
        message: String,

        #[arg(short, long)]
        signature: String,

        #[arg(long)]
        pubkey: String
    },

    /// Base58 encode/decode
    Base58 {
        #[command(subcommand)]
        action: Base58Action
    }
}

#[derive(Subcommand, Debug)]
pub enum Base58Action {
    Encode {
        #[arg(short, long)]
        input: String,
    },
    Decode {
        #[arg(short, long)]
        input: String,
    }
}
