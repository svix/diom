use http::header::CONTENT_TYPE;
use serde::{Serialize, de::DeserializeOwned};
use tap::Pipe;

pub(crate) const APPLICATION_MSGPACK: http::HeaderValue =
    http::HeaderValue::from_static("application/msgpack");

pub trait MsgpackRequestBuilder: Sized {
    fn msgpack<T: Serialize + ?Sized>(self, body: &T) -> Result<Self, rmp_serde::encode::Error>;
}

impl MsgpackRequestBuilder for reqwest::RequestBuilder {
    fn msgpack<T: Serialize + ?Sized>(self, body: &T) -> Result<Self, rmp_serde::encode::Error> {
        let serialized = rmp_serde::to_vec_named(body)?;
        self.header(CONTENT_TYPE, APPLICATION_MSGPACK)
            .body(serialized)
            .pipe(Ok)
    }
}

#[derive(Debug)]
pub enum MsgpackResponseError {
    Network(reqwest::Error),
    Serialization(rmp_serde::decode::Error),
    InvalidResponseContentType(http::HeaderValue),
}

impl From<reqwest::Error> for MsgpackResponseError {
    fn from(value: reqwest::Error) -> Self {
        Self::Network(value)
    }
}

impl From<rmp_serde::decode::Error> for MsgpackResponseError {
    fn from(value: rmp_serde::decode::Error) -> Self {
        Self::Serialization(value)
    }
}

impl std::fmt::Display for MsgpackResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Network(e) => e.fmt(f),
            Self::Serialization(e) => e.fmt(f),
            Self::InvalidResponseContentType(ct) => {
                write!(
                    f,
                    "invalid response content-type: got {ct:?}, expected {APPLICATION_MSGPACK:?}"
                )
            }
        }
    }
}

impl std::error::Error for MsgpackResponseError {
    fn description(&self) -> &str {
        "error reading msgpack body"
    }

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Network(e) => Some(e),
            Self::Serialization(e) => Some(e),
            Self::InvalidResponseContentType(_) => None,
        }
    }
}

#[allow(async_fn_in_trait)]
pub trait MsgpackResponse {
    // it would be nice if reqwest::Error::new were public so we could return that
    // instead of wrapping the error...
    async fn msgpack<T: DeserializeOwned>(self) -> Result<T, MsgpackResponseError>;
}

impl MsgpackResponse for reqwest::Response {
    async fn msgpack<T: DeserializeOwned>(self) -> Result<T, MsgpackResponseError> {
        if let Some(content_type) = self.headers().get(CONTENT_TYPE)
            && content_type != APPLICATION_MSGPACK
        {
            return Err(MsgpackResponseError::InvalidResponseContentType(
                content_type.clone(),
            ));
        }

        let full = self.bytes().await?;

        rmp_serde::from_slice(&full).map_err(Into::into)
    }
}
