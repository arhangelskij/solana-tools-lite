use crate::crypto::base58;
use crate::models::cmds::Base58Action;

pub fn handle_base58(action: &Base58Action) -> anyhow::Result<()> {
    match action {
        Base58Action::Encode { input } => {
            let encoded = base58::encode(input.as_bytes());
            println!("{}", encoded);
        }
        Base58Action::Decode { input } => {
            let decoded = base58::decode(input)?;
            println!("{}", String::from_utf8_lossy(&decoded));
        }
    }
    Ok(())
}