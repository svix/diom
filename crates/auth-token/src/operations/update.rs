use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::{
    AuthTokenNamespace, State,
    controller::{AuthTokenModel, PartialUpdateInput},
    operations::{AuthTokenRaftState, AuthTokenRequest, UpdateResponse},
};
use coyote_core::types::Metadata;
use coyote_id::{AuthTokenId, NamespaceId};
use coyote_operations::{OpContext, Result};
use fjall_utils::StorageType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateResponseData {
    /// `None` if the token was not found.
    pub model: Option<AuthTokenModel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAuthTokenOperation {
    namespace_id: NamespaceId,
    storage_type: StorageType,
    pub id: AuthTokenId,
    pub name: Option<String>,
    pub expiry: Option<Timestamp>,
    pub metadata: Option<Metadata>,
    pub scopes: Option<Vec<String>>,
    pub enabled: Option<bool>,
}

impl UpdateAuthTokenOperation {
    pub fn new(
        namespace: AuthTokenNamespace,
        id: AuthTokenId,
        name: Option<String>,
        expiry: Option<Timestamp>,
        metadata: Option<Metadata>,
        scopes: Option<Vec<String>>,
        enabled: Option<bool>,
    ) -> Self {
        Self {
            namespace_id: namespace.id,
            storage_type: namespace.storage_type,
            id,
            name,
            expiry,
            metadata,
            scopes,
            enabled,
        }
    }

    async fn apply_real(self, state: &State, ctx: &OpContext) -> Result<UpdateResponseData> {
        let model = state
            .controller
            .partial_update(
                self.namespace_id,
                self.id,
                PartialUpdateInput {
                    name: self.name,
                    expiry: self.expiry,
                    metadata: self.metadata,
                    scopes: self.scopes,
                    enabled: self.enabled,
                    now: ctx.timestamp,
                },
            )
            .await?;
        Ok(UpdateResponseData { model })
    }
}

impl AuthTokenRequest for UpdateAuthTokenOperation {
    async fn apply(self, state: AuthTokenRaftState<'_>, ctx: &OpContext) -> UpdateResponse {
        UpdateResponse(self.apply_real(state.state, ctx).await)
    }
}
