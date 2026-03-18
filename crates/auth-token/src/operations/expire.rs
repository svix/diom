use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::{
    AuthTokenNamespace, State,
    controller::AuthTokenModel,
    operations::{AuthTokenRaftState, AuthTokenRequest, ExpireResponse},
};
use coyote_id::{AuthTokenId, NamespaceId};
use coyote_operations::{OpContext, Result};
use fjall_utils::StorageType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpireResponseData {
    /// `None` if the token was not found.
    pub model: Option<AuthTokenModel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpireAuthTokenOperation {
    namespace_id: NamespaceId,
    storage_type: StorageType,
    pub id: AuthTokenId,
    /// The timestamp at which the token expires. `None` means expire immediately (now).
    pub expiry: Option<Timestamp>,
}

impl ExpireAuthTokenOperation {
    pub fn new(namespace: AuthTokenNamespace, id: AuthTokenId, expiry: Option<Timestamp>) -> Self {
        Self {
            namespace_id: namespace.id,
            storage_type: namespace.storage_type,
            id,
            expiry,
        }
    }

    fn apply_real(self, state: &State, ctx: &OpContext) -> Result<ExpireResponseData> {
        let expiry = self.expiry.unwrap_or(ctx.timestamp);
        let model = state
            .controller
            .expire(self.namespace_id, self.id, expiry, ctx.timestamp)?;
        Ok(ExpireResponseData { model })
    }
}

impl AuthTokenRequest for ExpireAuthTokenOperation {
    fn apply(self, state: AuthTokenRaftState<'_>, ctx: &OpContext) -> ExpireResponse {
        ExpireResponse(self.apply_real(state.state, ctx))
    }
}
