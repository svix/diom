use axum::{
    extract::{FromRequest, OptionalFromRequest, Request, rejection::BytesRejection},
    response::{IntoResponse, Response},
};
use bytes::{Bytes, BytesMut};
use http::{
    StatusCode,
    header::{self, HeaderMap, HeaderValue},
};
use serde::{Serialize, de::DeserializeOwned};

const APPLICATION_POSTCARD: HeaderValue = HeaderValue::from_static("application/x-postcard");

#[derive(Debug, Clone)]
pub struct MissingPostCardContentType;

impl IntoResponse for MissingPostCardContentType {
    fn into_response(self) -> Response {
        let status = StatusCode::BAD_REQUEST;

        tracing::warn!(?status, "missing content-type");

        (
            status,
            "Expected request with `Content-Type: application/x-postcard`",
        )
            .into_response()
    }
}

#[derive(Debug)]
pub struct PostCardParseError(postcard::Error);

impl IntoResponse for PostCardParseError {
    fn into_response(self) -> Response {
        let status = StatusCode::BAD_REQUEST;

        tracing::warn!(?status, error = ?self.0, "parse error");

        (status, "Failed to parse the request body as postcard").into_response()
    }
}

#[derive(Debug)]
pub enum PostCardRejection {
    MissingPostCardContentType(MissingPostCardContentType),
    PostCardParseError(PostCardParseError),
    BytesRejection(BytesRejection),
}

impl IntoResponse for PostCardRejection {
    fn into_response(self) -> Response {
        match self {
            Self::MissingPostCardContentType(r) => r.into_response(),
            Self::PostCardParseError(r) => r.into_response(),
            Self::BytesRejection(r) => r.into_response(),
        }
    }
}

impl From<MissingPostCardContentType> for PostCardRejection {
    fn from(value: MissingPostCardContentType) -> Self {
        Self::MissingPostCardContentType(value)
    }
}

impl From<PostCardParseError> for PostCardRejection {
    fn from(value: PostCardParseError) -> Self {
        Self::PostCardParseError(value)
    }
}

impl From<BytesRejection> for PostCardRejection {
    fn from(value: BytesRejection) -> Self {
        Self::BytesRejection(value)
    }
}

/// PostCard Extractor / Response.
///
/// When used as an extractor, it can deserialize request bodies into some type that
/// implements [`serde::de::DeserializeOwned`]. The request will be rejected (and a [`PostCardRejection`] will
/// be returned) if:
///
/// - The request doesn't have a `Content-Type: application/PostCard` (or similar) header.
/// - The body doesn't contain syntactically valid PostCard.
/// - The body contains syntactically valid PostCard, but it couldn't be deserialized into the target type.
/// - Buffering the request body fails.
///
/// ⚠️ Since parsing PostCard requires consuming the request body, the `PostCard` extractor must be
/// *last* if there are multiple extractors in a handler.
#[derive(Debug, Clone, Copy, Default)]
#[must_use]
pub struct PostCard<T>(pub T);

impl<T, S> FromRequest<S> for PostCard<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = PostCardRejection;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        if !postcard_content_type(req.headers()) {
            return Err(MissingPostCardContentType.into());
        }

        let bytes = Bytes::from_request(req, state).await?;
        Self::from_bytes(&bytes)
    }
}

impl<T, S> OptionalFromRequest<S> for PostCard<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = PostCardRejection;

    async fn from_request(req: Request, state: &S) -> Result<Option<Self>, Self::Rejection> {
        let headers = req.headers();
        if headers.get(header::CONTENT_TYPE).is_some() {
            if postcard_content_type(headers) {
                let bytes = Bytes::from_request(req, state).await?;
                Ok(Some(Self::from_bytes(&bytes)?))
            } else {
                Err(MissingPostCardContentType.into())
            }
        } else {
            Ok(None)
        }
    }
}

fn postcard_content_type(headers: &HeaderMap) -> bool {
    headers
        .get(header::CONTENT_TYPE)
        .is_some_and(|c| c == APPLICATION_POSTCARD)
}

impl<T> std::ops::Deref for PostCard<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for PostCard<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<T> for PostCard<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}

impl<T> PostCard<T>
where
    T: DeserializeOwned,
{
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, PostCardRejection> {
        postcard::from_bytes(bytes)
            .map_err(|e| PostCardRejection::PostCardParseError(PostCardParseError(e)))
            .map(Self)
    }
}

impl<T> IntoResponse for PostCard<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        // Extracted into separate fn so it's only compiled once for all T.
        fn make_response(ser_result: Result<BytesMut, postcard::Error>) -> Response {
            match ser_result {
                Ok(buf) => {
                    ([(header::CONTENT_TYPE, APPLICATION_POSTCARD)], buf.freeze()).into_response()
                }
                Err(err) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    [(
                        header::CONTENT_TYPE,
                        HeaderValue::from_static(mime::TEXT_PLAIN_UTF_8.as_ref()),
                    )],
                    err.to_string(),
                )
                    .into_response(),
            }
        }

        let buf = BytesMut::with_capacity(128);
        let res = postcard::to_extend(&self.0, buf);
        make_response(res)
    }
}
