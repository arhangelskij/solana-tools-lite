use clap::{ArgGroup, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate a new mnemonic and keypair, or derive from existing mnemonic
    Gen {
        /// Read mnemonic from file or stdin ("-"). If omitted, a new mnemonic is generated.
        #[arg(long, value_name = "FILE")]
        mnemonic: Option<String>,
        /// Read passphrase from file or stdin ("-"). Optional.
        #[arg(long, value_name = "FILE")]
        passphrase: Option<String>,
        /// UNSAFE: print secret to stdout
        #[arg(long = "unsafe-show-secret", default_value = "false")]
        unsafe_show_secret: bool,
        /// Output path for a wallet
        #[arg(long, short)]
        output: Option<String>,
        /// Force save(override) a wallet file [env: SOLANA_TOOLS_LITE_FORCE]
        #[arg(long, short, default_value = "false")]
        force: bool,
    },

    /// Sign a message
    #[command(group(ArgGroup::new("data_source").required(true).args(["message", "from_file"])))]
    Sign {
        /// Message to sign (inline)
        #[arg(short, long, group = "data_source")]
        message: Option<String>,

        /// Read message from file or stdin ("-")
        #[arg(long = "from-file", value_name = "FILE", group = "data_source")]
        from_file: Option<String>,

        /// Path to keypair file (stdin disabled for secrets) [env: SOLANA_SIGNER_KEYPAIR]
        #[arg(long, short = 'k')]
        keypair: Option<String>,

        /// Optional output file (if not set, print to stdout)
        #[arg(long, short)]
        output: Option<String>,

        /// Force save(override) a file [env: SOLANA_TOOLS_LITE_FORCE]
        #[arg(long, short, default_value = "false")]
        force: bool,
    },

    /// Verify a signature
    #[command(group(ArgGroup::new("msg_src").required(true).args(["message", "from_file"])))]
    #[command(group(ArgGroup::new("sig_src").required(true).args(["signature", "signature_file"])))]
    #[command(group(ArgGroup::new("pk_src").required(true).args(["pubkey", "pubkey_file"])))]
    Verify {
        /// Message to verify (inline)
        #[arg(short, long, group = "msg_src")]
        message: Option<String>,

        /// Read message from file or stdin ("-")
        /// Accepts both `--from-file` and `--message-file` for convenience.
        #[arg(
            long = "from-file",
            alias = "message-file",
            value_name = "FILE",
            group = "msg_src"
        )]
        from_file: Option<String>,

        /// Signature to verify (Base58, inline)
        #[arg(short, long, group = "sig_src")]
        signature: Option<String>,

        /// Read signature from file or stdin ("-")
        #[arg(long = "signature-file", value_name = "FILE", group = "sig_src")]
        signature_file: Option<String>,

        /// Public key (Base58, inline)
        #[arg(long, group = "pk_src")]
        pubkey: Option<String>,

        /// Read public key from file or stdin ("-")
        #[arg(long = "pubkey-file", value_name = "FILE", group = "pk_src")]
        pubkey_file: Option<String>,

        /// Optional output file (if not set, print to stdout)
        #[arg(long, short = 'o')]
        output: Option<String>,

        /// Force save(override) a file [env: SOLANA_TOOLS_LITE_FORCE]
        #[arg(long, short, default_value = "false")]
        force: bool,
    },

    /// Base58 encode/decode
    Base58 {
        #[command(subcommand)]
        action: Base58Action,
    },

    /// Sign a transaction file (JSON/Base64/Base58)
    SignTx {
        /// Path to input transaction (UI JSON/Base64/Base58)
        #[arg(long, short = 'i')]
        input: String,

        /// Optional lookup table file (JSON map: table address -> array of addresses)
        #[arg(long = "tables", value_name = "FILE")]
        lookup_tables: Option<String>,

        /// Path to keypair file (stdin disabled for secrets) [env: SOLANA_SIGNER_KEYPAIR]
        #[arg(long, short = 'k')]
        keypair: Option<String>,

        /// Optional output file (if not set, print to stdout)
        #[arg(long, short = 'o')]
        output: Option<String>,

        /// Force output format (json|base64|base58). If not specified, we mirror the input format. [env: SOLANA_TOOLS_LITE_OUTPUT_FORMAT]
        #[arg(long = "output-format", value_enum)]
        output_format: Option<OutFmt>,

        /// Force save(override) the output file when it exists [env: SOLANA_TOOLS_LITE_FORCE]
        #[arg(long, short, default_value = "false")]
        force: bool,

        /// Auto-approve without interactive prompt (useful for CI/pipelines) [env: SOLANA_TOOLS_LITE_YES]
        #[arg(long = "yes", short = 'y', action = clap::ArgAction::SetTrue)]
        assume_yes: bool,

        /// Fail if total fee (base + priority) exceeds this limit (lamports) [env: SOLANA_TOOLS_LITE_MAX_FEE]
        #[arg(long = "max-fee", value_name = "LAMPORTS")]
        max_fee: Option<u64>,

        /// Emit signing summary as JSON to stdout (requires --output for signed tx)
        #[arg(long = "summary-json", default_value = "false")]
        summary_json: bool,
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
