use clap::Parser;
use solana_tools_lite::flows;
use solana_tools_lite::handlers;
use solana_tools_lite::layers::cli::Cli;
use solana_tools_lite::models::cmds::Commands;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Gen {
            mnemonic,
            passphrase,
            show_secret,
            output,
            force,
        } => {
            // Resolve optional refs and delegate to the flow only
            let mnemonic_path = mnemonic.as_deref();
            let passphrase_path = passphrase.as_deref();
            let output_path = output.as_deref();

            if let Err(e) = flows::generation::execute(
                mnemonic_path,
                passphrase_path,
                cli.json_pretty,
                *show_secret,
                output_path,
                *force,
            ) {
                eprintln!("Flow error: {e}");
                std::process::exit(1);
            }
        }

        Commands::Sign {
            message,
            from_file,
            keypair,
            output,
            force,
        } => {
            if let Err(e) = flows::sign::execute(
                message.as_deref(),
                from_file.as_deref(),
                keypair,
                output.as_deref(),
                *force,
                cli.json_pretty,
            ) {
                eprintln!("Sign flow error: {e}");
                std::process::exit(1);
            }
        }

        Commands::Verify {
            message,
            from_file,
            signature,
            signature_file,
            pubkey,
            pubkey_file,
            output,
            force
        } => {
            if let Err(e) = flows::verify::execute(
                message.as_deref(),
                from_file.as_deref(),
                signature.as_deref(),
                signature_file.as_deref(),
                pubkey.as_deref(),
                pubkey_file.as_deref(),
                output.as_deref(),
                *force,
                cli.json_pretty,
            ) {
                eprintln!("Flow error: {e}");
                std::process::exit(1);
            }
        }

        Commands::Base58 { action } => {
            if let Err(e) = handlers::base58::handle_base58(action) {
                eprintln!("Error executing base58 command: {e}");
                std::process::exit(1);
            }
        }

        Commands::SignTx {
            input,
            keypair,
            output,
            output_format,
        } => {
            if let Err(e) = flows::sign_tx::execute(
                Some(input.as_str()),
                keypair,
                output.as_deref(),
                cli.json_pretty,
                *output_format,
            ) {
                eprintln!("Error executing sign-tx command: {e}");
                std::process::exit(1);
            }
        }
    }
}

