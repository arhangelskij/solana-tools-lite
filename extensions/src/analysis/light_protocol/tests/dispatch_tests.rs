
use crate::analysis::light_protocol::parsing::parse_light_instruction;
use crate::analysis::light_protocol::models::LightProtocolAction;
use crate::analysis::light_protocol::constants;
use solana_tools_lite::models::pubkey_base58::PubkeyBase58;

#[test]
fn test_ctoken_instruction_dispatch_table() {
    let program_id = PubkeyBase58::try_from(constants::COMPRESSED_TOKEN_PROGRAM_ID).unwrap();
    
    // Structure: (discriminator, amount, check_function)
    struct TestCase {
        disc: u8,
        amount: u64,
        check: fn(LightProtocolAction, u64) -> bool,
        name: &'static str,
    }

    let cases = vec![
        TestCase {
            disc: constants::DISCRIMINATOR_CTOKEN_TRANSFER,
            amount: 100,
            check: |a, v| matches!(a, LightProtocolAction::CTokenTransfer { amount: Some(x) } if x == v),
            name: "CTokenTransfer",
        },
        TestCase {
            disc: constants::DISCRIMINATOR_CTOKEN_MINT_TO,
            amount: 200,
            check: |a, v| matches!(a, LightProtocolAction::CTokenMintTo { amount: Some(x) } if x == v),
            name: "CTokenMintTo",
        },
        TestCase {
            disc: constants::DISCRIMINATOR_CTOKEN_BURN,
            amount: 300,
            check: |a, v| matches!(a, LightProtocolAction::CTokenBurn { amount: Some(x) } if x == v),
            name: "CTokenBurn",
        },
        TestCase {
            disc: constants::DISCRIMINATOR_CTOKEN_TRANSFER_CHECKED,
            amount: 400,
            check: |a, v| matches!(a, LightProtocolAction::CTokenTransferChecked { amount: Some(x) } if x == v),
            name: "CTokenTransferChecked",
        },
        TestCase {
            disc: constants::DISCRIMINATOR_CTOKEN_MINT_TO_CHECKED,
            amount: 500,
            check: |a, v| matches!(a, LightProtocolAction::CTokenMintToChecked { amount: Some(x) } if x == v),
            name: "CTokenMintToChecked",
        },
        TestCase {
            disc: constants::DISCRIMINATOR_CTOKEN_BURN_CHECKED,
            amount: 600,
            check: |a, v| matches!(a, LightProtocolAction::CTokenBurnChecked { amount: Some(x) } if x == v),
            name: "CTokenBurnChecked",
        },
        // Approve instructions
        TestCase {
            disc: constants::DISCRIMINATOR_CTOKEN_APPROVE,
            amount: 700,
            check: |a, v| matches!(a, LightProtocolAction::CTokenApprove { amount: Some(x) } if x == v),
            name: "CTokenApprove",
        },
    ];

    for case in cases {
        let mut data = Vec::new();
        data.push(case.disc);
        data.extend_from_slice(&case.amount.to_le_bytes()); 
        // Pad to ensure we have enough bytes (though amount is at offset 1, so 1+8=9 bytes is min)
        data.extend_from_slice(&[0u8; 10]); 

        let action = parse_light_instruction(&program_id, &data);
        
        assert!(
            (case.check)(action.clone(), case.amount),
            "Failed check for {}: parsed as {:?}",
            case.name,
            action
        );
    }
}

#[test]
fn test_ctoken_withdraw_funding_pool() {
     let program_id = PubkeyBase58::try_from(constants::COMPRESSED_TOKEN_PROGRAM_ID).unwrap();
     let amount = 999u64;
     let mut data = Vec::new();
     data.push(constants::DISCRIMINATOR_WITHDRAW_FUNDING_POOL);
     data.extend_from_slice(&amount.to_le_bytes()); 
     
     let action = parse_light_instruction(&program_id, &data);
     if let LightProtocolAction::WithdrawFundingPool { amount: Some(parsed) } = action {
         assert_eq!(parsed, amount);
     } else {
         panic!("Failed to parse WithdrawFundingPool: {:?}", action);
     }
}
