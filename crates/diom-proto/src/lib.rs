#![warn(clippy::str_to_string)]

mod body_limit;
mod error;
mod internal_client;
mod msgpack;
mod msgpack_client;
mod msgpack_or_json;
pub mod prelude;
mod request_input;

pub use self::{
    body_limit::limit_requests_body_gracefully,
    error::{ErrorBody, ErrorType},
    internal_client::{InternalClient, InternalRequest, InternalRequestError},
    msgpack::MsgPack,
    msgpack_or_json::{MsgPackOrJson, capture_accept_hdr},
    request_input::{AccessMetadata, RequestInput},
};
