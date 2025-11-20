use axum::{Router, extract::Json, http::StatusCode, routing::post};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct RateLimitStore {
    // Token bucket: key -> (tokens, last_refill_time, capacity, refill_rate)
    token_buckets: Arc<RwLock<HashMap<String, (f64, Instant, f64, f64)>>>,

    // Sliding window: key -> (timestamps of requests)
    sliding_windows: Arc<RwLock<HashMap<String, VecDeque<Instant>>>>,

    // Fixed window: key -> (count, window_start)
    fixed_windows: Arc<RwLock<HashMap<String, (u64, Instant)>>>,
}

impl RateLimitStore {
    pub fn new() -> Self {
        Self {
            token_buckets: Arc::new(RwLock::new(HashMap::new())),
            sliding_windows: Arc::new(RwLock::new(HashMap::new())),
            fixed_windows: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[derive(Debug, Deserialize)]
struct TokenBucketRequest {
    key: String,
    capacity: f64,
    refill_rate: f64, // tokens per second
    #[serde(default = "default_tokens")]
    tokens: f64,
}

fn default_tokens() -> f64 {
    1.0
}

#[derive(Debug, Serialize)]
struct TokenBucketResponse {
    allowed: bool,
    remaining: f64,
}

#[derive(Debug, Deserialize)]
struct SlidingWindowRequest {
    key: String,
    max_requests: u64,
    window_seconds: u64,
}

#[derive(Debug, Serialize)]
struct SlidingWindowResponse {
    allowed: bool,
    current_count: u64,
    remaining: u64,
}

#[derive(Debug, Deserialize)]
struct FixedWindowRequest {
    key: String,
    max_requests: u64,
    window_seconds: u64,
}

#[derive(Debug, Serialize)]
struct FixedWindowResponse {
    allowed: bool,
    current_count: u64,
    remaining: u64,
    window_reset_at: u64, // seconds until window resets
}

async fn token_bucket_handler(
    axum::extract::State(store): axum::extract::State<RateLimitStore>,
    Json(req): Json<TokenBucketRequest>,
) -> (StatusCode, Json<TokenBucketResponse>) {
    let mut buckets = store.token_buckets.write().await;
    let now = Instant::now();

    let (mut tokens, last_refill, capacity, refill_rate) = buckets
        .get(&req.key)
        .copied()
        .unwrap_or((req.capacity, now, req.capacity, req.refill_rate));

    // Refill tokens based on time elapsed
    let elapsed = now.duration_since(last_refill).as_secs_f64();
    tokens = (tokens + elapsed * refill_rate).min(capacity);

    let allowed = tokens >= req.tokens;

    if allowed {
        tokens -= req.tokens;
    }

    buckets.insert(req.key, (tokens, now, capacity, refill_rate));

    (
        StatusCode::OK,
        Json(TokenBucketResponse {
            allowed,
            remaining: tokens,
        }),
    )
}

async fn sliding_window_handler(
    axum::extract::State(store): axum::extract::State<RateLimitStore>,
    Json(req): Json<SlidingWindowRequest>,
) -> (StatusCode, Json<SlidingWindowResponse>) {
    let mut windows = store.sliding_windows.write().await;
    let now = Instant::now();
    let window_duration = Duration::from_secs(req.window_seconds);
    let cutoff = now - window_duration;

    let timestamps = windows.entry(req.key).or_insert_with(VecDeque::new);

    // Remove expired timestamps
    while let Some(&front) = timestamps.front() {
        if front <= cutoff {
            timestamps.pop_front();
        } else {
            break;
        }
    }

    let current_count = timestamps.len() as u64;
    let allowed = current_count < req.max_requests;

    if allowed {
        timestamps.push_back(now);
    }

    let remaining = req
        .max_requests
        .saturating_sub(current_count + if allowed { 1 } else { 0 });

    (
        StatusCode::OK,
        Json(SlidingWindowResponse {
            allowed,
            current_count: if allowed {
                current_count + 1
            } else {
                current_count
            },
            remaining,
        }),
    )
}

async fn fixed_window_handler(
    axum::extract::State(store): axum::extract::State<RateLimitStore>,
    Json(req): Json<FixedWindowRequest>,
) -> (StatusCode, Json<FixedWindowResponse>) {
    let mut windows = store.fixed_windows.write().await;
    let now = Instant::now();
    let window_duration = Duration::from_secs(req.window_seconds);

    let (mut count, window_start) = windows.get(&req.key).copied().unwrap_or((0, now));

    // Check if we need to reset the window
    if now.duration_since(window_start) >= window_duration {
        count = 0;
        windows.insert(req.key.clone(), (0, now));
    }

    let allowed = count < req.max_requests;

    if allowed {
        count += 1;
        windows.insert(req.key.clone(), (count, window_start));
    }

    let remaining = req.max_requests.saturating_sub(count);
    let window_reset_at = window_duration
        .saturating_sub(now.duration_since(window_start))
        .as_secs();

    (
        StatusCode::OK,
        Json(FixedWindowResponse {
            allowed,
            current_count: count,
            remaining,
            window_reset_at,
        }),
    )
}

#[derive(Debug, Deserialize)]
struct ResetRequest {
    key: String,
}

#[derive(Debug, Serialize)]
struct ResetResponse {
    success: bool,
}

async fn reset_handler(
    axum::extract::State(store): axum::extract::State<RateLimitStore>,
    Json(req): Json<ResetRequest>,
) -> (StatusCode, Json<ResetResponse>) {
    let mut token_buckets = store.token_buckets.write().await;
    let mut sliding_windows = store.sliding_windows.write().await;
    let mut fixed_windows = store.fixed_windows.write().await;

    token_buckets.remove(&req.key);
    sliding_windows.remove(&req.key);
    fixed_windows.remove(&req.key);

    (StatusCode::OK, Json(ResetResponse { success: true }))
}

pub fn router() -> Router {
    let store = RateLimitStore::new();

    Router::new()
        .route("/token_bucket", post(token_bucket_handler))
        .route("/sliding_window", post(sliding_window_handler))
        .route("/fixed_window", post(fixed_window_handler))
        .route("/reset", post(reset_handler))
        .with_state(store)
}
