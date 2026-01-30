#[cfg(test)]
mod tests {
    use clap::Parser;
    use solana_tools_lite_cli::shell::cli::Cli;
    use solana_tools_lite_cli::models::cmds::OutFmt;
    use solana_tools_lite_cli::models::cmds::{Base58Action, Commands};

    /// Test that CLI arguments correctly parse into the `Gen` variant of `Commands`.
    #[test]
    fn test_parse_gen_command() {
        let args = vec![
            "solana-lite",
            "gen",
            "--mnemonic",
            "./mnemonic.txt",
            "--passphrase",
            "pass",
            "--output",
            "./path",
        ];

        let cli = Cli::parse_from(args);
        match cli.command {
            Commands::Gen {
                mnemonic,
                passphrase,
                unsafe_show_secret,
                output,
                force,
            } => {
                assert_eq!(mnemonic.as_deref(), Some("./mnemonic.txt"));
                assert_eq!(passphrase.as_deref(), Some("pass"));
                // False by default
                assert_eq!(unsafe_show_secret, false);
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
            "--message",
            "hello",
            "--keypair",
            "./tests/fixtures/test_keypair.json",
        ];
        let cli = Cli::parse_from(args);
        match cli.command {
            Commands::Sign {
                message,
                from_file,
                keypair,
                output,
                force,
            } => {
                assert_eq!(message.as_deref(), Some("hello"));
                assert_eq!(from_file, None);
                assert_eq!(
                    keypair.as_deref(),
                    Some("./tests/fixtures/test_keypair.json")
                );
                // Defaults for optional flags
                assert_eq!(output, None);
                assert_eq!(force, false);
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
            "--from-file",
            "./path/message.txt",
            "--keypair",
            "./tests/fixtures/test_keypair.json",
        ];
        let cli = Cli::parse_from(args);
        match cli.command {
            Commands::Sign {
                message,
                from_file,
                keypair,
                output,
                force,
            } => {
                assert_eq!(message, None);
                assert_eq!(from_file.as_deref(), Some("./path/message.txt"));
                assert_eq!(
                    keypair.as_deref(),
                    Some("./tests/fixtures/test_keypair.json")
                );
                assert_eq!(output, None);
                assert_eq!(force, false);
            }
            _ => panic!("Parsed into wrong command variant"),
        }
    }

    /// Test parsing the `sign` command with output path and --force set.
    #[test]
    fn test_parse_sign_with_output_and_force() {
        let args = vec![
            "solana-lite",
            "sign",
            "--message",
            "hello",
            "--keypair",
            "./tests/fixtures/test_keypair.json",
            "--output",
            "./out/result.json",
            "--force",
        ];
        let cli = Cli::parse_from(args);
        match cli.command {
            Commands::Sign {
                message,
                from_file,
                keypair,
                output,
                force,
            } => {
                assert_eq!(message.as_deref(), Some("hello"));
                assert!(from_file.is_none());
                assert_eq!(
                    keypair.as_deref(),
                    Some("./tests/fixtures/test_keypair.json")
                );
                assert_eq!(output.as_deref(), Some("./out/result.json"));
                assert!(force);
            }
            _ => panic!("Parsed into wrong command variant"),
        }
    }

    /// Test that `sign` fails to parse when both message and from-file are provided.
    #[test]
    fn test_parse_sign_mutually_exclusive_sources() {
        let args = vec![
            "solana-lite",
            "sign",
            "--message",
            "hello",
            "--from-file",
            "msg.txt",
            "--keypair",
            "./tests/fixtures/test_keypair.json",
        ];
        let res = Cli::try_parse_from(args);
        assert!(
            res.is_err(),
            "expected clap to error when both msg sources are provided"
        );
    }

    /// Test parsing the `verify` command with inline message, signature, and pubkey.
    #[test]
    fn test_parse_verify_command_inline() {
        let args = vec![
            "solana-lite",
            "verify",
            "--message",
            "black swan",
            "--signature",
            "sig",
            "--pubkey",
            "pub",
        ];
        let cli = Cli::parse_from(args);
        match cli.command {
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
                assert_eq!(message.as_deref(), Some("black swan"));
                assert!(from_file.is_none());

                assert_eq!(signature.as_deref(), Some("sig"));
                assert!(signature_file.is_none());

                assert_eq!(pubkey.as_deref(), Some("pub"));
                assert!(pubkey_file.is_none());

                assert!(output.is_none());
                assert_eq!(force, false);
            }
            _ => panic!("Parsed into wrong command variant"),
        }

        // Additional test with --output and --force flags
        let args_with_output_force = vec![
            "solana-lite",
            "verify",
            "--message",
            "black swan",
            "--signature",
            "sig",
            "--pubkey",
            "pub",
            "--output",
            "./out.json",
            "--force",
        ];
        let cli = Cli::parse_from(args_with_output_force);
        match cli.command {
            Commands::Verify { output, force, .. } => {
                assert_eq!(output.as_deref(), Some("./out.json"));
                assert_eq!(force, true);
            }
            _ => panic!("Parsed into wrong command variant"),
        }
    }
    /// Test parsing the `verify` command with message/signature/pubkey from files.
    #[test]
    fn test_parse_verify_command_from_files() {
        let args = vec![
            "solana-lite",
            "verify",
            "--message-file",
            "./path/message.txt",
            "--signature-file",
            "./path/sig.bin",
            "--pubkey-file",
            "./path/pubkey.txt",
        ];

        let cli = Cli::parse_from(args);

        match cli.command {
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
                assert!(message.is_none());
                assert_eq!(from_file.as_deref(), Some("./path/message.txt"));

                assert!(signature.is_none());
                assert_eq!(signature_file.as_deref(), Some("./path/sig.bin"));

                assert!(pubkey.is_none());
                assert_eq!(pubkey_file.as_deref(), Some("./path/pubkey.txt"));

                assert!(output.is_none());
                assert_eq!(force, false);
            }
            _ => panic!("Parsed into wrong command variant"),
        }
    }

    /// Test that `verify` fails to parse when one of the required groups is missing.
    #[test]
    fn test_parse_verify_command_missing_group_errors() {
        // No pubkey/pubkey-file → should fail
        let args = vec![
            "solana-lite",
            "verify",
            "--message",
            "hi",
            "--signature",
            "sig",
        ];
        let res = Cli::try_parse_from(args);
        assert!(
            res.is_err(),
            "expected clap to error when pk_src is missing"
        );

        // Both options of the same group (message and message-file) → should fail (mutually exclusive)
        let args2 = vec![
            "solana-lite",
            "verify",
            "--message",
            "hi",
            "--message-file",
            "m.txt",
            "--signature",
            "sig",
            "--pubkey",
            "pub",
        ];
        let res2 = Cli::try_parse_from(args2);
        assert!(
            res2.is_err(),
            "expected clap to error when both msg_src options are present"
        );
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
    /// Test parsing the `sign-tx` command with all options provided.
    #[test]
    fn test_parse_sign_tx_full() {
        let args = vec![
            "solana-lite",
            "--json", // global JSON flag
            "sign-tx",
            "--input",
            "in.json",
            "--keypair",
            "wallet.json",
            "--output",
            "out.json",
            "--output-format",
            "base64", // explicit output format
            "--force",
        ];
        let cli = Cli::parse_from(args);
        assert!(cli.json, "global --json should be set");
        match cli.command {
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
                assert_eq!(input, "in.json");
                assert_eq!(keypair.as_deref(), Some("wallet.json"));
                assert_eq!(output.as_deref(), Some("out.json"));
                assert!(matches!(output_format, Some(OutFmt::Base64)));
                assert_eq!(output.as_deref(), Some("out.json"));
                assert!(force);
                assert!(lookup_tables.is_none());
                assert!(!assume_yes);
                assert!(max_fee.is_none());
                assert!(!summary_json);
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
                output_format,
                force,
                lookup_tables,
                assume_yes,
                max_fee,
                summary_json,
            } => {
                assert_eq!(input, "in.json");
                assert_eq!(keypair.as_deref(), Some("wallet.json"));
                assert_eq!(output, None);
                assert_eq!(force, false);
                assert!(lookup_tables.is_none());
                assert_eq!(assume_yes, false);
                assert!(max_fee.is_none());
                assert!(!summary_json);

                assert!(matches!(output_format, None));
            }
            _ => panic!("Parsed into wrong command variant"),
        }
    }

    /// Test parsing the `sign-tx` command with summary-json flag.
    #[test]
    fn test_parse_sign_tx_summary_json_flag() {
        let args = vec![
            "solana-lite",
            "sign-tx",
            "--input",
            "in.json",
            "--keypair",
            "wallet.json",
            "--summary-json",
            "--output",
            "out.json",
        ];
        let cli = Cli::parse_from(args);
        match cli.command {
            Commands::SignTx {
                summary_json,
                output,
                ..
            } => {
                assert!(summary_json);
                assert_eq!(output.as_deref(), Some("out.json"));
            }
            _ => panic!("Parsed into wrong command variant"),
        }
    }

    /// Test parsing the `analyze` command with all options.
    #[test]
    fn test_parse_analyze_command() {
        let args = vec![
            "solana-lite",
            "analyze",
            "--input",
            "tx.json",
            "--pubkey",
            "Author1111111111111111111111111111111111111",
            "--tables",
            "luts.json",
            "--summary-json",
        ];
        let cli = Cli::parse_from(args);
        match cli.command {
            Commands::Analyze {
                input,
                pubkey,
                lookup_tables,
                summary_json,
            } => {
                assert_eq!(input, "tx.json");
                assert_eq!(
                    pubkey.as_deref(),
                    Some("Author1111111111111111111111111111111111111")
                );
                assert_eq!(lookup_tables.as_deref(), Some("luts.json"));
                assert!(summary_json);
            }
            _ => panic!("Parsed into wrong command variant"),
        }
    }

}
