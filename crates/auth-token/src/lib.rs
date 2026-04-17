use diom_error::Result;
use diom_namespace::{Namespace, entities::AuthTokenConfig};
use fjall_utils::Databases;

pub mod controller;
pub mod entities;
pub mod operations;
pub mod storage;

use crate::controller::AuthTokenController;

pub type AuthTokenNamespace = Namespace<AuthTokenConfig>;

const AUTH_TOKEN_KEYSPACE: &str = "mod_auth_token";

#[derive(Clone)]
pub struct State {
    pub controller: AuthTokenController,
}

impl State {
    pub fn init(dbs: Databases) -> Result<Self> {
        Ok(Self {
            controller: AuthTokenController::new(dbs.persistent, AUTH_TOKEN_KEYSPACE),
        })
    }
}
