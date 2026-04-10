use http::header::CONTENT_TYPE;
use serde::{Serialize, de::DeserializeOwned};
use tap::Pipe;

pub(crate) const APPLICATION_POSTCARD: http::HeaderValue =
    http::HeaderValue::from_static("application/x-postcard");

pub trait PostcardRequestBuilder: Sized {
    fn postcard<T: Serialize + ?Sized>(self, body: &T) -> Result<Self, postcard::Error>;
}

impl PostcardRequestBuilder for reqwest::RequestBuilder {
    fn postcard<T: Serialize + ?Sized>(self, body: &T) -> Result<Self, postcard::Error> {
        let serialized = postcard::to_allocvec(body)?;
        self.header(CONTENT_TYPE, APPLICATION_POSTCARD)
            .body(serialized)
            .pipe(Ok)
    }
}

#[derive(Debug)]
pub enum PostcardResponseError {
    Network(reqwest::Error),
    Serialization(postcard::Error),
    InvalidResponseContentType(http::HeaderValue),
}

impl From<reqwest::Error> for PostcardResponseError {
    fn from(value: reqwest::Error) -> Self {
        Self::Network(value)
    }
}

impl From<postcard::Error> for PostcardResponseError {
    fn from(value: postcard::Error) -> Self {
        Self::Serialization(value)
    }
}

impl std::fmt::Display for PostcardResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Network(e) => e.fmt(f),
            Self::Serialization(e) => e.fmt(f),
            Self::InvalidResponseContentType(ct) => {
                write!(
                    f,
                    "invalid response content-type: got {ct:?}, expected {APPLICATION_POSTCARD:?}"
                )
            }
        }
    }
}

impl std::error::Error for PostcardResponseError {
    fn description(&self) -> &str {
        "error reading postcard body"
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
pub trait PostcardResponse {
    // it would be nice if reqwest::Error::new were public so we could return that
    // instead of wrapping the error...
    async fn postcard<T: DeserializeOwned>(self) -> Result<T, PostcardResponseError>;
}

impl PostcardResponse for reqwest::Response {
    async fn postcard<T: DeserializeOwned>(self) -> Result<T, PostcardResponseError> {
        if let Some(content_type) = self.headers().get(CONTENT_TYPE) {
            if content_type != APPLICATION_POSTCARD {
                return Err(PostcardResponseError::InvalidResponseContentType(
                    content_type.clone(),
                ));
            }
        }

        let full = self.bytes().await?;

        postcard::from_bytes(&full).map_err(Into::into)
    }
}
