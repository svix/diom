use std::sync::LazyLock;

pub mod backoff;
mod monotime;
pub mod shutdown;
pub mod types;

pub static INSTANCE_ID: LazyLock<String> = LazyLock::new(|| uuid::Uuid::new_v4().to_string());

pub use monotime::Monotime;
