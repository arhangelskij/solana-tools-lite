use clap::Parser;
use solana_tools_lite::handlers;
use solana_tools_lite::layers::cli::Cli;
use solana_tools_lite::models::cmds::Commands;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Gen {
            mnemonic,
            passphrase,
        } => {
            if let Err(e) =
                handlers::generate::handle_gen(mnemonic.clone(), passphrase.clone(), cli.json)
            {
                eprintln!("Error executing gen command: {e}");
                std::process::exit(1);
            }
        }

        Commands::Sign {
            message,
            secret_key,
        } => {
            if let Err(e) = handlers::sign::handle_sign(message, secret_key, cli.json) {
                eprintln!("Error executing sign command: {e}");
                std::process::exit(1);
            }
        }

        Commands::Verify {
            message,
            signature,
            pubkey,
        } => {
            let exit_code = handlers::verify::handle_verify(message, signature, pubkey, cli.json);
            std::process::exit(exit_code);
        }

        Commands::Base58 { action } => {
            if let Err(e) = handlers::base58::handle_base58(action) {
                eprintln!("Error executing base58 command: {e}");
                std::process::exit(1);
            }
        }

        Commands::SignTx {
            input,
            secret_key,
            output,
        } => {
            if let Err(e) =
                handlers::sign::handle_sign_transaction_file(input, secret_key, output, cli.json)
            {
                eprintln!("Error executing sign-tx command: {e}");
                std::process::exit(1);
            }
        }
    }
}
