use std::convert::TryFrom;

use crate::models::hash_base58::HashBase58;
use crate::models::input_transaction::{InputTransaction, UiTransaction};
use crate::models::pubkey_base58::PubkeyBase58;
use crate::models::transaction::{Instruction, Message, Transaction};

use crate::errors::TransactionParseError;
use data_encoding::BASE64;
use bs58;
use serde_json;
use ed25519_dalek::Signature;
//TODO: naming
use crate::deserializer::deserialize_transaction as deserialize_tx_bytes;

impl TryFrom<InputTransaction> for Transaction {
    type Error = TransactionParseError;

    fn try_from(input: InputTransaction) -> Result<Self, Self::Error> {
        match input {
            InputTransaction::Base64(s) => {
                // Decode Base64-encoded raw Solana transaction bytes
                let raw = BASE64
                    .decode(s.as_bytes())
                    .map_err(|e| TransactionParseError::InvalidBase64(e.to_string()))?;

                // Use our custom deserializer instead of bincode::Decode
                let tx = deserialize_tx_bytes(&raw)
                    .map_err(|e| TransactionParseError::InvalidFormat(e.to_string()))?;

                Ok(tx)
            }
            InputTransaction::Base58(s) => {
                // Decode Base58-encoded JSON
                let decoded = bs58::decode(s)
                    .into_vec()
                    .map_err(|e| TransactionParseError::InvalidFormat(e.to_string()))?;
                //TODO: 🔴 1) here must be bincode deserialize!
                let ui_tx: UiTransaction = serde_json::from_slice(&decoded)
                    .map_err(|e| TransactionParseError::InvalidFormat(e.to_string()))?;
                Transaction::try_from(ui_tx)
            }
            InputTransaction::Json(ui_tx) => {
                let message = Message {
                    header: ui_tx.message.header,
                    account_keys: ui_tx
                        .message
                        .account_keys
                        .into_iter()
                        //TODO: 🟡 rid of unwrap
                        .map(|s| {
                            PubkeyBase58::try_from(s.as_str()).map_err(|e| {
                                TransactionParseError::InvalidPubkeyFormat(e.to_string())
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                    recent_blockhash: HashBase58::try_from(ui_tx.message.recent_blockhash.as_str())
                        .unwrap(),
                    instructions: ui_tx
                        .message
                        .instructions
                        .into_iter()
                        .map(|i| -> Result<Instruction, TransactionParseError> {
                            let data = bs58::decode(&i.data)
                                .into_vec()
                                .map_err(|e| TransactionParseError::InvalidInstructionData(e.to_string()))?;

                            Ok(Instruction {
                                program_id_index: i.program_id_index,
                                accounts: i.accounts,
                                data,
                            })
                        })
                        .collect::<Result<Vec<Instruction>, TransactionParseError>>()?,
                };

                let signatures = ui_tx
                    .signatures
                    .into_iter()
                    .map(|s| -> Result<Signature, TransactionParseError> {
                        let bytes = bs58::decode(&s)
                            .into_vec()
                            .map_err(|e| TransactionParseError::InvalidSignatureFormat(e.to_string()))?;
//TODO: check it
                        let sig_bytes: [u8; 64] = bytes
                            .as_slice()
                            .try_into()
                            .map_err(|_| TransactionParseError::InvalidSignatureLength(bytes.len()))?;

                        Signature::try_from(&sig_bytes[..])
                            .map_err(|e| TransactionParseError::InvalidSignatureFormat(e.to_string()))
                    })
                    .collect::<Result<Vec<Signature>, TransactionParseError>>()?;

                Ok(Transaction {
                    signatures: signatures,
                    message,
                })
            }
        }
    }
}