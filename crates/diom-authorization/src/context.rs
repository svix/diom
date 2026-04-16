use std::{collections::HashMap, sync::LazyLock};

use crate::Permissions;

const USER_CONTEXT_PREFIX: &str = "context.";

#[derive(Clone, Copy)]
pub struct Context<'a> {
    pub role: &'a str,
    pub map: &'a HashMap<String, String>,
}

impl<'a> Context<'a> {
    pub fn new(permissions: &'a Permissions) -> Self {
        Self {
            role: permissions.role.as_str(),
            map: &permissions.context,
        }
    }

    /// Create a test context with no useful data in it.
    pub fn empty_for_tests() -> Self {
        static EMPTY_MAP: LazyLock<HashMap<String, String>> = LazyLock::new(HashMap::new);
        Self {
            role: "__test_role",
            map: &EMPTY_MAP,
        }
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        if key == "role" {
            Some(self.role)
        } else if let Some(name) = key.strip_prefix(USER_CONTEXT_PREFIX) {
            self.map.get(name).map(|s| s.as_str())
        } else {
            None
        }
    }
}
