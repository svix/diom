use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::{
    AuthTokenNamespace, State,
    controller::{AuthTokenModel, PartialUpdateInput},
    operations::{AuthTokenRaftState, AuthTokenRequest, UpdateResponse},
};
use diom_core::{
    PersistableValue,
    types::{Metadata, UnixTimestampMs},
};
use diom_error::Result;
use diom_id::{AuthTokenId, NamespaceId};
use diom_operations::OpContext;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateResponseData {
    /// `None` if the token was not found.
    pub model: Option<AuthTokenModel>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
pub struct UpdateAuthTokenOperation {
    namespace_id: NamespaceId,
    pub id: AuthTokenId,
    pub name: Option<String>,
    pub expiry: Option<UnixTimestampMs>,
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
            id,
            name,
            expiry: expiry.map(|e| e.into()),
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
        UpdateResponse::new(self.apply_real(state.state, ctx).await)
    }
}
