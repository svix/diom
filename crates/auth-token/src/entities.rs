use std::fmt;

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use diom_error::Result;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const KEY_SIZE: usize = 24;

#[derive(Clone)]
pub struct TokenPlaintext(String);

impl TokenPlaintext {
    pub fn generate(prefix: &str, suffix: Option<&str>) -> Result<Self> {
        let mut buf = [0u8; KEY_SIZE];
        rand::RngCore::fill_bytes(&mut rand::rng(), &mut buf);

        let generated = URL_SAFE_NO_PAD.encode(buf);

        let suffix = suffix.unwrap_or("api");
        let plaintext = format!("{prefix}_{generated}.{suffix}");
        Ok(Self::from_plaintext_dangerous(plaintext))
    }

    pub fn from_plaintext_dangerous(pt: String) -> Self {
        Self(pt)
    }

    pub fn hash(&self) -> TokenHashed {
        self.0.as_str().into()
    }

    /// Exposes the plaintext. Ensure this value is only rendered to users when a token is created. This value should never be logged.
    pub fn expose_plaintext_dangerously(self) -> String {
        self.0
    }
}

/// A hashed auth token. The inner bytes are never printed to prevent accidental exposure.
#[derive(Clone, Serialize, Deserialize)]
pub struct TokenHashed([u8; 32]);

impl TokenHashed {
    pub fn inner(&self) -> &[u8; 32] {
        &self.0
    }
}

impl From<[u8; 32]> for TokenHashed {
    fn from(value: [u8; 32]) -> Self {
        Self(value)
    }
}

impl From<&str> for TokenHashed {
    fn from(value: &str) -> Self {
        Self(Sha256::digest(value.as_bytes()).into())
    }
}

impl fmt::Debug for TokenHashed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TokenHashed({})", URL_SAFE_NO_PAD.encode(self.0))
    }
}

impl TryFrom<&[u8]> for TokenHashed {
    type Error = diom_error::Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        let arr: [u8; 32] = bytes.try_into().map_err(|_| {
            diom_error::Error::internal(format!("expected 32 bytes, got {}", bytes.len()))
        })?;
        Ok(Self(arr))
    }
}
