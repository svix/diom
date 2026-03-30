use std::collections::HashMap;

use coyote_authorization::{AccessPolicyId, AccessRule, RoleId};
use coyote_error::{Result, ResultExt as _};
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

    pub fn decode_fjall_key(key: &fjall::UserKey) -> Result<AccessPolicyId> {
        assert!(key.len() >= 3);
        assert!(key[0] == Self::ROW_TYPE);

        let s = str::from_utf8(&key[1..key.len() - 1])
            .or_internal_error()?
            .to_owned();
        Ok(AccessPolicyId(s))
    }
}
