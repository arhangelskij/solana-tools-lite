use clap::Parser;
use solana_tools_lite::handlers;
use solana_tools_lite::layers::cli::Cli;
use solana_tools_lite::models::cmds::Commands;
use solana_tools_lite::flows;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Gen {
            mnemonic,
            passphrase,
            show_secret,
            output, 
            force 
        } => {
            // Resolve optional refs for handler
            let mnemonic_path = mnemonic.as_ref();
            let passphrase_path = passphrase.as_ref();
            let output_path = output.as_deref();

            // Call domain handler and handle errors early
            let result = handlers::generate::execute(mnemonic_path, passphrase_path)
                .unwrap_or_else(|e| {
                    eprintln!("Error executing gen command: {e}");
                    std::process::exit(1);
                });

            // Present the result and save wallet file.
            // If presenter fails we exit with error.
            if let Err(e) = flows::generation::execute(
                &result,
                cli.json_pretty,
                *show_secret,
                output_path,
                *force
            ) {
                eprintln!("Flow error: {e}");
                std::process::exit(1);
            }
        }

        Commands::Sign {
            message,
            secret_key,
        } => {
            if let Err(e) = handlers::sign::handle_sign(message, secret_key) {
                eprintln!("Error executing sign command: {e}");
                std::process::exit(1);
            }
        }

        Commands::Verify {
            message,
            signature,
            pubkey,
        } => {
            let exit_code =
                handlers::verify::handle_verify(message, signature, pubkey, cli.json_pretty);
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
            output_format,
        } => {
            if let Err(e) = handlers::sign_tx::handle_sign_transaction_file(
                Some(&input.clone()),
                secret_key,
                output.as_ref(),
                cli.json_pretty,
                *output_format,
            ) {
                eprintln!("Error executing sign-tx command: {e}");
                std::process::exit(1);
            }
        }
    }
}
