// this file is @generated
use crate::{Configuration, error::Result, models::*};

pub struct Kv<'a> {
    cfg: &'a Configuration,
}

impl<'a> Kv<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// KV Set
    pub async fn set(&self, kv_set_in: KvSetIn) -> Result<KvSetOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1/kv/set")
            .with_body_param(kv_set_in)
            .execute(self.cfg)
            .await
    }

    /// KV Get
    pub async fn get(&self, kv_get_in: KvGetIn) -> Result<KvGetOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1/kv/get")
            .with_body_param(kv_get_in)
            .execute(self.cfg)
            .await
    }

    /// Get KV store
    pub async fn get_group(&self, kv_get_group_in: KvGetGroupIn) -> Result<KvGetGroupOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1/kv/get-group")
            .with_body_param(kv_get_group_in)
            .execute(self.cfg)
            .await
    }

    /// KV Delete
    pub async fn delete(&self, kv_delete_in: KvDeleteIn) -> Result<KvDeleteOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1/kv/delete")
            .with_body_param(kv_delete_in)
            .execute(self.cfg)
            .await
    }
}
