use std::time::Duration;

use hyper::body::Bytes;
use hyper_util::client::legacy::Client as HyperClient;

pub mod api;
mod client;
mod connector;
mod error;
pub mod models;
mod request;
mod serde_bytes_opt;

use self::connector::Connector;
pub use self::{
    client::{DiomClient, DiomOptions, DEFAULT_URL},
    error::{Error, Result},
};

pub struct Configuration {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub bearer_access_token: Option<String>,
    pub timeout: Option<Duration>,
    pub num_retries: u32,
    pub retry_schedule: Option<Vec<Duration>>,

    client: HyperClient<Connector, http_body_util::Full<Bytes>>,
}
