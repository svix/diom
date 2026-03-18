use crate::entities::TokenHashed;
use coyote_core::types::Metadata;
use coyote_error::Result;
use coyote_id::{AuthTokenId, NamespaceId};
use fjall::{KeyspaceCreateOptions, KvSeparationOptions};
use fjall_utils::{TableRow, WriteBatchExt};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::tables::{AuthTokenEntity, AuthTokenRow, IdIndexRow, OwnerIndexRow};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthTokenModel {
    pub id: AuthTokenId,
    pub name: String,
    pub expiry: Option<Timestamp>,
    pub metadata: Metadata,
    pub owner_id: String,
    pub scopes: Vec<String>,
    /// Whether this token is currently enabled.
    pub enabled: bool,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl From<AuthTokenRow> for AuthTokenModel {
    fn from(row: AuthTokenRow) -> Self {
        Self {
            id: row.id,
            name: row.name,
            expiry: row.expiry,
            metadata: row.metadata,
            owner_id: row.owner_id,
            scopes: row.scopes,
            enabled: row.enabled,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CreateTokenInput {
    pub id: AuthTokenId,
    pub name: String,
    pub token_hashed: TokenHashed,
    pub expiry: Option<Timestamp>,
    pub metadata: Metadata,
    pub owner_id: String,
    pub scopes: Vec<String>,
    pub enabled: bool,
    pub now: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialUpdateInput {
    pub name: Option<String>,
    pub expiry: Option<Timestamp>,
    pub metadata: Option<Metadata>,
    pub scopes: Option<Vec<String>>,
    pub enabled: Option<bool>,
    pub now: Timestamp,
}

#[derive(Clone)]
pub struct AuthTokenController {
    pub(crate) db: fjall::Database,
    pub(crate) keyspace: fjall::Keyspace,
}

impl AuthTokenController {
    pub fn new(db: fjall::Database, keyspace_name: &'static str) -> Self {
        let keyspace = {
            let opts = KeyspaceCreateOptions::default()
                .with_kv_separation(Some(KvSeparationOptions::default()));
            db.keyspace(keyspace_name, || opts).unwrap()
        };

        Self { db, keyspace }
    }

    pub(crate) fn fetch_by_hash(
        &self,
        namespace_id: NamespaceId,
        token_hashed: &TokenHashed,
    ) -> Result<Option<AuthTokenModel>> {
        let Some(row) = AuthTokenRow::fetch(
            &self.keyspace,
            AuthTokenRow::key_for(namespace_id, token_hashed),
        )?
        else {
            return Ok(None);
        };
        Ok(Some(row.into()))
    }

    /// Look up a token by its hash. Returns `None` if not found or expired.
    pub fn fetch_non_expired(
        &self,
        namespace_id: NamespaceId,
        token_hashed: &TokenHashed,
        now: Timestamp,
    ) -> Result<Option<AuthTokenModel>> {
        let token = self.fetch_by_hash(namespace_id, token_hashed)?;
        if let Some(token) = &token
            && let Some(expiry) = token.expiry
            && expiry <= now
        {
            return Ok(None);
        }
        Ok(token)
    }

    pub fn create(
        &self,
        namespace_id: NamespaceId,
        input: CreateTokenInput,
    ) -> Result<AuthTokenModel> {
        let entity = AuthTokenEntity {
            namespace_id,
            token_hashed: input.token_hashed,
            row: AuthTokenRow {
                id: input.id,
                name: input.name,
                expiry: input.expiry,
                metadata: input.metadata,
                owner_id: input.owner_id,
                scopes: input.scopes,
                enabled: input.enabled,
                created_at: input.now,
                updated_at: input.now,
            },
        };
        let mut batch = self.db.batch();
        entity.upsert(&mut batch, &self.keyspace)?;
        batch.commit()?;
        Ok(entity.row.into())
    }

    pub fn expire(
        &self,
        namespace_id: NamespaceId,
        id: AuthTokenId,
        expiry: Timestamp,
        now: Timestamp,
    ) -> Result<Option<AuthTokenModel>> {
        let Some(index_row) =
            IdIndexRow::fetch(&self.keyspace, IdIndexRow::key_for(namespace_id, id))?
        else {
            return Ok(None);
        };
        let Some(mut row) = AuthTokenRow::fetch(
            &self.keyspace,
            AuthTokenRow::key_for(namespace_id, &index_row.token_hashed),
        )?
        else {
            return Ok(None);
        };
        if let Some(existing_expiry) = row.expiry
            && existing_expiry <= now
        {
            return Ok(None);
        }
        row.expiry = Some(expiry);
        let entity = AuthTokenEntity {
            namespace_id,
            token_hashed: index_row.token_hashed,
            row,
        };
        let mut batch = self.db.batch();
        entity.upsert(&mut batch, &self.keyspace)?;
        batch.commit()?;
        Ok(Some(entity.row.into()))
    }

    pub fn delete(&self, namespace_id: NamespaceId, id: AuthTokenId) -> Result<bool> {
        let Some(id_index) =
            IdIndexRow::fetch(&self.keyspace, IdIndexRow::key_for(namespace_id, id))?
        else {
            return Ok(false);
        };
        let Some(row) = AuthTokenRow::fetch(
            &self.keyspace,
            AuthTokenRow::key_for(namespace_id, &id_index.token_hashed),
        )?
        else {
            tracing::warn!(?id, ?id_index.token_hashed, "Found idx but not token. Cleaning up index.");
            let mut batch = self.db.batch();
            batch.remove_row(&self.keyspace, IdIndexRow::key_for(namespace_id, id))?;
            batch.commit()?;
            return Ok(true);
        };
        let entity = AuthTokenEntity {
            namespace_id,
            token_hashed: id_index.token_hashed,
            row,
        };
        let mut batch = self.db.batch();
        entity.remove(&mut batch, &self.keyspace)?;
        batch.commit()?;
        Ok(true)
    }

    pub fn list_by_owner(
        &self,
        namespace_id: NamespaceId,
        owner_id: &str,
    ) -> Result<Vec<AuthTokenModel>> {
        let prefix = OwnerIndexRow::owner_prefix(namespace_id, owner_id);
        let mut tokens = Vec::new();
        for item in self.keyspace.prefix(&prefix) {
            let key = item.key()?;
            let token_hashed = OwnerIndexRow::extract_token_hashed(&key)?;
            if let Some(row) = AuthTokenRow::fetch(
                &self.keyspace,
                AuthTokenRow::key_for(namespace_id, &token_hashed),
            )? {
                tokens.push(row.into());
            } else {
                tracing::warn!("Skipping missing owner.");
            }
        }
        Ok(tokens)
    }

    pub fn partial_update(
        &self,
        namespace_id: NamespaceId,
        id: AuthTokenId,
        input: PartialUpdateInput,
    ) -> Result<Option<AuthTokenModel>> {
        let Some(id_index) =
            IdIndexRow::fetch(&self.keyspace, IdIndexRow::key_for(namespace_id, id))?
        else {
            return Ok(None);
        };
        let Some(mut row) = AuthTokenRow::fetch(
            &self.keyspace,
            AuthTokenRow::key_for(namespace_id, &id_index.token_hashed),
        )?
        else {
            return Ok(None);
        };
        let PartialUpdateInput {
            name,
            expiry,
            metadata,
            scopes,
            enabled,
            now,
        } = input;
        if let Some(name) = name {
            row.name = name;
        }
        if let Some(expiry) = expiry {
            row.expiry = Some(expiry);
        }
        if let Some(metadata) = metadata {
            row.metadata = metadata;
        }
        if let Some(scopes) = scopes {
            row.scopes = scopes;
        }
        if let Some(enabled) = enabled {
            row.enabled = enabled;
        }
        row.updated_at = now;
        let entity = AuthTokenEntity {
            namespace_id,
            token_hashed: id_index.token_hashed,
            row,
        };
        let mut batch = self.db.batch();
        entity.upsert(&mut batch, &self.keyspace)?;
        batch.commit()?;
        Ok(Some(entity.row.into()))
    }
}
