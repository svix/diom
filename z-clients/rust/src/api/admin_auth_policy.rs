// this file is @generated
use crate::{Configuration, error::Result, models::*};

pub struct AdminAuthPolicy<'a> {
    cfg: &'a Configuration,
}

impl<'a> AdminAuthPolicy<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Create or update an access policy
    pub async fn configure(
        &self,
        admin_access_policy_configure_in: AdminAccessPolicyConfigureIn,
    ) -> Result<AdminAccessPolicyConfigureOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.admin.auth-policy.configure")
            .with_body(admin_access_policy_configure_in)
            .execute(self.cfg)
            .await
    }

    /// Delete an access policy
    pub async fn delete(
        &self,
        admin_access_policy_delete_in: AdminAccessPolicyDeleteIn,
    ) -> Result<AdminAccessPolicyDeleteOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.admin.auth-policy.delete")
            .with_body(admin_access_policy_delete_in)
            .execute(self.cfg)
            .await
    }

    /// Get an access policy by ID
    pub async fn get(
        &self,
        admin_access_policy_get_in: AdminAccessPolicyGetIn,
    ) -> Result<AdminAccessPolicyOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.admin.auth-policy.get")
            .with_body(admin_access_policy_get_in)
            .execute(self.cfg)
            .await
    }

    /// List all access policies
    pub async fn list(
        &self,
        admin_access_policy_list_in: AdminAccessPolicyListIn,
    ) -> Result<ListResponseAdminAccessPolicyOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.admin.auth-policy.list")
            .with_body(admin_access_policy_list_in)
            .execute(self.cfg)
            .await
    }
}
