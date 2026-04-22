use std::{borrow::Cow, fmt, str::FromStr};

use diom_core::PersistableValue;
use diom_id::Module;
use itertools::Itertools as _;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    Context, RequestedOperation,
    pattern::{KeyPattern, NamespacePattern},
};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema, PersistableValue)]
#[serde(transparent)]
pub struct RoleId(pub String);

impl RoleId {
    pub fn admin() -> Self {
        Self("admin".to_owned())
    }

    /// Role used by requests to the internal API server.
    ///
    /// Might be split into multiple roles down the line.
    pub fn operator() -> Self {
        Self("operator".to_owned())
    }

    pub fn from_string(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RoleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema, PersistableValue)]
#[serde(transparent)]
pub struct AccessPolicyId(pub String);

impl AccessPolicyId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AccessPolicyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, JsonSchema, PersistableValue)]
pub struct AccessRule {
    pub effect: AccessRuleEffect,
    pub resource: ResourcePattern,
    pub actions: Vec<String>,
}

impl AccessRule {
    pub fn uses_reserved_namespace(&self) -> bool {
        self.resource.namespace.is_reserved()
    }
}

// Unfortunately it is currently quite easy to create an AccessRule in code that
// when serialized would fail to deserialize again.
// This custom serialize implementation prevents that by reporting a
// serialization error if the access rule would not be deserializable.
//
// At the time of writing, there is no code that creates such an access rule
// (at least no `api::AccessRule`, the special operator rules only ever exist
// as the `AccessRule` type from the crate root), but it's hard to prevent
// such code from being introduced statically.
impl Serialize for AccessRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct as _;

        if self.uses_reserved_namespace() {
            return Err(serde::ser::Error::custom(
                "access rule with reserved namespace must not be serialized",
            ));
        }

        let Self {
            effect,
            resource,
            actions,
        } = self;

        let mut s = serde::Serializer::serialize_struct(serializer, "AccessRule", 3)?;
        s.serialize_field("effect", effect)?;
        s.serialize_field("resource", resource)?;
        s.serialize_field("actions", actions)?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for AccessRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct AccessRuleRepr {
            effect: AccessRuleEffect,
            resource: ResourcePattern,
            actions: Vec<String>,
        }

        let AccessRuleRepr {
            effect,
            resource,
            actions,
        } = AccessRuleRepr::deserialize(deserializer)?;

        let result = Self {
            effect,
            resource,
            actions,
        };

        if result.uses_reserved_namespace() {
            return Err(serde::de::Error::custom(format!(
                "namespace {} is reserved",
                result.resource.namespace
            )));
        }

        Ok(result)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema, PersistableValue)]
#[serde(rename_all = "snake_case")]
pub enum AccessRuleEffect {
    Allow,
    Deny,
}

#[derive(Clone, Debug, PartialEq, Eq, PersistableValue)]
pub struct ResourcePattern {
    pub module: ModulePattern,
    pub namespace: NamespacePattern,
    pub key: KeyPattern,
}

impl ResourcePattern {
    pub fn matches(&self, op: &RequestedOperation<'_>, context: Context<'_>) -> bool {
        self.module.matches(op.module)
            && self.namespace.matches(op.namespace)
            && self.key.matches(op.key, context)
    }
}

impl fmt::Display for ResourcePattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            module,
            namespace,
            key,
        } = self;
        write!(f, "{module}:{namespace}:{key}")
    }
}

impl FromStr for ResourcePattern {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [module, namespace, key] = s
            .split(':')
            .collect_array()
            .ok_or("invalid resource pattern, must contain exactly two colons")?;
        Ok(Self {
            module: module.parse()?,
            namespace: namespace.parse()?,
            key: key.parse()?,
        })
    }
}

impl Serialize for ResourcePattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for ResourcePattern {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl JsonSchema for ResourcePattern {
    fn schema_name() -> Cow<'static, str> {
        String::schema_name()
    }

    fn inline_schema() -> bool {
        true
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        String::json_schema(generator)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PersistableValue)]
pub enum ModulePattern {
    /// Wildcard (`*`).
    ///
    /// Does not match admin modules.
    Any,
    Exactly(Module),
}

impl ModulePattern {
    fn matches(&self, module: Module) -> bool {
        match self {
            Self::Any => !module.is_admin_module(),
            Self::Exactly(m) => module == *m,
        }
    }
}

impl fmt::Display for ModulePattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Any => f.write_str("*"),
            Self::Exactly(m) => m.fmt(f),
        }
    }
}

impl FromStr for ModulePattern {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Self::Any),
            _ => s.parse().map(Self::Exactly),
        }
    }
}
