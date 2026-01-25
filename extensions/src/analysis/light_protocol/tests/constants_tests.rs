use crate::analysis::light_protocol::constants::*;
use solana_tools_lite::models::pubkey_base58::PubkeyBase58;

#[test]
fn test_supported_programs_success() {
    let result = supported_programs();
    assert!(result.is_ok());
    let programs = result.unwrap();
    assert_eq!(programs.len(), 5);
    
    // Verify all expected program IDs are present
    let program_strings: Vec<String> = programs.iter().map(|p| p.to_string()).collect();
    assert!(program_strings.contains(&LIGHT_SYSTEM_PROGRAM_ID.to_string()));
    assert!(program_strings.contains(&ACCOUNT_COMPRESSION_PROGRAM_ID.to_string()));
    assert!(program_strings.contains(&COMPRESSED_TOKEN_PROGRAM_ID.to_string()));
    assert!(program_strings.contains(&LIGHT_REGISTRY_ID.to_string()));
    assert!(program_strings.contains(&SPL_NOOP_PROGRAM_ID.to_string()));
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
    assert!(PubkeyBase58::try_from(LIGHT_SYSTEM_PROGRAM_ID).is_ok());
    assert!(PubkeyBase58::try_from(ACCOUNT_COMPRESSION_PROGRAM_ID).is_ok());
    assert!(PubkeyBase58::try_from(COMPRESSED_TOKEN_PROGRAM_ID).is_ok());
    assert!(PubkeyBase58::try_from(LIGHT_REGISTRY_ID).is_ok());
    assert!(PubkeyBase58::try_from(SPL_NOOP_PROGRAM_ID).is_ok());
}

#[test]
fn test_compressed_token_program_discriminators() {
    // Test 1-byte discriminators for Compressed Token Program
    assert_eq!(DISCRIMINATOR_CTOKEN_TRANSFER, 3);
    assert_eq!(DISCRIMINATOR_CTOKEN_APPROVE, 4);
    assert_eq!(DISCRIMINATOR_CTOKEN_REVOKE, 5);
    assert_eq!(DISCRIMINATOR_CTOKEN_MINT_TO, 7);
    assert_eq!(DISCRIMINATOR_CTOKEN_BURN, 8);
    assert_eq!(DISCRIMINATOR_CLOSE_TOKEN_ACCOUNT, 9);
    assert_eq!(DISCRIMINATOR_CTOKEN_FREEZE_ACCOUNT, 10);
    assert_eq!(DISCRIMINATOR_CTOKEN_THAW_ACCOUNT, 11);
    assert_eq!(DISCRIMINATOR_CTOKEN_TRANSFER_CHECKED, 12);
    assert_eq!(DISCRIMINATOR_CTOKEN_MINT_TO_CHECKED, 14);
    assert_eq!(DISCRIMINATOR_CTOKEN_BURN_CHECKED, 15);
    assert_eq!(DISCRIMINATOR_CREATE_TOKEN_ACCOUNT, 18);
    assert_eq!(DISCRIMINATOR_CREATE_ASSOCIATED_TOKEN_ACCOUNT, 100);
    assert_eq!(DISCRIMINATOR_TRANSFER2, 101);
    assert_eq!(DISCRIMINATOR_CREATE_ASSOCIATED_TOKEN_ACCOUNT_IDEMPOTENT, 102);
    assert_eq!(DISCRIMINATOR_MINT_ACTION, 103);
    assert_eq!(DISCRIMINATOR_CLAIM, 104);
    assert_eq!(DISCRIMINATOR_WITHDRAW_FUNDING_POOL, 105);
}

#[test]
fn test_light_system_program_discriminators() {
    // Test 8-byte discriminators for Light System Program
    assert_eq!(DISCRIMINATOR_INVOKE, [26, 16, 169, 7, 21, 202, 242, 25]);
    assert_eq!(DISCRIMINATOR_INVOKE_CPI, [49, 212, 191, 129, 39, 194, 43, 196]);
    assert_eq!(DISCRIMINATOR_INVOKE_CPI_WITH_READ_ONLY, [86, 47, 163, 166, 21, 223, 92, 8]);
    assert_eq!(DISCRIMINATOR_INVOKE_CPI_WITH_ACCOUNT_INFO, [228, 34, 128, 84, 47, 139, 86, 240]);
}

#[test]
fn test_account_compression_program_discriminators() {
    // Test 8-byte discriminators for Account Compression Program
    assert_eq!(DISCRIMINATOR_INSERT_INTO_QUEUES, [180, 143, 159, 153, 35, 46, 248, 163]);
}

#[test]
fn test_light_registry_program_discriminators() {
    // Test 8-byte discriminators for Light Registry Program
    assert_eq!(DISCRIMINATOR_CREATE_CONFIG_COUNTER, [221, 9, 219, 187, 215, 138, 209, 87]);
    assert_eq!(DISCRIMINATOR_CREATE_COMPRESSIBLE_CONFIG, [13, 182, 188, 82, 224, 82, 11, 174]);
}

#[test]
fn test_token_interface_discriminators() {
    // Test 8-byte discriminators for Token Interface
    assert_eq!(DISCRIMINATOR_TOKEN_INTERFACE_MINT_TO, [241, 34, 48, 186, 37, 179, 123, 192]);
    assert_eq!(DISCRIMINATOR_TOKEN_INTERFACE_TRANSFER, [163, 52, 200, 231, 140, 3, 69, 186]);
    assert_eq!(DISCRIMINATOR_BATCH_COMPRESS, [65, 206, 101, 37, 147, 42, 221, 144]);
    assert_eq!(DISCRIMINATOR_TOKEN_INTERFACE_APPROVE, [69, 74, 217, 36, 115, 117, 97, 76]);
    assert_eq!(DISCRIMINATOR_TOKEN_INTERFACE_REVOKE, [170, 23, 31, 34, 133, 173, 93, 242]);
    assert_eq!(DISCRIMINATOR_TOKEN_INTERFACE_FREEZE, [255, 91, 207, 84, 251, 194, 254, 63]);
    assert_eq!(DISCRIMINATOR_CTOKEN_THAW, [226, 249, 34, 57, 189, 21, 177, 101]);
    assert_eq!(DISCRIMINATOR_CREATE_TOKEN_POOL, [23, 169, 27, 122, 147, 169, 209, 152]);
    assert_eq!(DISCRIMINATOR_ADD_TOKEN_POOL, [114, 143, 210, 73, 96, 115, 1, 228]);
}

#[test]
fn test_u64_and_u16_sizes() {
    // Verify size constants
    assert_eq!(U64_SIZE, 8);
    assert_eq!(U16_SIZE, 2);
}
