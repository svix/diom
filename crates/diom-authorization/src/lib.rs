use std::{collections::HashMap, sync::Arc};

use diom_id::{AuthTokenId, Module};

pub mod api;
mod context;
mod pattern;
mod verification;

use self::api::{AccessRule, RoleId};

pub use self::{
    context::Context,
    pattern::{KeyPattern, KeyPatternSegment, ModulePattern, NamespacePattern, ResourcePattern},
    verification::{Forbidden, verify_operation},
};

#[derive(Debug)]
pub struct RequestedOperation<'a> {
    pub module: Module,
    pub namespace: Option<&'a str>,
    pub key: Option<&'a str>,
    pub action: &'static str,
}

impl RequestedOperation<'_> {
    pub fn resource_str(&self) -> String {
        let module = self.module;
        let namespace = self.namespace.unwrap_or("");
        let key = self.key.unwrap_or("*");
        format!("{module}:{namespace}:{key}")
    }
}

/// The `Permissions` for a request
#[derive(Clone)]
pub struct Permissions {
    /// The role of the requester
    pub role: RoleId,
    /// The auth token id, if we used auth token
    pub auth_token_id: Option<AuthTokenId>,
    /// The access rules of the requester's role
    pub access_rules: Arc<[AccessRule]>,
    /// Arbitrary key-value context forwarded from JWT claims (empty for non-JWT auth)
    pub context: HashMap<String, String>,
}

impl Permissions {
    /// Returns the builtin `admin` permissions.
    ///
    /// This constructor must only be used for requests that authenticate with
    /// the global admin token.
    pub fn admin() -> Self {
        Self {
            role: RoleId::admin(),
            auth_token_id: None,
            access_rules: AccessRule::admin_rules(),
            context: HashMap::new(),
        }
    }

    /// Returns the builtin `operator` permissions.
    ///
    /// This constructor must only be used for requests made through the
    /// internal self-call server.
    pub fn operator() -> Self {
        Self {
            role: RoleId::operator(),
            auth_token_id: None,
            access_rules: AccessRule::operator_rules(),
            context: HashMap::new(),
        }
    }
}
