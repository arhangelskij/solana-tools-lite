use ed25519_dalek::{SigningKey, Signature};
use crate::{
    errors::{SignError, Result},
    models::transaction::Transaction,
    crypto::ed25519,
    utils::{self, read_stdin_or_file, write_file},
};

/// Read tx JSON → sign → output JSON / stdout
pub fn handle_sign_transaction_file(
    input: &Option<String>,
    secret_key_b58: &str,
    output: &Option<String>,
    json_pretty: bool,
) -> Result<()> {
    // 1. Load TX JSON (file or stdin)
    let tx_raw = self::read_stdin_or_file(input)?;              // -> String
    let mut tx: Transaction =
        serde_json::from_str(&tx_raw).map_err(SignError::JsonParse)?;

    // 2. Decode & validate secret key
    let secret_bytes = bs58::decode(secret_key_b58)
        .into_vec()
        .map_err(|_| SignError::InvalidBase58)?;
    let seed: &[u8; 32] = secret_bytes.as_slice()
        .try_into()
        .map_err(|_| SignError::InvalidKeyLength)?;
    let signing_key = SigningKey::from_bytes(seed);

    // 3. Sign message
    sign_transaction(&mut tx, &signing_key)?;

    // 4. Serialize back
    let json_out = if json_pretty {
        serde_json::to_string_pretty(&tx).map_err(SignError::JsonSerialize)?
    } else {
        serde_json::to_string(&tx).map_err(SignError::JsonSerialize)?
    };

    // 5. Output
    if let Some(path) = output {
        write_file(path, &json_out)?;
    } else {
        println!("{json_out}");
    }

    Ok(())
}

/// Helper: sign first signature slot
pub fn sign_transaction(tx: &mut Transaction, key: &SigningKey) -> Result<()> {
    let msg_bytes = utils::serialize(&tx.message)?;
    let sig: Signature = ed25519::sign_message(key, &msg_bytes);

    let sig_b58 = bs58::encode(sig.to_bytes()).into_string();
    if tx.signatures.is_empty() {
        tx.signatures.push(sig_b58);
    } else {
        tx.signatures[0] = sig_b58;
    }
    Ok(())
}