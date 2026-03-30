use diom_error::Result;
use fjall_utils::Databases;

pub mod controller;
pub mod operations;
pub mod tables;

use crate::controller::AdminAuthController;

const ADMIN_AUTH_KEYSPACE: &str = "mod_admin_auth";

#[derive(Clone)]
pub struct State {
    pub controller: AdminAuthController,
}

impl State {
    pub fn init(dbs: Databases) -> Result<Self> {
        Ok(Self {
            controller: AdminAuthController::new(dbs.persistent, ADMIN_AUTH_KEYSPACE),
        })
    }
}
