use crate::crypto::bip39;
use crate::errors::GenError;
use crate::models::results::GenResult;
use crate::utils::hex_encode;
use crate::utils::pretty_print_json;
use ed25519_dalek::SigningKey;

pub fn handle_gen(
    mnemonic: Option<String>,
    passphrase: Option<String>,
    json: bool,
) -> anyhow::Result<()> {
    let mnemonic = match mnemonic {
        Some(m) => {
            bip39::validate_mnemonic(&m).map_err(|e| e)?;
            m
        }
        None => bip39::generate_mnemonic().map_err(|e| e)?,
    };

    let passphrase = passphrase.unwrap_or_default();

    let seed = bip39::derive_seed(&mnemonic, &passphrase).map_err(|e| e)?;
//TODO: 32 into const?
    let signing_key = SigningKey::from_bytes(&seed[..32]
        .try_into()
        .map_err(|_| GenError::InvalidSeedLength)?);

    let pubkey_bytes = signing_key.verifying_key().to_bytes();
    let pubkey_base58 = bs58::encode(pubkey_bytes).into_string();

    if json {
        let result = GenResult {
            mnemonic: mnemonic.clone(),
            public_key_base58: pubkey_base58.clone(),
            secret_key_base58: bs58::encode(signing_key.to_bytes()).into_string(),
            seed_hex: hex_encode(&seed),
            note: "Keep your mnemonic and secret key safe!",
            error: None,
        };
        pretty_print_json(&result);
    } else {
        println!("{}", pubkey_base58);
    }

    Ok(())
}
