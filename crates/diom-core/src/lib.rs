use std::sync::LazyLock;

pub mod backoff;
pub mod fifo_cache;
pub mod instrumented_mutex;
pub mod json;
mod monotime;
pub mod shutdown;
pub mod task;
pub mod types;
pub mod validation;

pub static INSTANCE_ID: LazyLock<String> = LazyLock::new(|| uuid::Uuid::new_v4().to_string());

pub use self::monotime::Monotime;
