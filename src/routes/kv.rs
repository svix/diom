use axum::{
    Router,
    extract::{Json, Query},
    http::StatusCode,
    routing::post,
};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

#[derive(Clone)]
struct KvEntry {
    value: serde_json::Value,
    expires_at: Option<Instant>,
}

#[derive(Clone)]
pub struct KvStore {
    data: Arc<RwLock<HashMap<String, KvEntry>>>,
}

impl KvStore {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn cleanup_expired(&self) {
        let mut data = self.data.write().await;
        let now = Instant::now();
        data.retain(|_, entry| {
            if let Some(expires_at) = entry.expires_at {
                expires_at > now
            } else {
                true
            }
        });
    }
}

#[derive(Debug, Deserialize)]
struct SetRequest {
    key: String,
    value: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct TtlQuery {
    ttl: Option<u64>,
}

// POST /kv/set?ttl=3600
async fn set_handler(
    axum::extract::State(store): axum::extract::State<KvStore>,
    Query(query): Query<TtlQuery>,
    Json(req): Json<SetRequest>,
) -> StatusCode {
    store.cleanup_expired().await;

    let expires_at = query
        .ttl
        .map(|ttl| Instant::now() + Duration::from_secs(ttl));

    let entry = KvEntry {
        value: req.value,
        expires_at,
    };

    let mut data = store.data.write().await;
    data.insert(req.key, entry);

    StatusCode::NO_CONTENT
}

#[derive(Debug, Deserialize)]
struct KeyRequest {
    key: String,
}

// POST /kv/get
async fn get_handler(
    axum::extract::State(store): axum::extract::State<KvStore>,
    Json(req): Json<KeyRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    store.cleanup_expired().await;

    let data = store.data.read().await;
    let now = Instant::now();

    if let Some(entry) = data.get(&req.key) {
        if let Some(expires_at) = entry.expires_at
            && expires_at <= now
        {
            return Err(StatusCode::NOT_FOUND);
        }
        Ok(Json(entry.value.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

// POST /kv/delete
async fn delete_handler(
    axum::extract::State(store): axum::extract::State<KvStore>,
    Json(req): Json<KeyRequest>,
) -> StatusCode {
    let mut data = store.data.write().await;
    if data.remove(&req.key).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

// POST /kv/exists (check if key exists)
async fn exists_handler(
    axum::extract::State(store): axum::extract::State<KvStore>,
    Json(req): Json<KeyRequest>,
) -> StatusCode {
    store.cleanup_expired().await;

    let data = store.data.read().await;
    let now = Instant::now();

    let exists = if let Some(entry) = data.get(&req.key) {
        if let Some(expires_at) = entry.expires_at {
            expires_at > now
        } else {
            true
        }
    } else {
        false
    };

    if exists {
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

pub fn router() -> Router {
    let store = KvStore::new();

    Router::new()
        .route("/set", post(set_handler))
        .route("/get", post(get_handler))
        .route("/delete", post(delete_handler))
        .route("/exists", post(exists_handler))
        .with_state(store)
}
