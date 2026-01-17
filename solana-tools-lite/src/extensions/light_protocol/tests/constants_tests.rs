use crate::extensions::light_protocol::constants::*;

#[test]
fn test_supported_programs_success() {
    let result = supported_programs();
    assert!(result.is_ok());
    let programs = result.unwrap();
    assert_eq!(programs.len(), 3);
    
    // Verify all expected program IDs are present
    let program_strings: Vec<String> = programs.iter().map(|p| p.to_string()).collect();
    assert!(program_strings.contains(&LIGHT_SYSTEM_PROGRAM_ID.to_string()));
    assert!(program_strings.contains(&ACCOUNT_COMPRESSION_PROGRAM_ID.to_string()));
    assert!(program_strings.contains(&COMPRESSED_TOKEN_PROGRAM_ID.to_string()));
}

#[test]
fn test_supported_programs_consistent() {
    // Multiple calls should return the same result (lazy initialization)
    let result1 = supported_programs();
    let result2 = supported_programs();
    
    assert!(result1.is_ok());
    assert!(result2.is_ok());
    
    let programs1 = result1.unwrap();
    let programs2 = result2.unwrap();
    
    assert_eq!(programs1.len(), programs2.len());
    for (p1, p2) in programs1.iter().zip(programs2.iter()) {
        assert_eq!(p1, p2);
    }
}

#[test]
fn test_program_id_constants_are_valid() {
    // Test that all individual program ID constants are valid
    assert!(crate::models::pubkey_base58::PubkeyBase58::try_from(LIGHT_SYSTEM_PROGRAM_ID).is_ok());
    assert!(crate::models::pubkey_base58::PubkeyBase58::try_from(ACCOUNT_COMPRESSION_PROGRAM_ID).is_ok());
    assert!(crate::models::pubkey_base58::PubkeyBase58::try_from(COMPRESSED_TOKEN_PROGRAM_ID).is_ok());
}

#[test]
fn test_discriminator_constants() {
    // Verify discriminators are the expected size
    assert_eq!(DISCRIMINATOR_CREATE_MINT.len(), DISCRIMINATOR_SIZE);
    assert_eq!(DISCRIMINATOR_MINT_TO.len(), DISCRIMINATOR_SIZE);
    assert_eq!(DISCRIMINATOR_TRANSFER.len(), DISCRIMINATOR_SIZE);
    assert_eq!(DISCRIMINATOR_COMPRESS_SOL.len(), DISCRIMINATOR_SIZE);
    assert_eq!(DISCRIMINATOR_COMPRESS_TOKEN.len(), DISCRIMINATOR_SIZE);
    assert_eq!(DISCRIMINATOR_DECOMPRESS.len(), DISCRIMINATOR_SIZE);
    assert_eq!(DISCRIMINATOR_STATE_UPDATE.len(), DISCRIMINATOR_SIZE);
    assert_eq!(DISCRIMINATOR_CLOSE_ACCOUNT.len(), DISCRIMINATOR_SIZE);
    
    // Verify discriminators are unique
    let discriminators = [
        DISCRIMINATOR_CREATE_MINT,
        DISCRIMINATOR_MINT_TO,
        DISCRIMINATOR_TRANSFER,
        DISCRIMINATOR_COMPRESS_SOL,
        DISCRIMINATOR_COMPRESS_TOKEN,
        DISCRIMINATOR_DECOMPRESS,
        DISCRIMINATOR_STATE_UPDATE,
        DISCRIMINATOR_CLOSE_ACCOUNT,
    ];
    
    for (i, disc1) in discriminators.iter().enumerate() {
        for (j, disc2) in discriminators.iter().enumerate() {
            if i != j {
                assert_ne!(disc1, disc2, "Discriminators at indices {} and {} are identical", i, j);
            }
        }
    }
}