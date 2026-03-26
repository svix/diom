use std::collections::HashMap;

use diom_authorization::{AccessPolicyId, AccessRule, RoleId};
use fjall_utils::{TableKey, TableRow};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

/// These values can never change. Only additions are allowed.
#[repr(u8)]
enum RowType {
    Role = 0,
    AccessPolicy = 1,
}

/// Primary row for a Role, keyed by `[ROW_TYPE][role_id_bytes]`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleRow {
    // FIXME: remove the id from this, we don't want it serialized.
    pub id: RoleId,
    pub description: String,
    pub rules: Vec<AccessRule>,
    pub policies: Vec<AccessPolicyId>,
    pub context: HashMap<String, String>,
    pub created: Timestamp,
    pub updated: Timestamp,
}

impl TableRow for RoleRow {
    const ROW_TYPE: u8 = RowType::Role as u8;
}

impl RoleRow {
    pub fn key_for(id: &RoleId) -> TableKey<Self> {
        TableKey::init_key(Self::ROW_TYPE, &[], &[id.as_str()])
    }
}

/// Primary row for an AccessPolicy, keyed by `[ROW_TYPE][policy_id_bytes]`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPolicyRow {
    // FIXME: remove the id from this, we don't want it serialized.
    pub id: AccessPolicyId,
    pub description: String,
    pub rules: Vec<AccessRule>,
    pub created: Timestamp,
    pub updated: Timestamp,
}

impl TableRow for AccessPolicyRow {
    const ROW_TYPE: u8 = RowType::AccessPolicy as u8;
}

impl AccessPolicyRow {
    pub fn key_for(id: &AccessPolicyId) -> TableKey<Self> {
        TableKey::init_key(Self::ROW_TYPE, &[], &[id.as_str()])
    }
}
