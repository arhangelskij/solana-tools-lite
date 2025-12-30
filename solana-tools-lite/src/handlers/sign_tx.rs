use crate::codec::serialize_message;
use crate::crypto::signing::sign_message;
use crate::errors::SignError;
use crate::models::input_transaction::InputTransaction;
use crate::models::results::SignTxResult;
use crate::models::{PubkeyBase58, Transaction};
use crate::Result;

use ed25519_dalek::{Signature, SigningKey};

/// Pure handler: sign an input transaction with the given key and return a domain result.
pub fn handle(
    input_tx: InputTransaction,
    signing_key: &SigningKey,
) -> Result<SignTxResult> {
    let mut tx: Transaction = Transaction::try_from(input_tx)?;
    tx.message.sanitize()?;

    sign_transaction_by_key(&mut tx, signing_key)?;
    Ok(SignTxResult { signed_tx: tx })
}

/// Signs a transaction using the provided signing key.
/// Finds the matching pubkey in account_keys and inserts the signature in the correct slot.
///
/// Returns an error if the pubkey is not found or if it’s not a required signer.
pub fn sign_transaction_by_key(tx: &mut Transaction, key: &SigningKey) -> Result<()> {
    let pubkey = PubkeyBase58::from(key.verifying_key().to_bytes());

    let signer_index = tx
        .message
        .account_keys()
        .iter()
        .position(|k| *k == pubkey)
        .ok_or(SignError::SignerKeyNotFound)?;

    if signer_index >= tx.message.header().num_required_signatures as usize {
        return Err(SignError::SigningNotRequiredForKey)?;
    }

    let msg_bytes = serialize_message(&tx.message);
    let sig = sign_message(key, &msg_bytes);

    // Ensure signatures array is exactly the size required by the message header
    let required_sigs = tx.message.header().num_required_signatures as usize;
    if tx.signatures.len() < required_sigs {
        tx.signatures
            .resize(required_sigs, Signature::from_bytes(&[0u8; 64]));
    }

    tx.signatures[signer_index] = sig;
    Ok(())
}
