use std::collections::HashMap;

use diom_authorization::{AccessPolicyId, AccessRule, RoleId};
use diom_core::task::spawn_blocking_in_current_span;
use diom_error::Result;
use fjall::{KeyspaceCreateOptions, KvSeparationOptions};
use fjall_utils::{TableRow, WriteBatchExt};
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

impl From<AccessPolicyRow> for AccessPolicyModel {
    fn from(row: AccessPolicyRow) -> Self {
        Self {
            id: row.id,
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

    pub async fn list_roles(&self) -> Result<Vec<RoleModel>> {
        let keyspace = self.keyspace.clone();
        spawn_blocking_in_current_span(move || {
            let models = RoleRow::values(&keyspace)?.map(RoleModel::from).collect();
            Ok(models)
        })
        .await?
    }

    pub async fn upsert_role(&self, input: UpsertRoleInput) -> Result<RoleModel> {
        let db = self.db.clone();
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
            let mut batch = db.batch();
            batch.insert_row(&keyspace, RoleRow::key_for(&row.id), &row)?;
            batch.commit()?;
            Ok(RoleModel::from(row))
        })
        .await?
    }

    pub async fn delete_role(&self, id: &RoleId) -> Result<bool> {
        let db = self.db.clone();
        let keyspace = self.keyspace.clone();
        let id = id.clone();
        spawn_blocking_in_current_span(move || {
            if RoleRow::fetch(&keyspace, RoleRow::key_for(&id))?.is_none() {
                return Ok(false);
            }
            let mut batch = db.batch();
            batch.remove_row(&keyspace, RoleRow::key_for(&id))?;
            batch.commit()?;
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
            Ok(row.map(AccessPolicyModel::from))
        })
        .await?
    }

    pub async fn list_policies(&self) -> Result<Vec<AccessPolicyModel>> {
        let keyspace = self.keyspace.clone();
        spawn_blocking_in_current_span(move || {
            let models = AccessPolicyRow::values(&keyspace)?
                .map(AccessPolicyModel::from)
                .collect();
            Ok(models)
        })
        .await?
    }

    pub async fn upsert_policy(&self, input: UpsertAccessPolicyInput) -> Result<AccessPolicyModel> {
        let db = self.db.clone();
        let keyspace = self.keyspace.clone();
        spawn_blocking_in_current_span(move || {
            let existing = AccessPolicyRow::fetch(&keyspace, AccessPolicyRow::key_for(&input.id))?;
            let created = existing.map(|r| r.created).unwrap_or(input.now);
            let row = AccessPolicyRow {
                id: input.id,
                description: input.description,
                rules: input.rules,
                created,
                updated: input.now,
            };
            let mut batch = db.batch();
            batch.insert_row(&keyspace, AccessPolicyRow::key_for(&row.id), &row)?;
            batch.commit()?;
            Ok(AccessPolicyModel::from(row))
        })
        .await?
    }

    pub async fn delete_policy(&self, id: &AccessPolicyId) -> Result<bool> {
        let db = self.db.clone();
        let keyspace = self.keyspace.clone();
        let id = id.clone();
        spawn_blocking_in_current_span(move || {
            if AccessPolicyRow::fetch(&keyspace, AccessPolicyRow::key_for(&id))?.is_none() {
                return Ok(false);
            }
            let mut batch = db.batch();
            batch.remove_row(&keyspace, AccessPolicyRow::key_for(&id))?;
            batch.commit()?;
            Ok(true)
        })
        .await?
    }
}
