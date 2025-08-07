use crate::adapters::io_adapter::{OutputFormat, read_input_transaction, write_output_transaction, read_secret_key_file, parse_signing_key_content};
use crate::models::input_transaction::{InputTransaction, UiTransaction};
use crate::{
    crypto::ed25519,
    errors::{Result, SignError},
    models::cmds::OutFmt,
    models::pubkey_base58::PubkeyBase58,
    models::transaction::Transaction,
    utils::serialize
};

use ed25519_dalek::{Signature, SigningKey};

/// Read tx JSON → sign → output JSON / stdout
//TODO: 🟡 rename into common name
pub fn handle_sign_transaction_file(
    input: Option<&String>, //TODO: use Path?
    secret_key_b58: &str,
    output: Option<&String>,
    json_pretty: bool,
    out_override: Option<OutFmt>,
) -> Result<()> {
    // 1. Load TX (JSON, Base64, or Base58) and convert to domain model
    let input_tx: InputTransaction = read_input_transaction(input.map(|s| s.as_str()))?;

    let default_format = match &input_tx {
        InputTransaction::Json(_) => OutputFormat::Json {
            pretty: json_pretty
        },
        InputTransaction::Base64(_) => OutputFormat::Base64,
        InputTransaction::Base58(_) => OutputFormat::Base58
    };

    let mut tx: Transaction = Transaction::try_from(input_tx)?;
    
    // 2. Read and parse the secret key content (file or "-")
    let key_text = read_secret_key_file(secret_key_b58)?;
    let signing_key = parse_signing_key_content(&key_text)?;

    // 3. Sign message
    sign_transaction_by_key(&mut tx, &signing_key)?;

    // 4. Serialize back (to UI DTO)
    let ui_tx = UiTransaction::from(&tx);

    // Override format output if needed
    let chosen_format = match out_override {
        Some(OutFmt::Json) => OutputFormat::Json {
            pretty: json_pretty
        },
        Some(OutFmt::Base64) => OutputFormat::Base64,
        Some(OutFmt::Base58) => OutputFormat::Base58,
        None => default_format
    };

    // 5. Output
    write_output_transaction(&ui_tx, chosen_format, output.map(|s| s.as_str()))?;

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
