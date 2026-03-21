use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::{
    AuthTokenNamespace, State,
    controller::{AuthTokenModel, RotateTokenInput},
    entities::TokenHashed,
    operations::{AuthTokenRaftState, AuthTokenRequest, RotateResponse},
};
use diom_id::{AuthTokenId, NamespaceId};
use diom_operations::{OpContext, Result};
use fjall_utils::StorageType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotateResponseData {
    /// `None` if the original token was not found.
    pub model: Option<AuthTokenModel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotateAuthTokenOperation {
    namespace_id: NamespaceId,
    storage_type: StorageType,
    old_id: AuthTokenId,
    new_id: AuthTokenId,
    new_token_hashed: TokenHashed,
    /// When the old token expires. `None` means expire immediately (now).
    old_expiry: Option<Timestamp>,
}

impl RotateAuthTokenOperation {
    pub fn new(
        namespace: AuthTokenNamespace,
        old_id: AuthTokenId,
        new_token_hashed: TokenHashed,
        old_expiry: Option<Timestamp>,
        now: Timestamp,
    ) -> Self {
        let new_id = AuthTokenId::new(now);
        Self {
            namespace_id: namespace.id,
            storage_type: namespace.storage_type,
            old_id,
            new_id,
            new_token_hashed,
            old_expiry,
        }
    }

    async fn apply_real(self, state: &State, ctx: &OpContext) -> Result<RotateResponseData> {
        let old_expiry = self.old_expiry.unwrap_or(ctx.timestamp);
        let model = state
            .controller
            .rotate(
                self.namespace_id,
                self.old_id,
                RotateTokenInput {
                    new_id: self.new_id,
                    new_token_hashed: self.new_token_hashed,
                    old_expiry,
                    now: ctx.timestamp,
                },
            )
            .await?;
        Ok(RotateResponseData { model })
    }
}

impl AuthTokenRequest for RotateAuthTokenOperation {
    async fn apply(self, state: AuthTokenRaftState<'_>, ctx: &OpContext) -> RotateResponse {
        RotateResponse(self.apply_real(state.state, ctx).await)
    }
}
