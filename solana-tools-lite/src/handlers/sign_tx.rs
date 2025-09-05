use crate::models::input_transaction::{InputTransaction, UiTransaction};
use crate::{
    crypto::ed25519,
    errors::{Result, SignError},
    models::pubkey_base58::PubkeyBase58,
    models::transaction::Transaction,
    utils::serialize,
};

use ed25519_dalek::{Signature, SigningKey};

/// Pure handler: sign a UI input transaction with the given key and return UI transaction.
pub fn handle_sign_transaction(
    input_tx: InputTransaction,
    signing_key: &SigningKey,
) -> Result<UiTransaction> {
    let mut tx: Transaction = Transaction::try_from(input_tx)?;
    sign_transaction_by_key(&mut tx, signing_key)?;
    
    Ok(UiTransaction::from(&tx))
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
