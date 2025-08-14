#[cfg(test)]
mod tests {
    use clap::Parser;
    use solana_tools_lite::layers::cli::Cli;
    use solana_tools_lite::models::cmds::{Commands, Base58Action};
    use solana_tools_lite::models::cmds::OutFmt;

    /// Test that CLI arguments correctly parse into the `Gen` variant of `Commands`.
    #[test]
    fn test_parse_gen_command() {
        let args = vec![
            "solana-lite",
            "gen",
            "--mnemonic", "test test test", //TODO: 🔴 mnemonic from file, but for test mb can use this variant
            "--passphrase", "pass",
            "--output", "./path"
        ];

        let cli = Cli::parse_from(args);
        match cli.command {
            Commands::Gen {
                mnemonic,
                passphrase,
                show_secret,
                output,
                force
            } => {
                assert_eq!(mnemonic.as_deref(), Some("test test test"));
                assert_eq!(passphrase.as_deref(), Some("pass"));
                // False by default
                assert_eq!(show_secret, false);
                assert_eq!(output.as_deref(), Some("./path"));
                // False by default
                assert_eq!(force, false);
            }
            _ => panic!("Parsed into wrong command variant"),
        }
    }

    /// Test parsing the `sign` command with message and optional mnemonic.
    #[test]
    fn test_parse_sign_message_command() {
        let args = vec![
            "solana-lite",
            "sign",
            "--message", "hello",
            "--keypair", "./tests/fixtures/test_keypair.json",
        ];
        let cli = Cli::parse_from(args);
        match cli.command {
            Commands::Sign {
                message,
                from_file,
                keypair,
            } => {
                assert_eq!(message.as_deref(), Some("hello"));
                assert_eq!(from_file, None);
                assert_eq!(keypair, "./tests/fixtures/test_keypair.json");
            }
            _ => panic!("Parsed into wrong command variant"),
        }
    }

    /// Test parsing the `sign` command with message and optional mnemonic.
    #[test]
    fn test_parse_sign_from_file_command() {
        let args = vec![
            "solana-lite",
            "sign",
            "--from-file", "./path/message.txt",
            "--keypair", "./tests/fixtures/test_keypair.json",
        ];
        let cli = Cli::parse_from(args);
        match cli.command {
            Commands::Sign {
                message,
                from_file,
                keypair
            } => {
                assert_eq!(message, None);
                assert_eq!(from_file.as_deref(), Some("./path/message.txt"));
                assert_eq!(keypair, "./tests/fixtures/test_keypair.json");
            }
            _ => panic!("Parsed into wrong command variant"),
        }
    }

    /// Test parsing the `verify` command with message, signature, and public key.
    #[test]
    fn test_parse_verify_command() {
        let args = vec![
            "solana-lite",
            "verify",
            "--message", "black swan",
            "--signature", "sig",
            "--pubkey", "pub" //TODO: 🟡 check pubkey 'coz could be from also file 
        ];
        let cli = Cli::parse_from(args);
        match cli.command {
            Commands::Verify {
                message,
                signature,
                pubkey,
            } => {
                assert_eq!(message, "black swan");
                assert_eq!(signature, "sig");
                assert_eq!(pubkey, "pub");
            }
            _ => panic!("Parsed into wrong command variant"),
        }
    }

    /// Test parsing the `base58 encode` subcommand with input.
    #[test]
    fn test_parse_base58_encode_command() {
        let args = vec!["solana-lite", "base58", "encode", "--input", "deadbeef"];
        let cli = Cli::parse_from(args);
        match cli.command {
            Commands::Base58 { action } => match action {
                Base58Action::Encode { input } => {
                    assert_eq!(input, "deadbeef");
                }
                _ => panic!("Expected Base58Action::Encode"),
            },
            _ => panic!("Parsed into wrong command variant"),
        }
    }

    /// Test parsing the `base58 decode` subcommand with input.
    #[test]
    fn test_parse_base58_decode_command() {
        let args = vec!["solana-lite", "base58", "decode", "--input", "cafebabe"];
        let cli = Cli::parse_from(args);
        
        match cli.command {
            Commands::Base58 { action } => match action {
                Base58Action::Decode { input } => {
                    assert_eq!(input, "cafebabe");
                }
                _ => panic!("Expected Base58Action::Decode"),
            },
            _ => panic!("Parsed into wrong command variant"),
        }
    }
       //TODO: check if we have all commands are tested

    /// Test parsing the `sign-tx` command with all options provided.
    #[test]

    fn test_parse_sign_tx_full() {
        let args = vec![
            "solana-lite",
            "--json-pretty",          // global pretty flag
            "sign-tx",
            "--input", "in.json",
            "--keypair", "wallet.json",
            "--output", "out.json",
            "--output-format", "base64",      // explicit output format
        ];
        let cli = Cli::parse_from(args);
        assert!(cli.json_pretty, "global --json-pretty should be set");
        match cli.command {
            Commands::SignTx { input, keypair, output, output_format } => {
                assert_eq!(input, "in.json");
                assert_eq!(keypair, "wallet.json");
                assert_eq!(output.as_deref(), Some("out.json"));
                assert!(matches!(output_format, Some(OutFmt::Base64)));
            }
            _ => panic!("Parsed into wrong command variant"),
        }
    }

    /// Test parsing the `sign-tx` command with only required options.
    #[test]
    fn test_parse_sign_tx_minimal() {
        let args = vec![
            "solana-lite",
            "sign-tx",
            "--input",
            "in.json",
            "--keypair",
            "wallet.json",
        ];
        let cli = Cli::parse_from(args);
        match cli.command {
            Commands::SignTx {
                input,
                keypair,
                output,
                output_format
            } => {
                assert_eq!(input, "in.json");
                assert_eq!(keypair, "wallet.json");
                assert_eq!(output, None);

                assert!(matches!(output_format, None));
            }
            _ => panic!("Parsed into wrong command variant"),
        }
    }
}