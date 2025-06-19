use clap::Parser;
use crate::models::cmds::Commands;

#[derive(Parser, Debug)]
#[command(name = "solana-tools-lite")]
#[command(about = "Lightweight Solana CLI Toolkit", long_about = None,
// If no subcommand is supplied, show help (stdout) and exit code 0
    arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}
