use serde::{Deserialize, Serialize};

/// A single executable instruction within a Solana transaction.
///
/// An instruction specifies a program account, a list of accounts to be passed
/// to the program, and a data blob that serves as input arguments.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Instruction {
    /// Index of the program account in the message's `account_keys` list.
    pub program_id_index: u8,

    /// List of indices into the `account_keys` array for accounts that should
    /// be passed to the program during execution.
    #[serde(with = "solana_short_vec")]
    pub accounts: Vec<u8>,

    /// Opaque binary data passed as input to the program.
    /// In textual representations (e.g., JSON or CLI), this is typically
    /// Base58 or Base64 encoded.
    #[serde(with = "solana_short_vec")]
    pub data: Vec<u8>,
}
