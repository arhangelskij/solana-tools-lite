use clap::{ArgGroup, Subcommand};

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
    #[command(group(ArgGroup::new("data_source").required(true).args(["message", "from_file"])))]
    Sign {
        /// Message to sign (inline)
        #[arg(short, long, group = "data_source")]
        message: Option<String>,

        /// Read message from file or stdin ("-")
        #[arg(long = "from-file", value_name = "FILE", group = "data_source")]
        from_file: Option<String>,

        /// Path to keypair file (stdin disabled for secrets)
        #[arg(long, short = 'k')]
        keypair: String //TODO: 🟡 think additionally about name 
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
        #[arg(long = "from-file", value_name = "FILE", group = "msg_src")]
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

        /// Force save(override) a file
        #[arg(long, short, default_value = "false")]
        force: bool,
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

        //TODO: 🔴 delete comment after checking command Base58-encoded private key (32 bytes)
        /// Path to keypair file (stdin disabled for secrets) 
        #[arg(long, short = 'k')]
        keypair: String,

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
        input: String
    },
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum OutFmt {
    Json,
    Base64,
    Base58
}