use std::convert::TryFrom;

use crate::models::hash_base58::HashBase58;
use crate::models::input_transaction::InputTransaction;
use crate::models::transaction::{Instruction, Message, Transaction};
use crate::models::pubkey_base58::PubkeyBase58;

use base64::{Engine, engine::general_purpose};
use bincode::config::standard;
use bincode::serde::decode_from_slice;

impl TryFrom<InputTransaction> for Transaction {
    type Error = String;

    fn try_from(input: InputTransaction) -> Result<Self, Self::Error> {
        match input {
            InputTransaction::Base64(s) => {
                let bytes = general_purpose::STANDARD
                    .decode(&s)
                    .map_err(|e| format!("Base64 decode error: {}", e))?;

                let config = standard().with_fixed_int_encoding();

                let (tx, _) = decode_from_slice::<Transaction, _>(&bytes, config)
                    .map_err(|e| format!("Deserialize error: {}", e))?; //TODO: custom?
                Ok(tx)
            }
            InputTransaction::Base58(s) => {
                let bytes = bs58::decode(&s)
                    .into_vec()
                    .map_err(|e| format!("Base58 decode error: {}", e))?;

                let config = standard().with_fixed_int_encoding();
               let (tx, _) = decode_from_slice::<Transaction, _>(&bytes, config)
                    .map_err(|e| format!("Deserialize error: {}", e))?; //TODO: custom?
                Ok(tx)
            }
            InputTransaction::Json(ui_tx) => {
                let message = Message {
                    header: ui_tx.message.header,
                    account_keys: ui_tx
                        .message
                        .account_keys
                        .into_iter()
                        //TODO: 🟡 rid of unwrap
                        .map(|s| PubkeyBase58::try_from(s.as_str()).unwrap())
                        .collect(),
                    recent_blockhash: HashBase58::try_from(ui_tx.message.recent_blockhash.as_str())
                        .unwrap(),
                    instructions: ui_tx
                        .message
                        .instructions
                        .into_iter()
                        .map(|i| Instruction {
                            program_id_index: i.program_id_index,
                            accounts: i.accounts,
                            data: bs58::decode(i.data).into_vec().unwrap(),
                        })
                        .collect(),
                };

                Ok(Transaction {
                    signatures: vec![],
                    message,
                })
            }
        }
    }
}
