use axum::{
    extract::{Json, Path, Query},
    http::StatusCode,
    routing::{delete, post},
    Router,
};
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
struct TokenBucketQuery {
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
struct SlidingWindowQuery {
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
struct FixedWindowQuery {
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

// POST /ratelimit/token_bucket/:key?capacity=100&refill_rate=10&tokens=1
async fn token_bucket_handler(
    axum::extract::State(store): axum::extract::State<RateLimitStore>,
    Path(key): Path<String>,
    Query(query): Query<TokenBucketQuery>,
) -> (StatusCode, Json<TokenBucketResponse>) {
    let mut buckets = store.token_buckets.write().await;
    let now = Instant::now();

    let (mut tokens, last_refill, capacity, refill_rate) = buckets
        .get(&key)
        .copied()
        .unwrap_or((query.capacity, now, query.capacity, query.refill_rate));

    // Refill tokens based on time elapsed
    let elapsed = now.duration_since(last_refill).as_secs_f64();
    tokens = (tokens + elapsed * refill_rate).min(capacity);

    let allowed = tokens >= query.tokens;

    if allowed {
        tokens -= query.tokens;
    }

    buckets.insert(key, (tokens, now, capacity, refill_rate));

    (
        StatusCode::OK,
        Json(TokenBucketResponse {
            allowed,
            remaining: tokens,
        }),
    )
}

// POST /ratelimit/sliding_window/:key?max_requests=100&window_seconds=60
async fn sliding_window_handler(
    axum::extract::State(store): axum::extract::State<RateLimitStore>,
    Path(key): Path<String>,
    Query(query): Query<SlidingWindowQuery>,
) -> (StatusCode, Json<SlidingWindowResponse>) {
    let mut windows = store.sliding_windows.write().await;
    let now = Instant::now();
    let window_duration = Duration::from_secs(query.window_seconds);
    let cutoff = now - window_duration;

    let timestamps = windows.entry(key).or_insert_with(VecDeque::new);

    // Remove expired timestamps
    while let Some(&front) = timestamps.front() {
        if front <= cutoff {
            timestamps.pop_front();
        } else {
            break;
        }
    }

    let current_count = timestamps.len() as u64;
    let allowed = current_count < query.max_requests;

    if allowed {
        timestamps.push_back(now);
    }

    let remaining = query.max_requests.saturating_sub(current_count + if allowed { 1 } else { 0 });

    (
        StatusCode::OK,
        Json(SlidingWindowResponse {
            allowed,
            current_count: if allowed { current_count + 1 } else { current_count },
            remaining,
        }),
    )
}

// POST /ratelimit/fixed_window/:key?max_requests=100&window_seconds=60
async fn fixed_window_handler(
    axum::extract::State(store): axum::extract::State<RateLimitStore>,
    Path(key): Path<String>,
    Query(query): Query<FixedWindowQuery>,
) -> (StatusCode, Json<FixedWindowResponse>) {
    let mut windows = store.fixed_windows.write().await;
    let now = Instant::now();
    let window_duration = Duration::from_secs(query.window_seconds);

    let (mut count, window_start) = windows
        .get(&key)
        .copied()
        .unwrap_or((0, now));

    // Check if we need to reset the window
    if now.duration_since(window_start) >= window_duration {
        count = 0;
        windows.insert(key.clone(), (0, now));
    }

    let allowed = count < query.max_requests;

    if allowed {
        count += 1;
        windows.insert(key.clone(), (count, window_start));
    }

    let remaining = query.max_requests.saturating_sub(count);
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

// DELETE /ratelimit/:key
async fn reset_handler(
    axum::extract::State(store): axum::extract::State<RateLimitStore>,
    Path(key): Path<String>,
) -> StatusCode {
    let mut token_buckets = store.token_buckets.write().await;
    let mut sliding_windows = store.sliding_windows.write().await;
    let mut fixed_windows = store.fixed_windows.write().await;

    token_buckets.remove(&key);
    sliding_windows.remove(&key);
    fixed_windows.remove(&key);

    StatusCode::NO_CONTENT
}

pub fn router() -> Router {
    let store = RateLimitStore::new();

    Router::new()
        .route("/token_bucket/:key", post(token_bucket_handler))
        .route("/sliding_window/:key", post(sliding_window_handler))
        .route("/fixed_window/:key", post(fixed_window_handler))
        .route("/:key", delete(reset_handler))
        .with_state(store)
}
