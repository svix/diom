#![warn(clippy::str_to_string)]

use std::sync::LazyLock;

pub mod backoff;
pub mod fifo_cache;
pub mod instrumented_mutex;
mod monotime;
pub mod persistable_value;
pub mod schema_shape;
pub mod shutdown;
pub mod svix_client;
pub mod task;
pub mod template_str;
pub mod tokio_nursery;
pub mod types;

// The instance of a single run of this application
pub static INSTANCE_ID: LazyLock<String> =
    LazyLock::new(|| uuid::Uuid::new_v4().simple().to_string());

pub use self::monotime::Monotime;

#[doc(hidden)]
pub mod __reexport {
    pub use inventory;
    pub use postcard;
    pub use regex;
    pub use schemars;
    pub use serde;
    pub use serde_json;
}

pub use diom_derive::{PersistableValue, PersistableVersioned};
pub use persistable_value::PersistableValue;
