use crate::models::cmds::Base58Action;

pub fn handle_base58(action: &Base58Action) -> anyhow::Result<()> {
    match action {
        Base58Action::Encode { input } => {
            let encoded = bs58::encode(input.as_bytes()).into_string();
            println!("{}", encoded);
        }
        Base58Action::Decode { input } => {
            let bytes = bs58::decode(input).into_vec()?; 
            println!("{}", String::from_utf8_lossy(&bytes));
        }
    }
    Ok(())
}