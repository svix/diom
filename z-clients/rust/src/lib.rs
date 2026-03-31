pub mod api;
mod client;
mod connector;
mod duration_ms_serde;
mod error;
pub mod models;
mod request;
mod serde_bytes_opt;

pub(crate) use self::client::Configuration;
pub use self::{
    client::{DiomClient, DiomOptions, DEFAULT_URL},
    error::{Error, Result},
};
