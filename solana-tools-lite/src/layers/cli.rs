use clap::Parser;
use crate::models::cmds::Commands;

#[derive(Parser, Debug)]
#[command(name = "solana-tools-lite")]
#[command(about = "Lightweight Solana CLI Toolkit", long_about = None,
// If no subcommand is supplied, show help (stdout) and exit code 0
    arg_required_else_help = true)]
#[command(infer_long_args = true)]
pub struct Cli {
    #[arg(global = true, long, help = "Output as JSON")]
    pub json_pretty: bool,
    #[command(subcommand)]
    pub command: Commands
}