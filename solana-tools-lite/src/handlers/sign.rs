use ed25519_dalek::{SigningKey, Signature, Signer};
use anyhow::{Result, Context};
use std::convert::TryInto;
use crate::models::results::SignResult;
use crate::utils::pretty_print_json;
use crate::models::transaction::Transaction;
use crate::crypto::ed25519;

/// Signs a given message with a provided secret key (base58 encoded)
pub fn handle_sign(message: &str, secret_key_b58: &str, json: bool) -> Result<String> {
    // Decode the base58 secret key
    let secret_bytes = bs58::decode(secret_key_b58)
        .into_vec()
        .context("Invalid base58 in secret key")?; //TODO: delete context + add custom errors

    // Convert to [u8; 32] (only the private seed part is needed)
    let secret_bytes_arr: &[u8; 32] = secret_bytes
        .as_slice()
        .try_into()
        .context("Secret key must be 32 bytes")?;

    // Create SigningKey from seed
    let signing_key = SigningKey::from_bytes(secret_bytes_arr);

    // Sign the message
    let signature: Signature = signing_key.sign(message.as_bytes());

    // Encode the signature in base58
    let signature_b58 = bs58::encode(signature.to_bytes()).into_string();
    let pubkey_b58 = bs58::encode(signing_key.verifying_key().to_bytes()).into_string();

    if json {
        let result = SignResult {
            message: message.to_string(),
            signature_base58: signature_b58.clone(),
            public_key: pubkey_b58
        };
        pretty_print_json(&result);
    } else {
        println!("{signature_b58}");
    }

    Ok(signature_b58)
}

//TODO: in prod use bincode and canonical message packing
pub(crate) fn sign_transaction(tx: &mut Transaction, signing_key: &SigningKey) ->  anyhow::Result<()> {
    let message_bytes: Vec<u8> = serde_json::to_vec(&tx.message)?; // или bincode
    let signature = ed25519::sign_message(signing_key, &message_bytes);
    if tx.signatures.is_empty() {
        tx.signatures.push(bs58::encode(signature.to_bytes()).into_string());
    } else {
        tx.signatures[0] = bs58::encode(signature.to_bytes()).into_string();
    }
//TODO: anyhow result + errors

    Ok(())
}

pub fn handle_sign_transaction_file(
    input: &String,
    secret_key: &String,
    output: &Option<String>,
    json: bool,
) -> anyhow::Result<()> {
    use std::fs;
    use std::io::Write;

    // 1. Read the transaction file
    let tx_data = fs::read_to_string(input)
        .with_context(|| format!("Failed to read input file: {input}"))?;

    // 2. Parse JSON into Transaction
    let mut tx: Transaction = serde_json::from_str(&tx_data)
        .with_context(|| format!("Failed to parse input as Transaction JSON"))?;

    // 3. Decode secret key (base58)
    let secret_bytes = bs58::decode(secret_key)
        .into_vec()
        .context("Invalid base58 in secret key")?;
    let secret_bytes_arr: &[u8; 32] = secret_bytes
        .as_slice()
        .try_into()
        .context("Secret key must be 32 bytes")?;
    let signing_key = SigningKey::from_bytes(secret_bytes_arr);

    // 4. Sign the transaction
    sign_transaction(&mut tx, &signing_key)?;

    // 5. Serialize updated tx as JSON
    let result_json = if json {
        serde_json::to_string_pretty(&tx)?
    } else {
        serde_json::to_string(&tx)?
    };

    // 6. Output
    if let Some(out_path) = output {
        let mut file = fs::File::create(out_path)
            .with_context(|| format!("Failed to create output file: {out_path}"))?; //TODO: ? add cust errors
        
        file.write_all(result_json.as_bytes())
            .with_context(|| format!("Failed to write to output file: {out_path}"))?;
        
        if !json {
            println!("Transaction signed and saved to {out_path}");
        }
    } else {
        println!("{result_json}");
    }

    Ok(())
}