use diom_error::Result;
use diom_namespace::{
    entities::KeyValueConfig,
    operations::create_namespace::{CreateNamespace, CreateNamespaceOutput},
};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::operations::{CreateKvResponse, KvRaftState, KvRequest};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateKvOperation {
    pub(crate) name: String,
}

impl From<CreateKvOperation> for CreateNamespace<KeyValueConfig> {
    fn from(value: CreateKvOperation) -> Self {
        CreateNamespace::new(value.name, KeyValueConfig {})
    }
}

impl CreateKvOperation {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    async fn apply_real(
        self,
        namespace_state: &diom_namespace::State,
        now: Timestamp,
    ) -> Result<CreateKvResponseData> {
        let op: CreateNamespace<KeyValueConfig> = self.into();
        let out = op.apply_operation(namespace_state, now).await?;
        Ok(out.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateKvResponseData {
    pub name: String,
    pub created: Timestamp,
    pub updated: Timestamp,
}

impl From<CreateNamespaceOutput<KeyValueConfig>> for CreateKvResponseData {
    fn from(value: CreateNamespaceOutput<KeyValueConfig>) -> Self {
        Self {
            name: value.name,
            created: value.created,
            updated: value.updated,
        }
    }
}

impl KvRequest for CreateKvOperation {
    async fn apply(
        self,
        state: KvRaftState<'_>,
        ctx: &diom_operations::OpContext,
    ) -> CreateKvResponse {
        CreateKvResponse::new(self.apply_real(state.namespace, ctx.timestamp).await)
    }
}
