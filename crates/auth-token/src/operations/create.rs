use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::{
    AuthTokenNamespace, State,
    controller::{AuthTokenModel, CreateTokenInput},
    entities::TokenHashed,
    operations::{AuthTokenRaftState, AuthTokenRequest, CreateResponse},
};
use diom_core::types::Metadata;
use diom_error::Result;
use diom_id::{AuthTokenId, NamespaceId, UuidV7RandomBytes};
use diom_operations::OpContext;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateResponseData {
    pub model: AuthTokenModel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAuthTokenOperation {
    namespace_id: NamespaceId,
    id_random_bytes: UuidV7RandomBytes,
    name: String,
    token_hashed: TokenHashed,
    expiry: Option<Timestamp>,
    metadata: Metadata,
    owner_id: String,
    scopes: Vec<String>,
    enabled: bool,
}

impl CreateAuthTokenOperation {
    // FIXME: it's indeed too many arguments, but the problem we need fixing is this duplication
    // and unrolling when creating operations, which I'm not sure how to do yet.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        namespace: AuthTokenNamespace,
        name: String,
        token_hashed: TokenHashed,
        expiry: Option<Timestamp>,
        metadata: Metadata,
        owner_id: String,
        scopes: Vec<String>,
        enabled: bool,
    ) -> Self {
        Self {
            namespace_id: namespace.id,
            id_random_bytes: UuidV7RandomBytes::new_random(),
            name,
            token_hashed,
            expiry,
            metadata,
            owner_id,
            scopes,
            enabled,
        }
    }

    async fn apply_real(self, state: &State, ctx: &OpContext) -> Result<CreateResponseData> {
        let id = AuthTokenId::new(ctx.timestamp, self.id_random_bytes);
        let model = state
            .controller
            .create(
                self.namespace_id,
                CreateTokenInput {
                    id,
                    name: self.name,
                    token_hashed: self.token_hashed,
                    expiry: self.expiry,
                    metadata: self.metadata,
                    owner_id: self.owner_id,
                    scopes: self.scopes,
                    enabled: self.enabled,
                    now: ctx.timestamp,
                },
            )
            .await?;
        Ok(CreateResponseData { model })
    }
}

impl AuthTokenRequest for CreateAuthTokenOperation {
    async fn apply(self, state: AuthTokenRaftState<'_>, ctx: &OpContext) -> CreateResponse {
        CreateResponse::new(self.apply_real(state.state, ctx).await)
    }
}
