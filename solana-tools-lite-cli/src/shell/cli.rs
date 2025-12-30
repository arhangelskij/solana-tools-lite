use crate::models::cmds::Commands;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "solana-tools-lite")]
#[command(about = "Lightweight Solana CLI Toolkit", long_about = None,
// If no subcommand is supplied, show help (stdout) and exit code 0
    arg_required_else_help = true)]
#[command(infer_long_args = true)]
pub struct Cli {
    #[arg(
        global = true,
        long = "json",
        help = "Output as JSON (pretty) [env: SOLANA_TOOLS_LITE_JSON]"
    )]
    pub json: bool,
    #[command(subcommand)]
    pub command: Commands,
}
