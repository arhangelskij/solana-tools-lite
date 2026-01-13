use solana_tools_lite::constants::programs;
use solana_tools_lite::extensions::light_protocol::constants::{
    DISCRIMINATOR_COMPRESS_SOL, DISCRIMINATOR_CREATE_MINT, DISCRIMINATOR_MINT_TO,
    DISCRIMINATOR_TRANSFER,
};
use solana_tools_lite::extensions::light_protocol::LightProtocol;
use solana_tools_lite::extensions::ProtocolAnalyzer;
use solana_tools_lite::models::extensions::{ExtensionAction, LightProtocolAction};
use solana_tools_lite::models::pubkey_base58::PubkeyBase58;

#[test]
fn test_detect_create_mint() {
    let analyzer = LightProtocol;
    let program_id = PubkeyBase58::try_from("cTokenmWW8bLPjZEBAUgYy3zKxQZW6VKi7bqNFEVv3m").unwrap();
    let data = DISCRIMINATOR_CREATE_MINT.to_vec();

    let result = analyzer.analyze(&program_id, &data);
    assert!(matches!(
        result,
        Some(ExtensionAction::LightProtocol(LightProtocolAction::CreateMint))
    ));
}

#[test]
fn test_detect_mint_to() {
    let analyzer = LightProtocol;
    let program_id = PubkeyBase58::try_from("cTokenmWW8bLPjZEBAUgYy3zKxQZW6VKi7bqNFEVv3m").unwrap();
    let data = DISCRIMINATOR_MINT_TO.to_vec();

    let result = analyzer.analyze(&program_id, &data);
    assert!(matches!(
        result,
        Some(ExtensionAction::LightProtocol(LightProtocolAction::MintTo))
    ));
}

#[test]
fn test_detect_transfer() {
    let analyzer = LightProtocol;
    let program_id = PubkeyBase58::try_from("cTokenmWW8bLPjZEBAUgYy3zKxQZW6VKi7bqNFEVv3m").unwrap();
    let data = DISCRIMINATOR_TRANSFER.to_vec();

    let result = analyzer.analyze(&program_id, &data);
    assert!(matches!(
        result,
        Some(ExtensionAction::LightProtocol(LightProtocolAction::Transfer))
    ));
}

#[test]
fn test_detect_compress_sol() {
    let analyzer = LightProtocol;
    let program_id = PubkeyBase58::try_from("compr6CUsB5m2jS4Y3831ztGSTnDpnKJTKS95d64XVq").unwrap();
    
    // Discriminator + Amount (1 SOL = 1_000_000_000 lamports)
    let lamports: u64 = 1_000_000_000;
    let mut data = DISCRIMINATOR_COMPRESS_SOL.to_vec();
    data.extend_from_slice(&lamports.to_le_bytes());

    let result = analyzer.analyze(&program_id, &data);
    
    if let Some(ExtensionAction::LightProtocol(LightProtocolAction::CompressSol { lamports: l })) = result {
        assert_eq!(l, Some(lamports));
    } else {
        panic!("Failed to detect CompressSol with amount: {:?}", result);
    }
}

#[test]
fn test_ignore_system_program() {
    let analyzer = LightProtocol;
    let program_id = programs::system_program();
    let data = vec![0u8; 8]; // Random data

    let result = analyzer.analyze(program_id, &data);
    assert!(result.is_none());
}

#[test]
fn test_unknown_light_instruction() {
    let analyzer = LightProtocol;
    let program_id = PubkeyBase58::try_from("cTokenmWW8bLPjZEBAUgYy3zKxQZW6VKi7bqNFEVv3m").unwrap();
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8]; // Random discriminator

    let result = analyzer.analyze(&program_id, &data);
    if let Some(ExtensionAction::LightProtocol(LightProtocolAction::Unknown { discriminator })) = result {
        assert_eq!(discriminator, [1, 2, 3, 4, 5, 6, 7, 8]);
    } else {
        panic!("Should have detected unknown instruction");
    }
}
