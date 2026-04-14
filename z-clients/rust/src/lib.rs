pub mod api;
mod client;
mod connector;
mod duration_ms_serde;
mod error;
pub mod models;
mod request;
mod serde_bytes_opt;
mod unix_timestamp_ms_serde;

pub(crate) use self::client::Configuration;
pub use self::{
    client::{DEFAULT_URL, DiomClient, DiomOptions},
    error::{
        ClientError, Error, ErrorKind, GenericError, NetworkError, Result, ServerError,
        ValidationError,
    },
};
