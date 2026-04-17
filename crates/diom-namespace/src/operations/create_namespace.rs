use diom_core::types::UnixTimestampMs;
use diom_error::Result;
use diom_id::{NamespaceId, UuidV7RandomBytes};

use fjall_utils::WriteBatchExt;
use serde::{Deserialize, Serialize};
use tracing::Span;

use crate::{
    State,
    entities::{ModuleConfig, NamespaceName},
    storage::{Namespace, NamespaceKey},
};

#[derive(Deserialize, Serialize)]
#[serde(bound = "C: ModuleConfig")]
pub struct CreateNamespace<C: ModuleConfig> {
    name: NamespaceName,
    config: C,
    id_random_bytes: UuidV7RandomBytes,
}

#[derive(Deserialize, Serialize)]
#[serde(bound = "C: ModuleConfig")]
pub struct CreateNamespaceOutput<C: ModuleConfig> {
    pub name: NamespaceName,
    pub config: C,
    pub created: UnixTimestampMs,
    pub updated: UnixTimestampMs,
}

impl<C: ModuleConfig + 'static> CreateNamespace<C> {
    pub fn new(name: NamespaceName, config: C, id_random_bytes: UuidV7RandomBytes) -> Self {
        Self {
            name,
            config,
            id_random_bytes,
        }
    }

    pub async fn apply_operation(
        self,
        state: &State,
        timestamp: UnixTimestampMs,
    ) -> Result<CreateNamespaceOutput<C>> {
        let db = state.db().clone();
        let keyspace = state.keyspace().clone();
        let span = Span::current();
        // can't use diom-core here because of circular deps
        #[allow(clippy::disallowed_methods)]
        tokio::task::spawn_blocking(move || {
            span.in_scope(|| {
                let namespace = match Namespace::<C>::fetch(&keyspace, &self.name)? {
                    Some(mut namespace) => {
                        namespace.updated = timestamp;
                        namespace.config = self.config;
                        namespace
                    }
                    None => {
                        let id = NamespaceId::new(timestamp, self.id_random_bytes);
                        Namespace {
                            id,
                            name: self.name,
                            created: timestamp,
                            updated: timestamp,
                            config: self.config,
                        }
                    }
                };

                {
                    let k1 = NamespaceKey::build_key(
                        &Namespace::<C>::module_id(),
                        namespace.name.as_ref(),
                    );
                    let mut batch = db.batch().durability(Some(fjall::PersistMode::SyncAll));
                    batch.insert_row(&keyspace, k1, &namespace)?;
                    batch.commit()?;
                }

                Ok(CreateNamespaceOutput {
                    name: namespace.name,
                    config: namespace.config,
                    created: namespace.created,
                    updated: namespace.updated,
                })
            })
        })
        .await?
    }
}
