use std::{fs};
use ed25519_dalek::{SigningKey, Signature};
use crate::{
    errors::{SignError, Result},
    models::transaction::Transaction,
    crypto::ed25519,
    utils::{pretty_print_json, read_stdin_or_file},
};

/// Read tx JSON → sign → output JSON / stdout
pub fn handle_sign_transaction_file(
    input: &Option<String>,
    secret_key_b58: &str,
    output: &Option<String>,
    json_pretty: bool,
) -> Result<()> {
    // 1. Load TX JSON (file or stdin)
    let tx_raw = read_stdin_or_file(input)?;              // -> String
    let mut tx: Transaction =
        serde_json::from_str(&tx_raw).map_err(SignError::SerdeJson)?;

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
        serde_json::to_string_pretty(&tx).map_err(SignError::SerdeJson)?
    } else {
        serde_json::to_string(&tx).map_err(SignError::SerdeJson)?
    };

    // 5. Output
    if let Some(path) = output {
        fs::write(path, &json_out).map_err(SignError::Io)?;
    } else {
        println!("{json_out}");
    }

    Ok(())
}

/// Helper: sign first signature slot
pub fn sign_transaction(tx: &mut Transaction, key: &SigningKey) -> Result<()> {
    let msg_bytes = serde_json::to_vec(&tx.message).map_err(SignError::SerdeJson)?;
    let sig: Signature = ed25519::sign_message(key, &msg_bytes);

    let sig_b58 = bs58::encode(sig.to_bytes()).into_string();
    if tx.signatures.is_empty() {
        tx.signatures.push(sig_b58);
    } else {
        tx.signatures[0] = sig_b58;
    }
    Ok(())
}

/*
//TODO: 🔴 in prod use bincode and canonical message packing
pub(crate) fn sign_transaction(tx: &mut Transaction, signing_key: &SigningKey) ->  anyhow::Result<()> {
    let message_bytes: Vec<u8> = serde_json::to_vec(&tx.message).map_err(SignError::SerdeJson)?; // or bincode
    
    let signature = ed25519::sign_message(signing_key, &message_bytes);

    if tx.signatures.is_empty() {
        tx.signatures.push(bs58::encode(signature.to_bytes()).into_string());
    } else {
        tx.signatures[0] = bs58::encode(signature.to_bytes()).into_string();
    }

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
    let tx_data = fs::read_to_string(input).map_err(SignError::Io)?; 

    // 2. Parse JSON into Transaction
    let mut tx: Transaction = serde_json::from_str(&tx_data)?;
       // .with_context(|| format!("Failed to parse input as Transaction JSON"))?;


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
        serde_json::to_string_pretty(&tx)? //TODO: check when you will test if correct error is returning
    } else {
        serde_json::to_string(&tx)?
    };

    // 6. Output
    if let Some(out_path) = output {
        let mut file = fs::File::create(out_path) .map_err(SignError::Io)?; //TODO: add out_path!
          //  .with_context(|| format!("Failed to create output file: {out_path}"))?;
       
        
        file.write_all(result_json.as_bytes())?;
           // .with_context(|| format!("Failed to write to output file: {out_path}"))?;
       // .map_err(SignError::Io)?; //TODO: think about to add output and write errors!
        
        if !json {
            println!("Transaction signed and saved to {out_path}");
        }
    } else {
        println!("{result_json}");
    }

    Ok(())
}

*/