use clap::{Parser};
use crate::models::cmds::Commands;

#[derive(Parser, Debug)]
#[command(name = "solana-tools-lite")]
#[command(about = "Lightweight Solana CLI Toolkit", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}