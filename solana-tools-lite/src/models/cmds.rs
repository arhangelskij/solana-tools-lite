use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate keypair from mnemonic
    Gen {
        /// Read mnemonic from file or stdin ("-"). If omitted, a new mnemonic is generated.
        #[arg(long, value_name = "FILE")]
        mnemonic: Option<String>,
        /// Read passphrase from file or stdin ("-"). Optional.
        #[arg(long, value_name = "FILE")]
        passphrase: Option<String>,
        /// Show secret in output
        #[arg(long, default_value = "false")]
        show_secret: bool, //TODO: 🟡 rename to unsafe_show_secret
        /// Output path for a wallet
         #[arg(long, short)]
        output: Option<String>,
        /// Force save(override) a wallet file
        #[arg(long, short, default_value = "false")]
        force: bool
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
    },

    /// Sign a transaction JSON file (cold-signer)
    SignTx {
        /// Path to input JSON file
        #[arg(long, short = 'i')]
        input: String,

        /// Base58-encoded private key (32 bytes)
        #[arg(long, short = 'k')]
        secret_key: String,

        /// Optional output file (if not set, print to stdout)
        #[arg(long, short = 'o')]
        output: Option<String>,

        /// Force output format (json|base64|base58). If not specified, we mirror the input format.
        #[arg(long = "output-format", value_enum, short = 'f')]
        output_format: Option<OutFmt>
    },
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
    },
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum OutFmt {
    Json,
    Base64,
    Base58,
}

//TODO: 🟡 add also base64?
