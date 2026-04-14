use crate::entities::TokenHashed;
use diom_core::{PersistableValue, types::Metadata};
use diom_error::Result;
use diom_id::{AuthTokenId, NamespaceId};
use fjall_utils::{TableKey, TableRow, WriteBatchExt};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

/// These values can never change. Only additions are allowed.
#[repr(u8)]
enum RowType {
    Token = 0,
    IdIndex = 1,
    OwnerIndex = 2,
}

/// Primary row. Keyed by (namespace_id, token_hashed) — the hot verify path.
#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
pub struct AuthTokenRow {
    pub id: AuthTokenId,
    pub name: String,
    pub expiry: Option<Timestamp>,
    pub metadata: Metadata,
    pub owner_id: String,
    pub scopes: Vec<String>,
    /// Whether this token is currently enabled.
    pub enabled: bool,
    pub created: Timestamp,
    pub updated: Timestamp,
}

impl TableRow for AuthTokenRow {
    const ROW_TYPE: u8 = RowType::Token as u8;
}

impl AuthTokenRow {
    pub fn key_for(namespace_id: NamespaceId, token_hashed: &TokenHashed) -> TableKey<Self> {
        TableKey::init_key(
            Self::ROW_TYPE,
            &[namespace_id.as_bytes(), token_hashed.inner()],
            &[],
        )
    }
}

/// Secondary index: maps (namespace_id, id) → token_hashed for ID-based lookups.
#[derive(Clone, Serialize, Deserialize, PersistableValue)]
pub struct IdIndexRow {
    pub token_hashed: TokenHashed,
}

impl TableRow for IdIndexRow {
    const ROW_TYPE: u8 = RowType::IdIndex as u8;
}

impl IdIndexRow {
    pub fn key_for(namespace_id: NamespaceId, id: AuthTokenId) -> TableKey<Self> {
        TableKey::init_key(
            Self::ROW_TYPE,
            &[namespace_id.as_bytes(), id.as_bytes()],
            &[],
        )
    }

    pub fn extract_token_hashed(value: fjall::UserValue) -> Result<TokenHashed> {
        let row = IdIndexRow::from_fjall_value(value)?;
        Ok(row.token_hashed)
    }
}

/// Secondary index: keyed by (namespace_id, owner_id, token_hashed) for owner listing.
/// Prefix-scan on (namespace_id, owner_id) to list all tokens for an owner.
#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
pub struct OwnerIndexRow {}

impl TableRow for OwnerIndexRow {
    const ROW_TYPE: u8 = RowType::OwnerIndex as u8;

    fn to_fjall_value(&self) -> Result<fjall::UserValue> {
        Ok(b"".into())
    }

    fn from_fjall_value(_value: fjall::UserValue) -> Result<Self> {
        Ok(Self {})
    }
}

impl OwnerIndexRow {
    /// Key layout: [ROW_TYPE(1)][namespace_id(16)][owner_id(var)]['\0'][token_id(16)][token_hashed(32)]
    pub fn key_for(
        namespace_id: NamespaceId,
        owner_id: &str,
        id: AuthTokenId,
        token_hashed: &TokenHashed,
    ) -> TableKey<Self> {
        let mut key =
            Vec::with_capacity(1 + size_of::<NamespaceId>() + owner_id.len() + 1 + 16 + 32);
        key.push(Self::ROW_TYPE);
        key.extend_from_slice(namespace_id.as_bytes());
        key.extend_from_slice(owner_id.as_bytes());
        key.push(b'\0');
        key.extend_from_slice(id.as_bytes());
        key.extend_from_slice(token_hashed.inner());
        TableKey::init_from_bytes(&key)
    }

    /// Returns the exclusive range-start key for scanning tokens after `after_id`.
    ///
    /// Using `Bound::Excluded` on this key skips the entry for `after_id` and
    /// starts at the next token_id.
    pub fn owner_iter_start(
        namespace_id: NamespaceId,
        owner_id: &str,
        after_id: AuthTokenId,
    ) -> Vec<u8> {
        let prefix = Self::owner_prefix(namespace_id, owner_id);
        let mut key = Vec::with_capacity(prefix.len() + 16 + 32);
        key.extend_from_slice(&prefix);
        key.extend_from_slice(after_id.as_bytes());
        key.extend(std::iter::repeat_n(0xFF, 32));
        key
    }

    /// Extracts `token_hashed` from the trailing 32 bytes of a raw fjall key.
    pub fn extract_token_hashed(key: &fjall::UserKey) -> Result<TokenHashed> {
        let data: &[u8] = key;
        let bytes: [u8; 32] = data
            .get(data.len().saturating_sub(32)..)
            .and_then(|s| s.try_into().ok())
            .ok_or_else(|| diom_error::Error::internal("malformed owner index key"))?;
        Ok(bytes.into())
    }

    /// Prefix used to scan all tokens for a given owner.
    pub fn owner_prefix(namespace_id: NamespaceId, owner_id: &str) -> Vec<u8> {
        let mut prefix = Vec::with_capacity(1 + size_of::<NamespaceId>() + owner_id.len() + 1);
        prefix.push(RowType::OwnerIndex as u8);
        prefix.extend_from_slice(namespace_id.as_bytes());
        prefix.extend_from_slice(owner_id.as_bytes());
        prefix.push(b'\0');
        prefix
    }
}

/// A complete auth token — primary row plus all index keys bundled together.
/// Use `upsert` or `remove` on a batch; never write individual rows directly.
pub(crate) struct AuthTokenEntity {
    pub namespace_id: NamespaceId,
    pub token_hashed: TokenHashed,
    pub row: AuthTokenRow,
}

impl AuthTokenEntity {
    pub(crate) fn upsert(
        &self,
        batch: &mut fjall::OwnedWriteBatch,
        ks: &fjall::Keyspace,
    ) -> Result<()> {
        batch.insert_row(
            ks,
            AuthTokenRow::key_for(self.namespace_id, &self.token_hashed),
            &self.row,
        )?;
        batch.insert_row(
            ks,
            IdIndexRow::key_for(self.namespace_id, self.row.id),
            &IdIndexRow {
                token_hashed: self.token_hashed.clone(),
            },
        )?;
        batch.insert_row(
            ks,
            OwnerIndexRow::key_for(
                self.namespace_id,
                &self.row.owner_id,
                self.row.id,
                &self.token_hashed,
            ),
            &OwnerIndexRow {},
        )?;
        Ok(())
    }

    pub(crate) fn remove(
        &self,
        batch: &mut fjall::OwnedWriteBatch,
        ks: &fjall::Keyspace,
    ) -> Result<()> {
        batch.remove_row(
            ks,
            AuthTokenRow::key_for(self.namespace_id, &self.token_hashed),
        )?;
        batch.remove_row(ks, IdIndexRow::key_for(self.namespace_id, self.row.id))?;
        batch.remove_row(
            ks,
            OwnerIndexRow::key_for(
                self.namespace_id,
                &self.row.owner_id,
                self.row.id,
                &self.token_hashed,
            ),
        )?;
        Ok(())
    }
}
