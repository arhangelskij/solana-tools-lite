
use serde::{Deserialize};
use crate::errors::TransactionParseError;

    //TODO: 🟡 check and use it
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct PubkeyBase58(pub [u8; 32]); // можно сделать обертку для блокхэщ и аккаунт кейс
//TODO: 🟡 Если нужно расширить, можно добавить impl From<PubkeyBase58> for Pubkey позже.
use std::convert::TryFrom;
use bs58;

impl TryFrom<&str> for PubkeyBase58 {
    type Error = crate::errors::ToolError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let decoded = bs58::decode(s)
            .into_vec()
            .map_err(|e| TransactionParseError::InvalidPubkeyFormat(e.to_string()))?;

        if decoded.len() != 32 {
            return Err(TransactionParseError::InvalidSignatureLength(decoded.len()))?;
        }
        

        let mut array = [0u8; 32];
        array.copy_from_slice(&decoded);
        Ok(PubkeyBase58(array))
    }
}

use serde::{Deserializer};
impl<'de> Deserialize<'de> for PubkeyBase58 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        PubkeyBase58::try_from(s.as_str()).map_err(serde::de::Error::custom)
    }
}

impl From<[u8; 32]> for PubkeyBase58 {
    fn from(bytes: [u8; 32]) -> Self {
        PubkeyBase58(bytes)
    }
}