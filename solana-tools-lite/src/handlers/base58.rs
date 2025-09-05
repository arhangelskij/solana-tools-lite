use crate::models::cmds::Base58Action;
use crate::errors::Result;

pub fn handle_base58(action: &Base58Action) ->  Result<()> {
    match action {
        Base58Action::Encode { input } => {
            let encoded = bs58::encode(input.as_bytes()).into_string();
            println!("{}", encoded);
        }
        Base58Action::Decode { input } => {
            let bytes = bs58::decode(input).into_vec()?; 
            println!("{}", String::from_utf8_lossy(&bytes)); //TODO: 5sept 🔴 not should not print
        }
    }
    Ok(())
}