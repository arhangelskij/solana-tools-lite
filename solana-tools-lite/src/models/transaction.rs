use serde::{Serialize, Deserialize};
use crate::models:: {
    hash_base58::HashBase58,
    pubkey_base58::PubkeyBase58
};

/// Represents a full Solana transaction, including all signatures and the serialized message.
/// Signatures may be empty for unsigned transactions (for cold signing).
#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    /// Array of base58-encoded signatures (one for each required signer).
    /// For unsigned TX, this can be empty or contain empty strings.
    #[serde(with = "serde_signature_base58")]
    pub signatures: Vec<Signature>,
    /// The actual message (to be signed): contains accounts, recent blockhash, and instructions.
    pub message: Message
}

/// Solana transaction message — the payload to be signed.
/// This must be serialized in a canonical format before signing!
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub header: MessageHeader,

    /// List of all account addresses (base58).
    #[serde(with = "solana_short_vec")]
    pub account_keys: Vec<PubkeyBase58>,

    /// Recent blockhash as base58 string (used for replay protection).
    pub recent_blockhash: HashBase58,

    /// List of instructions — each instruction defines a program call (e.g., transfer, mint).
    #[serde(with = "solana_short_vec")]
    pub instructions: Vec<Instruction>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MessageHeader {
    #[serde(alias = "numRequiredSignatures")]
    pub num_required_signatures: u8,
    #[serde(alias = "numReadonlySignedAccounts")]
    pub num_readonly_signed_accounts: u8,
    #[serde(alias = "numReadonlyUnsignedAccounts")]
    pub num_readonly_unsigned_accounts: u8
}

/// Solana instruction — represents a single call to a smart contract/program.
#[derive(Debug, Serialize, Deserialize)]
pub struct Instruction {
    /// Index of the program in the account_keys array.
    pub program_id_index: u8,

    /// List of indices of the involved accounts (in account_keys array).
    #[serde(with = "solana_short_vec")]
    pub accounts: Vec<u8>,

    /// Instruction data, base58 or base64 encoded (depends on source, but base58 is common in Solana).
    #[serde(with = "solana_short_vec")]
    pub data: Vec<u8>
}

//////////////// TODO: move into separate file
/// 
use ed25519_dalek::Signature;

mod serde_signature_base58 {
    use super::*;
    use serde::{Serializer, Deserializer, Deserialize};

    pub fn serialize<S>(sigs: &Vec<Signature>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let encoded: Vec<String> = sigs
            .iter()
            .map(|s| bs58::encode(s.to_bytes()).into_string())
            .collect();
        encoded.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Signature>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let encoded: Vec<String> = Deserialize::deserialize(deserializer)?;
        encoded
            .into_iter()
            .map(|s| {
                let bytes = bs58::decode(&s).into_vec().map_err(serde::de::Error::custom)?;
                if bytes.len() != 64 {
                    return Err(serde::de::Error::custom("Invalid signature length"));
                }
                let mut raw = [0u8; 64];
                raw.copy_from_slice(&bytes);
                Ok(Signature::from_bytes(&raw))
            })
            .collect()
    }
}