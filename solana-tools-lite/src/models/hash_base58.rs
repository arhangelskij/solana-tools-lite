use serde::{Serialize};

#[derive(Clone, Copy)]
pub struct HashBase58(pub [u8; 32]);

impl Serialize for HashBase58 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        self.0.serialize(serializer)
    }
}

use serde::de::{self, Deserialize, Deserializer, Visitor};
use std::fmt;

impl<'de> Deserialize<'de> for HashBase58 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HashVisitor;

        impl<'de> Visitor<'de> for HashVisitor {
            type Value = HashBase58;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a byte array of length 32")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if v.len() != 32 {
                    return Err(E::invalid_length(v.len(), &self));
                }
                let mut array = [0u8; 32];
                array.copy_from_slice(v);
                Ok(HashBase58(array))
            }
        }

        deserializer.deserialize_bytes(HashVisitor)
    }
}

impl TryFrom<&str> for HashBase58 {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let bytes = bs58::decode(s).into_vec().map_err(|e| e.to_string())?;
        if bytes.len() != 32 {
            return Err(format!("Invalid length: expected 32, got {}", bytes.len()));
        }
        let mut array = [0u8; 32];
        array.copy_from_slice(&bytes);
        Ok(HashBase58(array))
    }
}

impl fmt::Display for HashBase58 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", bs58::encode(self.0).into_string())
    }
}

impl fmt::Debug for HashBase58 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HashBase58({})", self)
    }
}