pub use crate::errors::Bip39Error;
use bip39::{Language, Mnemonic, MnemonicType};
#[cfg(feature = "zeroize")]
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Convenience alias for BIP-39–scoped results.
pub type Bip39Result<T> = std::result::Result<T, Bip39Error>;

/// Default/new-type configuration for mnemonic generation.
/// Defaults to English, 12 words (Phantom/Solana-compatible happy path).
#[derive(Clone, Copy, Debug)]
pub struct Bip39Config {
    pub language: Language,
    /// Allowed: 12, 24
    pub word_count: usize,
}

impl Default for Bip39Config {
    fn default() -> Self {
        Self {
            language: Language::English,
            word_count: 12,
        }
    }
}

impl Bip39Config {
    pub fn validate(&self) -> Bip39Result<()> {
        validate_word_count(self.word_count).map(|_| ())
    }
}

fn validate_word_count(word_count: usize) -> Bip39Result<MnemonicType> {
    match word_count {
        12 => Ok(MnemonicType::Words12),
        24 => Ok(MnemonicType::Words24),
        _ => Err(Bip39Error::InvalidWordCount(word_count)),
    }
}

/// Parsed & normalized mnemonic wrapper to avoid repeated parsing.
#[derive(Clone, Debug)]
pub struct NormalizedMnemonic {
    inner: Mnemonic,
}

impl NormalizedMnemonic {
    pub fn phrase(&self) -> String {
        self.inner.to_string()
    }

    pub fn language(&self) -> Language {
        self.inner.language()
    }

    pub fn as_mnemonic(&self) -> &Mnemonic {
        &self.inner
    }
}

/// Parse and normalize a mnemonic string (English wordlist).
pub fn parse_mnemonic(phrase: &str) -> Bip39Result<NormalizedMnemonic> {
    let mnemonic = Mnemonic::from_phrase(phrase, Language::English)
        .map_err(|e| Bip39Error::Mnemonic(e.to_string()))?;

    // Also validate that the phrase length is either 12 or 24 words
    let word_count = phrase.split_whitespace().count();
    validate_word_count(word_count)?;

    Ok(NormalizedMnemonic { inner: mnemonic })
}

/// Generate a mnemonic with explicit config (language, word count).
pub fn generate_mnemonic_with(config: Bip39Config) -> Bip39Result<NormalizedMnemonic> {
    let m_type = validate_word_count(config.word_count)?;
    let mnemonic = Mnemonic::new(m_type, config.language);

    Ok(NormalizedMnemonic { inner: mnemonic })
}

/// Generate a random 12-word English BIP-39 mnemonic phrase.
pub fn generate_mnemonic() -> Bip39Result<String> {
    Ok(generate_mnemonic_with(Bip39Config::default())?.phrase())
}

/// Derive a 64-byte seed from a BIP-39 mnemonic phrase and passphrase.
pub fn derive_seed(phrase: &str, passphrase: &str) -> Bip39Result<Seed> {
    let mnemonic = parse_mnemonic(phrase)?;

    Ok(derive_seed_from_mnemonic(&mnemonic, passphrase))
}

/// Derive a 64-byte seed from a validated mnemonic and passphrase.
pub fn derive_seed_from_mnemonic(mnemonic: &NormalizedMnemonic, passphrase: &str) -> Seed {
    let b_seed = bip39::Seed::new(&mnemonic.inner, passphrase);
    let mut bytes = [0u8; 64];
    bytes.copy_from_slice(b_seed.as_bytes());
    Seed::new(bytes)
}

/// Validate a BIP-39 mnemonic phrase.
pub fn validate_mnemonic(phrase: &str) -> Bip39Result<()> {
    parse_mnemonic(phrase).map(|_| ())
}

/// 64-byte seed wrapper to make zeroing explicit and avoid leaking in Debug.
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct Seed([u8; 64]);

impl Seed {
    pub fn new(bytes: [u8; 64]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn into_inner(self) -> [u8; 64] {
        self.0
    }
}

impl AsRef<[u8]> for Seed {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl core::fmt::Debug for Seed {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Seed([REDACTED])")
    }
}
