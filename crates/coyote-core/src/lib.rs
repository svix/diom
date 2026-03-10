use std::sync::LazyLock;

mod monotime;
pub mod shutdown;
pub mod sync;
pub mod types;

pub static INSTANCE_ID: LazyLock<String> = LazyLock::new(|| uuid::Uuid::new_v4().to_string());

pub use monotime::Monotime;
