use solana_tools_lite::models::input_transaction::{
    UiRawMessage, UiRawMessageLegacy, UiTransaction,
};
use solana_tools_lite::models::message::MessageHeader;
use solana_tools_lite::serde::fmt::{OutputFormat, encode_ui_transaction};

fn build_min_ui() -> UiTransaction {
    UiTransaction {
        signatures: vec![],
        message: UiRawMessage::Legacy(UiRawMessageLegacy {
            header: MessageHeader {
                num_required_signatures: 1,
                num_readonly_signed_accounts: 0,
                num_readonly_unsigned_accounts: 1,
            },
            account_keys: vec![
                "11111111111111111111111111111111".to_string(),
                "11111111111111111111111111111111".to_string(),
            ],
            recent_blockhash: "11111111111111111111111111111111".to_string(),
            instructions: vec![],
        }),
    }
}

#[test]
fn test_encode_ui_transaction_json_pretty_and_plain() {
    let ui = build_min_ui();
    let plain = encode_ui_transaction(&ui, OutputFormat::Json { pretty: false }).unwrap();
    let pretty = encode_ui_transaction(&ui, OutputFormat::Json { pretty: true }).unwrap();
    
    assert_eq!(plain, serde_json::to_string(&ui).unwrap());
    assert_eq!(pretty, serde_json::to_string_pretty(&ui).unwrap());
}
