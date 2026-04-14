use std::sync::LazyLock;

pub mod backoff;
pub mod fifo_cache;
pub mod instrumented_mutex;
mod monotime;
pub mod shutdown;
pub mod task;
pub mod types;
pub mod validation;

pub static INSTANCE_ID: LazyLock<String> = LazyLock::new(|| uuid::Uuid::new_v4().to_string());

pub use self::monotime::Monotime;

#[doc(hidden)]
pub mod __reexport {
    pub use regex;
    pub use schemars;
    pub use serde_json;
    pub use validator;
}
