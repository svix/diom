#![warn(clippy::str_to_string)]

mod error;
mod internal_client;
mod msgpack;
mod msgpack_client;
mod msgpack_or_json;
pub mod prelude;
mod request_input;

pub use self::{
    error::StandardErrorBody,
    internal_client::{InternalClient, InternalRequest, InternalRequestError},
    msgpack::MsgPack,
    msgpack_or_json::{MsgPackOrJson, capture_accept_hdr},
    request_input::{AccessMetadata, RequestInput},
};
