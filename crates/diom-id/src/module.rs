use diom_core::PersistableValue;
use std::{fmt, str::FromStr};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PersistableValue)]
pub enum Module {
    Cache = 1,
    Idempotency = 2,
    Kv = 3,
    RateLimit = 4,
    Msgs = 5,
    AuthToken = 6,

    AdminCluster = 200,
    AdminNamespace = 201,
    AdminAuthToken = 202,
    AdminRole = 203,
    AdminAccessPolicy = 204,
}

impl Module {
    pub fn is_admin_module(&self) -> bool {
        match self {
            Self::Cache
            | Self::Idempotency
            | Self::Kv
            | Self::RateLimit
            | Self::Msgs
            | Self::AuthToken => false,
            Self::AdminCluster
            | Self::AdminNamespace
            | Self::AdminAuthToken
            | Self::AdminRole
            | Self::AdminAccessPolicy => true,
        }
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Cache => "cache",
            Self::AuthToken => "auth_token",
            Self::Idempotency => "idempotency",
            Self::Kv => "kv",
            Self::Msgs => "msgs",
            Self::RateLimit => "rate_limit",

            Self::AdminCluster => "admin/cluster",
            Self::AdminNamespace => "admin/namespace",
            Self::AdminAuthToken => "admin/auth_token",
            Self::AdminRole => "admin/role",
            Self::AdminAccessPolicy => "admin/access_policy",
        };

        f.write_str(s)
    }
}

impl FromStr for Module {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auth_token" => Ok(Self::AuthToken),
            "cache" => Ok(Self::Cache),
            "idempotency" => Ok(Self::Idempotency),
            "kv" => Ok(Self::Kv),
            "msgs" => Ok(Self::Msgs),
            "rate_limit" => Ok(Self::RateLimit),

            "admin/cluster" => Ok(Self::AdminCluster),
            "admin/namespace" => Ok(Self::AdminNamespace),
            "admin/auth_token" => Ok(Self::AdminAuthToken),
            "admin/role" => Ok(Self::AdminRole),
            "admin/access_policy" => Ok(Self::AdminAccessPolicy),

            _ => Err("unknown module"),
        }
    }
}
