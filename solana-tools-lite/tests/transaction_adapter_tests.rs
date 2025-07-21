use solana_tools_lite::models::transaction::Transaction;
use solana_tools_lite::models::input_transaction::InputTransaction;
use std::fs;
use std::path::PathBuf;

#[path = "utils.rs"]
mod utils;
use utils::{generate_mock_pubkey, generate_mock_signature};


#[test]
fn test_adapter_with_generated_data() {
    let pubkey1 = generate_mock_pubkey();
    let pubkey2 = generate_mock_pubkey();
    let program_id = generate_mock_pubkey();
    let blockhash = generate_mock_pubkey();

    let signature = generate_mock_signature();

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/data/tx_template.json");
    let template = fs::read_to_string(path).expect("Template not found");

    let json = template
        .replace("$PK1", &pubkey1)
        .replace("$PK2", &pubkey2)
        .replace("$PROGID", &program_id)
        .replace("$BLOCKHASH", &blockhash)
        .replace("$SIG", &signature)
        .replace("$DATA", "3Bxs4SHffsqLHuC3");


    let parsed: InputTransaction = serde_json::from_str(&json).expect("failed to parse");
    let tx: Transaction = TryFrom::try_from(parsed).expect("conversion failed");

    assert_eq!(tx.signatures.len(), 1);
     assert_eq!(tx.message.account_keys.len(), 3);
    assert_eq!(tx.message.instructions.len(), 1);
}

#[test]
fn test_adapter_invalid_pubkey() {
    let input = r#"
    {
        "signatures": ["3prfupj2PMawf5PBYTnCaJzD1eBaFApgx2MkckXQoo7o4deNyEeeRzKA4JTqpXUWPfYw5PmHBQmUVHTba9vS3wXh"],
        "message": {
            "header": {
                "num_required_signatures": 1,
                "num_readonly_signed_accounts": 0,
                "num_readonly_unsigned_accounts": 1
            },
            "account_keys": ["INVALIDBASE58!"],
            "recent_blockhash": "3bsiZrwkE1FtgxeJtMrdBpBteYpXMHYutFSbLMVYEFH4",
            "instructions": []
        }
    }
    "#;

    let parsed = serde_json::from_str::<InputTransaction>(input);
    assert!(parsed.is_ok(), "InputTransaction JSON should be valid");

    let parsed = parsed.unwrap();
    let result: Result<Transaction, _> = TryFrom::try_from(parsed);

    assert!(result.is_err(), "Expected conversion to fail");
    let err = result.err().unwrap();
    assert!(
        err.to_string().to_lowercase().contains("invalid"),
        "Expected base58 decode error, got: {err}"
    );
}

#[test]
fn test_adapter_invalid_base64_input() {
    let invalid_base64 = InputTransaction::Base64("not@@base64$$".to_string());

    let result: Result<Transaction, _> = TryFrom::try_from(invalid_base64);

    assert!(result.is_err(), "Expected Base64 decoding to fail");
    let err = result.err().unwrap();
    assert!(
        err.to_string().to_lowercase().contains("base64"),
        "Expected base64 error, got: {err}"
    );
}

#[test]
fn test_adapter_invalid_base58_input() {
    let invalid_base58 = InputTransaction::Base58("not@@base58$$".to_string());

    let result: Result<Transaction, _> = TryFrom::try_from(invalid_base58);

    assert!(result.is_err(), "Expected Base58 decoding to fail");
    let err = result.err().unwrap();
    assert!(
        err.to_string().to_lowercase().contains("base58"),
        "Expected base58 error, got: {err}"
    );
}

//TODO: add more tests

///////////////////////////////////////////////
//TODO: 🔴 1!
#[test]
fn test_adapter_valid_base64_json() -> Result<(), Box<dyn std::error::Error>> {
    // This Base64 encodes a UiTransaction JSON with one signature, three account keys, and one instruction.
    let b64 = "ewogICJzaWduYXR1cmVzIjogWwogICAgIjVSSGJkUERSTTNFa0p1OXVGcUNrR3ZEOEJFekVZYVdHV1ljWW93WFE4NERXUlhZZEtyRVJwOUhadjM2V2JUb1JZRG5HekJOUlJMOVAzbXVDUTRuYjMzWHoiCiAgXSwKICAibWVzc2FnZSI6IHsKICAgICJoZWFkZXIiOiB7CiAgICAgICJudW1SZXF1aXJlZFNpZ25hdHVyZXMiOiAxLAogICAgICAibnVtUmVhZG9ubHlTaWduZWRBY2NvdW50cyI6IDAsCiAgICAgICJudW1SZWFkb25seVVuc2lnbmVkQWNjb3VudHMiOiAxCiAgICB9LAogICAgImFjY291bnRLZXlzIjogWwogICAgICAiSFllRlVkRGtKQUtzV1c3TWY2aTlFdEdNUjFFWmJrcndYdktrTHExWTFVOGgiLAogICAgICAiREV2UGJNUlh6Uk1DbVpxNzhQQnk0WHRVM3JCZVViS21rR0xWR3dlMkFOSnAiLAogICAgICAiMTExMTExMTExMTExMTExMTExMTExMTExMTExMTExMTEiCiAgICBdLAogICAgInJlY2VudEJsb2NraGFzaCI6ICI4aEVwVkdkcEZTc1d2SzE3UmIyRzR6Mmt0SnpndFJ3bkJaTlJqY1Rvamd0TiIsCiAgICAiaW5zdHJ1Y3Rpb25zIjogWwogICAgICB7CiAgICAgICAgInByb2dyYW1JZEluZGV4IjogMiwKICAgICAgICAiYWNjb3VudHMiOiBbMCwgMV0sCiAgICAgICAgImRhdGEiOiAiM0J4czRTSGZmc3FMSHVDMyIKICAgICAgfQogICAgXQogIH0KfQ==";
    // Simulate reading from file by constructing InputTransaction::Base64
    let input = InputTransaction::Base64(b64.to_string());

    let tx: Transaction = TryFrom::try_from(input)?;

    println!("tx: {:?}", tx);

    assert_eq!(tx.signatures.len(), 1);
    assert_eq!(tx.message.account_keys.len(), 3);
    assert_eq!(tx.message.instructions.len(), 1);
    Ok(())
}

#[test]
fn test_adapter_valid_base58_json() -> Result<(), Box<dyn std::error::Error>> {
    // This Base64 encodes a UiTransaction JSON with one signature, three account keys, and one instruction.
    let b58 = "6ZhHi5yP1uGGeudT33nHG6WNHfvvhLv84Zg2hsZaHnSTDF81EExFEM3KoAVf4p5GKD8CJwQCgb9FysAEZ7T1mmNtQv5FzNTchXAj4ebe8uFZS4pWKS35g2EiyDaZgoq1wM3hvNTFxCKDXNMCmELZTZg6BMT2Govb3j85VtMKpWrTEPiK3sExMf6VaJr7mazAcJeFx15XS5ARxL8ayjswNaXdRkyt2kJqFXC963j2LuCSztxFirYk8KU3hy2dMiPAgmNJd2BPeTLupAGKYkTx81HASmAkh5fvEpS7ADjv3r4sfe7eSQ95q6TdkBx7kY9FskcJ4mpaTKBKhomUF53SC29LUTh4wUSX9ePhHPKszYW82dKDBX743pKuPLZAvZQ23V3kmSJ2dgiARKEWMHrDjRFy7Dz5wz5RTs8kyrnjcRFC6UwKjym7xnnhMYLwe8F3z8cCbqTzidMp26D9euGeV8KfjE81xGS8AkgxgrF2xkms311Rnwk6e6hBwqkTozE1YWyQh6yNzvM1QNGW6KFjN878bwv16QXBjPkj9Gg2XSdjQCPoTRn9acxydZXSFHjvznNzGj6TKBuYeJyTjDePiXfrwCaF5FiTaWnY7We14eGwVKJ7HP5se9b1rcA21JjMmevKdEzZ6tabiv7ddjY8moiS3QghMZHhUwR5ddH6zo8ST1uxBQTa5v6CiAQ7PgcUgjQb4QHG1jeb5TQqdrD6okbmDQaudfVnq3CiChAnh5jDh65WCJ9dzd2ZgcZ1wd4cMwCpWDztecFtExe7UdWdPwHNZ4wWf3VaVjg5qS5C2q4Vi8AXv1CFyRdXxY4XXsP3a6guLDVkaP28oM1Eq6vyCeiWWmgRdNWycaACdAn92wGcjrnC5jhqoDuZdC121cqqP7i";
    // Simulate reading from file by constructing InputTransaction::Base64
    let input = InputTransaction::Base58(b58.to_string());

    let tx: Transaction = TryFrom::try_from(input)?;

    println!("tx: {:?}", tx);

    assert_eq!(tx.signatures.len(), 1);
    assert_eq!(tx.message.account_keys.len(), 3);
    assert_eq!(tx.message.instructions.len(), 1);
    Ok(())
}