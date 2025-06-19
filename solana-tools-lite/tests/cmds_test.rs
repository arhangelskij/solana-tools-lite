#[cfg(test)]
mod tests {
    use clap::Parser;
    use solana_tools_lite::layers::cli::Cli;
    use solana_tools_lite::models::cmds::{Commands, Base58Action};

    /// Test that CLI arguments correctly parse into the `Gen` variant of `Commands`.
    #[test]
    fn test_parse_gen_command() {
        let args = vec![
            "solana-lite",
            "gen",
            "--mnemonic",
            "test test test",
            "--passphrase",
            "pass",
            "--explain",
        ];

        let cli = Cli::parse_from(args);
        match cli.command {
            Commands::Gen {
                mnemonic,
                passphrase,
                explain,
            } => {
                assert_eq!(mnemonic.as_deref(), Some("test test test"));
                assert_eq!(passphrase.as_deref(), Some("pass"));
                assert!(explain);
            }
            _ => panic!("Parsed into wrong command variant"),
        }
    }

    /// Test parsing the `sign` command with message and optional mnemonic.
    #[test]
    fn test_parse_sign_command() {
        let args = vec![
            "solana-lite",
            "sign",
            "--message",
            "hello",
            "--secret-key",
            "4f3edf983ac636a65a842ce7c78d9aa706d3b113b5ad2efc73362be3dfc1ad7a",
        ];
        let cli = Cli::parse_from(args);
        match cli.command {
            Commands::Sign {
                message,
                secret_key,
            } => {
                assert_eq!(message, "hello");
                assert_eq!(secret_key.as_deref(), Some("4f3edf983ac636a65a842ce7c78d9aa706d3b113b5ad2efc73362be3dfc1ad7a"));
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
            "--message",
            "hello",
            "--signature",
            "sig",
            "--pubkey",
            "pub",
        ];
        let cli = Cli::parse_from(args);
        match cli.command {
            Commands::Verify {
                message,
                signature,
                pubkey,
            } => {
                assert_eq!(message, "hello");
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
}
