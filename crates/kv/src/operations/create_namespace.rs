use diom_error::Result;
use diom_id::UuidV7RandomBytes;
use diom_namespace::{
    entities::{KeyValueConfig, NamespaceName},
    operations::create_namespace::{CreateNamespace, CreateNamespaceOutput},
};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::operations::{CreateKvResponse, KvRaftState, KvRequest};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateKvOperation {
    pub(crate) name: NamespaceName,
    id_random_bytes: UuidV7RandomBytes,
}

impl From<CreateKvOperation> for CreateNamespace<KeyValueConfig> {
    fn from(value: CreateKvOperation) -> Self {
        CreateNamespace::new(value.name, KeyValueConfig {}, value.id_random_bytes)
    }
}

impl CreateKvOperation {
    pub fn new(name: NamespaceName) -> Self {
        Self {
            name,
            id_random_bytes: UuidV7RandomBytes::new_random(),
        }
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
    pub name: NamespaceName,
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
