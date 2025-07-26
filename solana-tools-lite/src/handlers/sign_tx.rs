use crate::{
    crypto::ed25519,
    errors::{Result, SignError},
    models::pubkey_base58::PubkeyBase58,
    models::transaction::Transaction,
    utils::{serialize},
};
use ed25519_dalek::{Signature, SigningKey};
use crate::layers::io::*;
use crate::adapters::io_adapter::read_input_transaction;
use crate::models::input_transaction::InputTransaction;

/// Read tx JSON → sign → output JSON / stdout
//TODO: 🟡 rename into common name
pub fn handle_sign_transaction_file(
    input: Option<&String>, //TODO: use Path?
    secret_key_b58: &str,
    output: Option<&String>,
    json_pretty: bool,
) -> Result<()> {
    // 1. Load TX (JSON, Base64, or Base58) and convert to domain model
    // println!("[DEBUG] -- before read_input");
    let input_tx: InputTransaction = read_input_transaction(input.map(|s| s.as_str()))?;

    let mut tx: Transaction = Transaction::try_from(input_tx)?;
      

      println!("[DEBUG] --  Transaction::try_from(input_tx)");
    // 2. Decode & validate secret key
    let secret_bytes = bs58::decode(secret_key_b58)
        .into_vec()
        .map_err(|_| SignError::InvalidBase58)?;
    let seed: &[u8; 32] = secret_bytes
        .as_slice()
        .try_into()
        .map_err(|_| SignError::InvalidKeyLength)?;
    let signing_key = SigningKey::from_bytes(seed);

    // 3. Sign message
    sign_transaction_by_key(&mut tx, &signing_key)?;

    // 4. Serialize back
    let json_out = if json_pretty {
        serde_json::to_string_pretty(&tx).map_err(SignError::JsonSerialize)?
    } else {
        serde_json::to_string(&tx).map_err(SignError::JsonSerialize)?
    };

    // 5. Output
    if let Some(path) = output {
        write_output(Some(path.as_str()), &json_out)?;
    } else {
        println!("{json_out}");
    }

    Ok(())
}

/// Signs a transaction using the provided signing key.
/// Finds the matching pubkey in account_keys and inserts the signature in the correct slot.
///
/// Returns an error if the pubkey is not found or if it’s not a required signer.
pub fn sign_transaction_by_key(tx: &mut Transaction, key: &SigningKey) -> Result<()> {
    let pubkey = PubkeyBase58::try_from(key.verifying_key().to_bytes())
        .map_err(|_| SignError::InvalidPubkeyFormat)?; //TODO: check if error is actual

        //TODO: additionally check position and signers
    let signer_index = tx
        .message
        .account_keys
        .iter()
        .position(|k| *k == pubkey)
        .ok_or(SignError::SignerKeyNotFound)?;

    if signer_index >= tx.message.header.num_required_signatures as usize {
        return Err(SignError::SigningNotRequiredForKey)?;
    }

    let msg_bytes = serialize(&tx.message)?;
    let sig = ed25519::sign_message(key, &msg_bytes);

    // Resize signatures if needed
    if tx.signatures.len() <= signer_index {
        tx.signatures
            .resize(signer_index + 1, Signature::from_bytes(&[0u8; 64]));
    }

    tx.signatures[signer_index] = sig;
    Ok(())
}
