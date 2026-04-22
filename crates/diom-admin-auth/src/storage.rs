use std::collections::HashMap;

use diom_authorization::api::{AccessPolicyId, AccessRule, RoleId};
use diom_core::{PersistableValue, types::UnixTimestampMs};
use fjall_utils::{FjallKey, TableRow};
use serde::{Deserialize, Serialize};

/// These values can never change. Only additions are allowed.
#[repr(u8)]
enum RowType {
    Role = 0,
    AccessPolicy = 1,
}

/// Primary row for a Role, keyed by `[ROW_TYPE][role_id_bytes]`.
#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
pub struct RoleRow {
    // FIXME: remove the id from this, we don't want it serialized.
    pub id: RoleId,
    pub description: String,
    pub rules: Vec<AccessRule>,
    pub policies: Vec<AccessPolicyId>,
    pub context: HashMap<String, String>,
    pub created: UnixTimestampMs,
    pub updated: UnixTimestampMs,
}

impl TableRow for RoleRow {
    const ROW_TYPE: u8 = RowType::Role as u8;
}

#[derive(FjallKey)]
#[table_key(prefix = RowType::Role)]
pub(crate) struct RoleKey {
    #[key(0)]
    pub(crate) id: String,
}

/// Primary row for an AccessPolicy, keyed by `[ROW_TYPE][policy_id_bytes]`.
#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
pub struct AccessPolicyRow {
    pub description: String,
    pub rules: Vec<AccessRule>,
    pub created: UnixTimestampMs,
    pub updated: UnixTimestampMs,
}

impl TableRow for AccessPolicyRow {
    const ROW_TYPE: u8 = RowType::AccessPolicy as u8;
}

#[derive(FjallKey)]
#[table_key(prefix = RowType::AccessPolicy)]
pub(crate) struct AccessPolicyKey {
    #[key(0)]
    pub(crate) id: String,
}
