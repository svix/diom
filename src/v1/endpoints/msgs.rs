use aide::axum::{ApiRouter, routing::post_with};
use axum::{Extension, extract::State};
use diom_derive::aide_annotate;
use diom_error::{Error, HttpError, Result, ResultExt};
use diom_namespace::entities::StorageType;
use diom_proto::MsgPackOrJson;
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use stream_internals::entities::{Retention, default_retention_bytes, default_retention_millis};
use validator::Validate;

use crate::{AppState, core::cluster::RaftState, v1::utils::openapi_tag};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct CreateNamespaceIn {
    pub name: String,
    #[serde(default)]
    pub retention: Retention,
    #[serde(default)]
    pub storage_type: StorageType,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct CreateNamespaceOut {
    pub name: String,
    pub retention: Retention,
    pub storage_type: StorageType,
    pub created: Timestamp,
    pub updated: Timestamp,
}

/// Creates or updates a msgs namespace with the given name.
#[aide_annotate(op_id = "v1.msgs.namespace.create")]
async fn create_namespace(
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<CreateNamespaceIn>,
) -> Result<MsgPackOrJson<CreateNamespaceOut>> {
    let operation = diom_msgs::operations::CreateNamespaceOperation::new(
        data.name,
        data.retention,
        data.storage_type,
    );
    let response = repl.client_write(operation).await.map_err_generic()?.0?;

    Ok(MsgPackOrJson(CreateNamespaceOut {
        name: response.name,
        retention: response.retention,
        storage_type: response.storage_type,
        created: response.created,
        updated: response.updated,
    }))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct GetNamespaceIn {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct GetNamespaceOut {
    pub name: String,
    pub retention: Retention,
    pub storage_type: StorageType,
    pub created: Timestamp,
    pub updated: Timestamp,
}

/// Gets a msgs namespace by name.
#[aide_annotate(op_id = "v1.msgs.namespace.get")]
async fn get_namespace(
    State(state): State<AppState>,
    MsgPackOrJson(data): MsgPackOrJson<GetNamespaceIn>,
) -> Result<MsgPackOrJson<GetNamespaceOut>> {
    let namespace = state
        .namespace_state
        .fetch_stream_namespace(&data.name)?
        .ok_or_else(|| Error::http(HttpError::not_found(None, None)))?;

    let millis = u64::try_from(namespace.config.retention_period.as_millis())
        .ok()
        .and_then(|ms| ms.try_into().ok())
        .unwrap_or_else(default_retention_millis);
    let bytes = namespace
        .max_storage_bytes
        .unwrap_or_else(default_retention_bytes);

    Ok(MsgPackOrJson(GetNamespaceOut {
        name: namespace.name,
        retention: Retention { millis, bytes },
        storage_type: namespace.storage_type,
        created: namespace.created_at,
        updated: namespace.updated_at,
    }))
}

pub fn router() -> ApiRouter<AppState> {
    let tag = openapi_tag("Msgs");

    ApiRouter::new()
        .api_route_with(
            "/msgs/namespace/create",
            post_with(create_namespace, create_namespace_operation),
            &tag,
        )
        .api_route_with(
            "/msgs/namespace/get",
            post_with(get_namespace, get_namespace_operation),
            &tag,
        )
}
