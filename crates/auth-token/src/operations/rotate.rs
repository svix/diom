use diom_core::PersistableValue;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::{
    AuthTokenNamespace, State,
    controller::{AuthTokenModel, RotateTokenInput},
    entities::TokenHashed,
    operations::{AuthTokenRaftState, AuthTokenRequest, RotateResponse},
};
use diom_error::Result;
use diom_id::{AuthTokenId, NamespaceId, UuidV7RandomBytes};
use diom_operations::OpContext;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotateResponseData {
    /// `None` if the original token was not found.
    pub model: Option<AuthTokenModel>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
pub struct RotateAuthTokenOperation {
    namespace_id: NamespaceId,
    old_id: AuthTokenId,
    new_id_random_bytes: UuidV7RandomBytes,
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
    ) -> Self {
        Self {
            namespace_id: namespace.id,
            old_id,
            new_id_random_bytes: UuidV7RandomBytes::new_random(),
            new_token_hashed,
            old_expiry,
        }
    }

    async fn apply_real(self, state: &State, ctx: &OpContext) -> Result<RotateResponseData> {
        let old_expiry = self.old_expiry.unwrap_or(ctx.timestamp);
        let new_id = AuthTokenId::new(ctx.timestamp, self.new_id_random_bytes);
        let model = state
            .controller
            .rotate(
                self.namespace_id,
                self.old_id,
                RotateTokenInput {
                    new_id,
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
        RotateResponse::new(self.apply_real(state.state, ctx).await)
    }
}
