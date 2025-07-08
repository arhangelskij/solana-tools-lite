#[test]
fn test_fail_on_serialize_unsupported_type() {
    use solana_tools_lite::{errors::ToolError, utils::serialize};
    struct BadSerialize;

    impl serde::Serialize for BadSerialize {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            Err(serde::ser::Error::custom(
                "Intentional serialization failure",
            ))
        }
    }

    #[derive(serde::Serialize)]
    struct Bad {
        field: BadSerialize,
    }

    let data = Bad {
        field: BadSerialize,
    };

    let result = serialize(&data);
    match result {
        Err(ToolError::Bincode(_)) => { /* ok */ }
        Err(e) => panic!("Wrong error type: {:?}", e),
        Ok(_) => panic!("Expected error, got Ok"),
    }
}