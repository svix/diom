use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::{
    AuthTokenNamespace, State,
    controller::{AuthTokenModel, CreateTokenInput},
    entities::TokenHashed,
    operations::{AuthTokenRaftState, AuthTokenRequest, CreateResponse},
};
use coyote_core::types::Metadata;
use coyote_id::{AuthTokenId, NamespaceId};
use coyote_operations::{OpContext, Result};
use fjall_utils::StorageType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateResponseData {
    pub model: AuthTokenModel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAuthTokenOperation {
    namespace_id: NamespaceId,
    storage_type: StorageType,
    pub(crate) id: AuthTokenId,
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
        now: Timestamp,
    ) -> Self {
        let id = AuthTokenId::new(now);
        Self {
            namespace_id: namespace.id,
            storage_type: namespace.storage_type,
            id,
            name,
            token_hashed,
            expiry,
            metadata,
            owner_id,
            scopes,
            enabled,
        }
    }

    fn apply_real(self, state: &State, ctx: &OpContext) -> Result<CreateResponseData> {
        let model = state.controller.create(
            self.namespace_id,
            CreateTokenInput {
                id: self.id,
                name: self.name,
                token_hashed: self.token_hashed,
                expiry: self.expiry,
                metadata: self.metadata,
                owner_id: self.owner_id,
                scopes: self.scopes,
                enabled: self.enabled,
                now: ctx.timestamp,
            },
        )?;
        Ok(CreateResponseData { model })
    }
}

impl AuthTokenRequest for CreateAuthTokenOperation {
    fn apply(self, state: AuthTokenRaftState<'_>, ctx: &OpContext) -> CreateResponse {
        CreateResponse(self.apply_real(state.state, ctx))
    }
}
