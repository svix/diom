use crate::entities::TokenHashed;
use diom_core::{task::spawn_blocking_in_current_span, types::Metadata};
use diom_error::Result;
use diom_id::{AuthTokenId, NamespaceId};
use fjall::KeyspaceCreateOptions;
use fjall_utils::TableRow;
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
    pub created: Timestamp,
    pub updated: Timestamp,
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
            created: row.created,
            updated: row.updated,
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

#[derive(Clone, Serialize, Deserialize)]
pub struct RotateTokenInput {
    pub new_id: AuthTokenId,
    pub new_token_hashed: TokenHashed,
    /// When the old token expires.
    pub old_expiry: Timestamp,
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
            let opts = KeyspaceCreateOptions::default();
            db.keyspace(keyspace_name, || opts).unwrap()
        };

        Self { db, keyspace }
    }

    fn fetch_by_hash_non_expired(
        keyspace: &fjall::Keyspace,
        namespace_id: NamespaceId,
        token_hashed: &TokenHashed,
        now: Timestamp,
    ) -> Result<Option<AuthTokenRow>> {
        let Some(row) =
            AuthTokenRow::fetch(keyspace, AuthTokenRow::key_for(namespace_id, token_hashed))?
        else {
            return Ok(None);
        };
        // FIXME: probably should throw an error instead.
        if let Some(existing_expiry) = row.expiry
            && existing_expiry <= now
        {
            return Ok(None);
        }

        Ok(Some(row))
    }

    fn fetch_by_id_non_expired(
        keyspace: &fjall::Keyspace,
        namespace_id: NamespaceId,
        id: AuthTokenId,
        now: Timestamp,
    ) -> Result<Option<(IdIndexRow, AuthTokenRow)>> {
        let Some(index_row) = IdIndexRow::fetch(keyspace, IdIndexRow::key_for(namespace_id, id))?
        else {
            return Ok(None);
        };
        let Some(row) =
            Self::fetch_by_hash_non_expired(keyspace, namespace_id, &index_row.token_hashed, now)?
        else {
            return Ok(None);
        };
        Ok(Some((index_row, row)))
    }

    /// Look up a token by its hash. Returns `None` if not found or expired.
    pub async fn fetch_non_expired(
        &self,
        namespace_id: NamespaceId,
        token_hashed: &TokenHashed,
        now: Timestamp,
    ) -> Result<Option<AuthTokenModel>> {
        let keyspace = self.keyspace.clone();
        let token_hashed = token_hashed.clone();
        spawn_blocking_in_current_span(move || {
            let model =
                Self::fetch_by_hash_non_expired(&keyspace, namespace_id, &token_hashed, now)?;
            Ok(model.map(|x| x.into()))
        })
        .await?
    }

    pub async fn create(
        &self,
        namespace_id: NamespaceId,
        input: CreateTokenInput,
    ) -> Result<AuthTokenModel> {
        let db = self.db.clone();
        let keyspace = self.keyspace.clone();

        spawn_blocking_in_current_span(move || {
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
                    created: input.now,
                    updated: input.now,
                },
            };
            let mut batch = db.batch();
            entity.upsert(&mut batch, &keyspace)?;
            batch.commit()?;
            Ok(entity.row.into())
        })
        .await?
    }

    fn expire_internal(
        batch: &mut fjall::OwnedWriteBatch,
        keyspace: &fjall::Keyspace,
        namespace_id: NamespaceId,
        id: AuthTokenId,
        expiry: Timestamp,
        now: Timestamp,
    ) -> Result<Option<AuthTokenModel>> {
        let Some((index_row, mut row)) =
            Self::fetch_by_id_non_expired(keyspace, namespace_id, id, now)?
        else {
            return Ok(None);
        };
        row.expiry = Some(expiry);
        row.updated = now;
        let entity = AuthTokenEntity {
            namespace_id,
            token_hashed: index_row.token_hashed,
            row,
        };
        entity.upsert(batch, keyspace)?;
        Ok(Some(entity.row.into()))
    }

    pub async fn expire(
        &self,
        namespace_id: NamespaceId,
        id: AuthTokenId,
        expiry: Timestamp,
        now: Timestamp,
    ) -> Result<Option<AuthTokenModel>> {
        let db = self.db.clone();
        let keyspace = self.keyspace.clone();
        spawn_blocking_in_current_span(move || {
            let mut batch = db.batch();
            let ret = Self::expire_internal(&mut batch, &keyspace, namespace_id, id, expiry, now)?;
            batch.commit()?;
            Ok(ret)
        })
        .await?
    }

    pub async fn delete(&self, namespace_id: NamespaceId, id: AuthTokenId) -> Result<bool> {
        let db = self.db.clone();
        let keyspace = self.keyspace.clone();
        spawn_blocking_in_current_span(move || {
            let Some(id_index) =
                IdIndexRow::fetch(&keyspace, IdIndexRow::key_for(namespace_id, id))?
            else {
                return Ok(false);
            };
            let Some(row) = AuthTokenRow::fetch(
                &keyspace,
                AuthTokenRow::key_for(namespace_id, &id_index.token_hashed),
            )?
            else {
                tracing::warn!(?id, ?id_index.token_hashed, "Found idx but not token. Cleaning up index.");
                keyspace.remove(IdIndexRow::key_for(namespace_id, id).into_fjall_key())?;
                return Ok(true);
            };
            let entity = AuthTokenEntity {
                namespace_id,
                token_hashed: id_index.token_hashed,
                row,
            };
            let mut batch = db.batch();
            entity.remove(&mut batch, &keyspace)?;
            batch.commit()?;
            Ok(true)
        })
        .await?
    }

    pub async fn list_by_owner(
        &self,
        namespace_id: NamespaceId,
        owner_id: &str,
        limit: usize,
        iterator: Option<AuthTokenId>,
    ) -> Result<Vec<AuthTokenModel>> {
        let keyspace = self.keyspace.clone();
        let owner_id = owner_id.to_owned();
        spawn_blocking_in_current_span(move || {
            let prefix = OwnerIndexRow::owner_prefix(namespace_id, &owner_id);
            let iterator =
                iterator.map(|id| OwnerIndexRow::owner_iter_start(namespace_id, &owner_id, id));
            let mut tokens = Vec::new();
            for (key, _) in OwnerIndexRow::list_range(&keyspace, &prefix, iterator, limit)? {
                let token_hashed = OwnerIndexRow::extract_token_hashed(&key)?;
                match AuthTokenRow::fetch(
                    &keyspace,
                    AuthTokenRow::key_for(namespace_id, &token_hashed),
                )? {
                    Some(row) => tokens.push(row.into()),
                    None => tracing::warn!("Skipping missing owner."),
                }
            }
            Ok(tokens)
        })
        .await?
    }

    /// Create a new token copied from an existing one, and expire the old token.
    /// Returns `None` if the original token was not found.
    pub async fn rotate(
        &self,
        namespace_id: NamespaceId,
        old_id: AuthTokenId,
        input: RotateTokenInput,
    ) -> Result<Option<AuthTokenModel>> {
        let db = self.db.clone();
        let keyspace = self.keyspace.clone();
        spawn_blocking_in_current_span(move || {
            let Some((_old_id_index, old_row)) =
                Self::fetch_by_id_non_expired(&keyspace, namespace_id, old_id, input.now)?
            else {
                return Ok(None);
            };

            let new_entity = AuthTokenEntity {
                namespace_id,
                token_hashed: input.new_token_hashed,
                row: AuthTokenRow {
                    id: input.new_id,
                    name: old_row.name.clone(),
                    expiry: old_row.expiry,
                    metadata: old_row.metadata.clone(),
                    owner_id: old_row.owner_id.clone(),
                    scopes: old_row.scopes.clone(),
                    enabled: old_row.enabled,
                    created: input.now,
                    updated: input.now,
                },
            };
            let mut batch = db.batch();
            Self::expire_internal(
                &mut batch,
                &keyspace,
                namespace_id,
                old_row.id,
                input.old_expiry,
                input.now,
            )?;
            new_entity.upsert(&mut batch, &keyspace)?;
            batch.commit()?;
            Ok(Some(new_entity.row.into()))
        })
        .await?
    }

    pub async fn partial_update(
        &self,
        namespace_id: NamespaceId,
        id: AuthTokenId,
        input: PartialUpdateInput,
    ) -> Result<Option<AuthTokenModel>> {
        let db = self.db.clone();
        let keyspace = self.keyspace.clone();
        spawn_blocking_in_current_span(move || {
            let Some((id_index, mut row)) =
                Self::fetch_by_id_non_expired(&keyspace, namespace_id, id, input.now)?
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
            row.updated = now;
            let entity = AuthTokenEntity {
                namespace_id,
                token_hashed: id_index.token_hashed,
                row,
            };
            let mut batch = db.batch();
            entity.upsert(&mut batch, &keyspace)?;
            batch.commit()?;
            Ok(Some(entity.row.into()))
        })
        .await?
    }
}

#[allow(clippy::disallowed_methods)]
#[cfg(test)]
mod tests {
    use diom_id::{NamespaceId, UuidV7RandomBytes};
    use fjall::Database;
    use jiff::{Timestamp, ToSpan};

    use super::*;
    use crate::entities::TokenPlaintext;

    struct Fixture {
        _workdir: tempfile::TempDir,
        controller: AuthTokenController,
    }

    impl Fixture {
        fn new() -> Self {
            let workdir = tempfile::tempdir().unwrap();
            let db = Database::builder(workdir.as_ref())
                .temporary(true)
                .open()
                .unwrap();
            let controller = AuthTokenController::new(db, "test_auth_tokens");
            Self {
                _workdir: workdir,
                controller,
            }
        }
    }

    fn ns() -> NamespaceId {
        NamespaceId::nil()
    }

    async fn create_token(
        controller: &AuthTokenController,
        name: &str,
        owner_id: &str,
        ts: Timestamp,
        random_bytes: UuidV7RandomBytes,
    ) -> AuthTokenModel {
        let token = TokenPlaintext::generate("sk", None).unwrap();
        let input = CreateTokenInput {
            id: AuthTokenId::new(ts, random_bytes),
            name: name.to_string(),
            token_hashed: token.hash(),
            expiry: None,
            metadata: Default::default(),
            owner_id: owner_id.to_string(),
            scopes: vec![],
            enabled: true,
            now: ts,
        };
        controller.create(ns(), input).await.unwrap()
    }

    #[tokio::test]
    async fn test_list_sorted_and_iterator() {
        let fx = Fixture::new();
        let c = &fx.controller;
        let owner = "owner1";

        // Create 5 tokens at spread-out timestamps to get stable UUIDv7 ordering.
        let base = Timestamp::UNIX_EPOCH;
        let mut created = Vec::new();
        for i in 0..5i32 {
            let ts = base.checked_add((i + 1).seconds()).unwrap();
            created.push(
                create_token(
                    c,
                    &format!("token-{i}"),
                    owner,
                    ts,
                    UuidV7RandomBytes::new_random(),
                )
                .await,
            );
        }
        created.sort_by_key(|t| *t.id.as_bytes());

        // All 5 returned in ID order when limit is generous.
        let page = c.list_by_owner(ns(), owner, 10, None).await.unwrap();
        assert_eq!(
            page.iter().map(|t| t.id).collect::<Vec<_>>(),
            created.iter().map(|t| t.id).collect::<Vec<_>>()
        );

        // Limit is respected.
        let page = c.list_by_owner(ns(), owner, 3, None).await.unwrap();
        assert_eq!(page.len(), 3);
        assert_eq!(page[0].id, created[0].id);

        // Iterator skips up to and including the given ID.
        let page = c
            .list_by_owner(ns(), owner, 10, Some(created[1].id))
            .await
            .unwrap();
        assert_eq!(page.len(), 3);
        assert_eq!(page[0].id, created[2].id);
        assert_eq!(page[2].id, created[4].id);

        // Unknown iterator returns empty.
        let page = c
            .list_by_owner(ns(), owner, 10, Some(AuthTokenId::max()))
            .await
            .unwrap();
        assert!(page.is_empty());
    }
}
