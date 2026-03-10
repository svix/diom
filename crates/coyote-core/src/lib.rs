use std::sync::LazyLock;

pub mod shutdown;
pub mod types;

pub static INSTANCE_ID: LazyLock<String> = LazyLock::new(|| uuid::Uuid::new_v4().to_string());
