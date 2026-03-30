pub mod api;
mod client;
mod connector;
mod error;
pub mod models;
mod request;

pub(crate) use self::client::Configuration;
pub use self::{
    client::{CoyoteClient, CoyoteOptions, DEFAULT_URL},
    error::{Error, Result},
};
