use clap::Parser;
use solana_tools_lite_cli::flows;
use solana_tools_lite_cli::shell::cli::Cli;
use solana_tools_lite_cli::shell::config::ConfigResolver;
use solana_tools_lite_cli::shell::error::{fail_invalid_input, report_cli_error};
use solana_tools_lite_cli::models::cmds::Commands;

fn main() {
    // Initialize protocol extensions if feature is enabled
    #[cfg(feature = "protocol-extensions")]
    extensions::init();
    
    let cli = Cli::parse();

    // Global JSON resolution
    let json = ConfigResolver::resolve_json(cli.json);

    match &cli.command {
        Commands::Gen {
            mnemonic,
            passphrase,
            unsafe_show_secret,
            output,
            force,
        } => {
            // Force save can be set via --force or ENV for consistency
            let force_resolved = ConfigResolver::resolve_force(*force);

            if let Err(e) = flows::generation::execute(
                mnemonic.as_deref(),
                passphrase.as_deref(),
                json,
                *unsafe_show_secret,
                output.as_deref(),
                force_resolved,
            ) {
                report_cli_error("gen", e);
            }
        }

        Commands::Sign {
            message,
            from_file,
            keypair,
            output,
            force,
        } => {
            let kp_path = require_keypair("sign", keypair.clone());
            let force_resolved = ConfigResolver::resolve_force(*force);

            if let Err(e) = flows::sign::execute(
                message.as_deref(),
                from_file.as_deref(),
                &kp_path,
                output.as_deref(),
                force_resolved,
                json,
            ) {
                report_cli_error("sign", e);
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
            force,
        } => {
            let force_resolved = ConfigResolver::resolve_force(*force);

            if let Err(e) = flows::verify::execute(
                message.as_deref(),
                from_file.as_deref(),
                signature.as_deref(),
                signature_file.as_deref(),
                pubkey.as_deref(),
                pubkey_file.as_deref(),
                output.as_deref(),
                force_resolved,
                json,
            ) {
                report_cli_error("verify", e);
            }
        }

        Commands::Base58 { action } => {
            if let Err(e) = flows::base58::execute(action, json) {
                report_cli_error("base58", e);
            }
        }

        Commands::SignTx {
            input,
            keypair,
            output,
            output_format,
            force,
            lookup_tables,
            assume_yes,
            max_fee,
            summary_json,
        } => {
            let kp_path = require_keypair("sign-tx", keypair.clone());

            let out_fmt = ConfigResolver::resolve_output_format(*output_format);
            let force_resolved = ConfigResolver::resolve_force(*force);
            let yes_resolved = ConfigResolver::resolve_yes(*assume_yes);
            let fee_resolved = ConfigResolver::resolve_max_fee(*max_fee);

            if let Err(e) = flows::sign_tx::execute(
                Some(input.as_str()),
                &kp_path,
                output.as_deref(),
                json,
                out_fmt,
                force_resolved,
                lookup_tables.as_deref(),
                yes_resolved,
                fee_resolved,
                *summary_json,
            ) {
                report_cli_error("sign-tx", e);
            }
        }

        Commands::Analyze {
            input,
            lookup_tables,
            pubkey,
            summary_json,
        } => {
            if let Err(e) = flows::analyze::execute(
                Some(input.as_str()),
                pubkey.as_deref(),
                lookup_tables.as_deref(),
                *summary_json,
            ) {
                report_cli_error("analyze", e);
            }
        }
    }
}

fn require_keypair(cmd_name: &str, keypair: Option<String>) -> String {
    ConfigResolver::resolve_keypair(keypair).unwrap_or_else(|| {
        fail_invalid_input(
            cmd_name,
            "keypair path is required (use --keypair or SOLANA_SIGNER_KEYPAIR env)",
        );
    })
}
