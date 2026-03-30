use std::collections::HashMap;

use coyote_authorization::{AccessPolicyId, AccessRule, RoleId};
use coyote_core::task::spawn_blocking_in_current_span;
use coyote_error::Result;
use fjall::{KeyspaceCreateOptions, KvSeparationOptions};
use fjall_utils::TableRow;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::tables::{AccessPolicyRow, RoleRow};

// ── Role model ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleModel {
    pub id: RoleId,
    pub description: String,
    pub rules: Vec<AccessRule>,
    pub policies: Vec<AccessPolicyId>,
    pub context: HashMap<String, String>,
    pub created: Timestamp,
    pub updated: Timestamp,
}

impl From<RoleRow> for RoleModel {
    fn from(row: RoleRow) -> Self {
        Self {
            id: row.id,
            description: row.description,
            rules: row.rules,
            policies: row.policies,
            context: row.context,
            created: row.created,
            updated: row.updated,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UpsertRoleInput {
    pub id: RoleId,
    pub description: String,
    pub rules: Vec<AccessRule>,
    pub policies: Vec<AccessPolicyId>,
    pub context: HashMap<String, String>,
    pub now: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPolicyModel {
    pub id: AccessPolicyId,
    pub description: String,
    pub rules: Vec<AccessRule>,
    pub created: Timestamp,
    pub updated: Timestamp,
}
impl AccessPolicyModel {
    fn new(id: AccessPolicyId, row: AccessPolicyRow) -> Self {
        Self {
            id,
            description: row.description,
            rules: row.rules,
            created: row.created,
            updated: row.updated,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UpsertAccessPolicyInput {
    pub id: AccessPolicyId,
    pub description: String,
    pub rules: Vec<AccessRule>,
    pub now: Timestamp,
}

#[derive(Clone)]
pub struct AdminAuthController {
    #[expect(unused)] // until we add some db.batch() operations
    pub(crate) db: fjall::Database,
    pub(crate) keyspace: fjall::Keyspace,
}

impl AdminAuthController {
    pub fn new(db: fjall::Database, keyspace_name: &'static str) -> Self {
        let keyspace = {
            let opts = KeyspaceCreateOptions::default()
                .with_kv_separation(Some(KvSeparationOptions::default()));
            db.keyspace(keyspace_name, || opts).unwrap()
        };
        Self { db, keyspace }
    }

    // Role CRUD

    pub async fn get_role(&self, id: &RoleId) -> Result<Option<RoleModel>> {
        let keyspace = self.keyspace.clone();
        let id = id.clone();
        spawn_blocking_in_current_span(move || {
            let row = RoleRow::fetch(&keyspace, RoleRow::key_for(&id))?;
            Ok(row.map(RoleModel::from))
        })
        .await?
    }

    pub async fn list_roles(
        &self,
        limit: usize,
        start_after: Option<String>,
    ) -> Result<Vec<RoleModel>> {
        let keyspace = self.keyspace.clone();
        spawn_blocking_in_current_span(move || {
            // FIXME: actually use the iterator on fjall (like we do in auth_token) rather than
            // doing it in rust.
            let models = RoleRow::values(&keyspace)?
                .map(RoleModel::from)
                .filter(|m| start_after.as_deref().is_none_or(|s| m.id.as_str() > s))
                .take(limit)
                .collect();
            Ok(models)
        })
        .await?
    }

    pub async fn upsert_role(&self, input: UpsertRoleInput) -> Result<RoleModel> {
        let keyspace = self.keyspace.clone();
        spawn_blocking_in_current_span(move || {
            let existing = RoleRow::fetch(&keyspace, RoleRow::key_for(&input.id))?;
            let created = existing.map(|r| r.created).unwrap_or(input.now);
            let row = RoleRow {
                id: input.id,
                description: input.description,
                rules: input.rules,
                policies: input.policies,
                context: input.context,
                created,
                updated: input.now,
            };
            keyspace.insert(
                RoleRow::key_for(&row.id).into_fjall_key(),
                row.to_fjall_value()?,
            )?;
            Ok(RoleModel::from(row))
        })
        .await?
    }

    pub async fn delete_role(&self, id: &RoleId) -> Result<bool> {
        let keyspace = self.keyspace.clone();
        let id = id.clone();
        spawn_blocking_in_current_span(move || {
            if RoleRow::fetch(&keyspace, RoleRow::key_for(&id))?.is_none() {
                return Ok(false);
            }
            keyspace.remove(RoleRow::key_for(&id).into_fjall_key())?;
            Ok(true)
        })
        .await?
    }

    // Policy CRUD

    pub async fn get_policy(&self, id: &AccessPolicyId) -> Result<Option<AccessPolicyModel>> {
        let keyspace = self.keyspace.clone();
        let id = id.clone();
        spawn_blocking_in_current_span(move || {
            let row = AccessPolicyRow::fetch(&keyspace, AccessPolicyRow::key_for(&id))?;
            Ok(row.map(|r| AccessPolicyModel::new(id, r)))
        })
        .await?
    }

    pub async fn list_policies(
        &self,
        limit: usize,
        start_after: Option<String>,
    ) -> Result<Vec<AccessPolicyModel>> {
        let keyspace = self.keyspace.clone();
        spawn_blocking_in_current_span(move || {
            let mut models = Vec::new();

            for (key, row) in AccessPolicyRow::iter(&keyspace) {
                let id = AccessPolicyRow::decode_fjall_key(&key)?;
                let model = AccessPolicyModel::new(id, row);

                // FIXME: actually use the iterator on fjall (like we do in auth_token) rather than
                // doing it in rust.
                if start_after
                    .as_deref()
                    .is_some_and(|s| s > model.id.as_str())
                {
                    continue;
                }

                models.push(model);
                if models.len() >= limit {
                    break;
                }
            }

            Ok(models)
        })
        .await?
    }

    pub async fn upsert_policy(&self, input: UpsertAccessPolicyInput) -> Result<AccessPolicyModel> {
        let keyspace = self.keyspace.clone();
        spawn_blocking_in_current_span(move || {
            let existing = AccessPolicyRow::fetch(&keyspace, AccessPolicyRow::key_for(&input.id))?;
            let created = existing.map(|r| r.created).unwrap_or(input.now);
            let row = AccessPolicyRow {
                description: input.description,
                rules: input.rules,
                created,
                updated: input.now,
            };
            keyspace.insert(
                AccessPolicyRow::key_for(&input.id).into_fjall_key(),
                row.to_fjall_value()?,
            )?;
            Ok(AccessPolicyModel::new(input.id, row))
        })
        .await?
    }

    pub async fn delete_policy(&self, id: &AccessPolicyId) -> Result<bool> {
        let keyspace = self.keyspace.clone();
        let id = id.clone();
        spawn_blocking_in_current_span(move || {
            if AccessPolicyRow::fetch(&keyspace, AccessPolicyRow::key_for(&id))?.is_none() {
                return Ok(false);
            }
            keyspace.remove(AccessPolicyRow::key_for(&id).into_fjall_key())?;
            Ok(true)
        })
        .await?
    }
}
