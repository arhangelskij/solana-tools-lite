use crate::flows::presenter::Presentable;
use crate::models::cmds::Base58Action;
use solana_tools_lite::handlers::base58;
use crate::shell::error::CliError;

/// Base58 flow: delegates to the pure handler and presents the result.
pub fn execute(action: &Base58Action, json: bool) -> Result<(), CliError> {
    let result = match action {
        Base58Action::Encode { input } => base58::encode(input)?,
        Base58Action::Decode { input } => base58::decode(input)?,
    };

    result.present(json, false, false)?;
    Ok(())
}
