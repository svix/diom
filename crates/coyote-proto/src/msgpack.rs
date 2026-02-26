use axum::{
    extract::{FromRequest, OptionalFromRequest, Request, rejection::BytesRejection},
    response::{IntoResponse, Response},
};
use bytes::{BufMut, Bytes, BytesMut};
use http::{
    StatusCode,
    header::{self, HeaderMap, HeaderValue},
};
use serde::{Serialize, de::DeserializeOwned};

#[derive(Debug, Clone)]
pub struct MissingMsgPackContentType;

impl IntoResponse for MissingMsgPackContentType {
    fn into_response(self) -> Response {
        let status = StatusCode::BAD_REQUEST;

        tracing::warn!(?status, "missing content-type");

        (
            status,
            "Expected request with `Content-Type: application/msgpack`",
        )
            .into_response()
    }
}

#[derive(Debug)]
pub struct MsgPackParseError(rmp_serde::decode::Error);

impl IntoResponse for MsgPackParseError {
    fn into_response(self) -> Response {
        let status = StatusCode::BAD_REQUEST;

        tracing::warn!(?status, error = ?self.0, "parse error");

        (status, "Failed to parse the request body as msgpack").into_response()
    }
}

#[derive(Debug)]
pub enum MsgPackRejection {
    MissingMsgPackContentType(MissingMsgPackContentType),
    MsgPackParseError(MsgPackParseError),
    BytesRejection(BytesRejection),
}

impl IntoResponse for MsgPackRejection {
    fn into_response(self) -> Response {
        match self {
            Self::MissingMsgPackContentType(r) => r.into_response(),
            Self::MsgPackParseError(r) => r.into_response(),
            Self::BytesRejection(r) => r.into_response(),
        }
    }
}

impl From<MissingMsgPackContentType> for MsgPackRejection {
    fn from(value: MissingMsgPackContentType) -> Self {
        Self::MissingMsgPackContentType(value)
    }
}

impl From<MsgPackParseError> for MsgPackRejection {
    fn from(value: MsgPackParseError) -> Self {
        Self::MsgPackParseError(value)
    }
}

impl From<BytesRejection> for MsgPackRejection {
    fn from(value: BytesRejection) -> Self {
        Self::BytesRejection(value)
    }
}

/// MsgPack Extractor / Response.
///
/// When used as an extractor, it can deserialize request bodies into some type that
/// implements [`serde::de::DeserializeOwned`]. The request will be rejected (and a [`MsgPackRejection`] will
/// be returned) if:
///
/// - The request doesn't have a `Content-Type: application/MsgPack` (or similar) header.
/// - The body doesn't contain syntactically valid MsgPack.
/// - The body contains syntactically valid MsgPack, but it couldn't be deserialized into the target type.
/// - Buffering the request body fails.
///
/// ⚠️ Since parsing MsgPack requires consuming the request body, the `MsgPack` extractor must be
/// *last* if there are multiple extractors in a handler.
#[derive(Debug, Clone, Copy, Default)]
#[must_use]
pub struct MsgPack<T>(pub T);

impl<T, S> FromRequest<S> for MsgPack<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = MsgPackRejection;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        if !msgpack_content_type(req.headers()) {
            return Err(MissingMsgPackContentType.into());
        }

        let bytes = Bytes::from_request(req, state).await?;
        Self::from_bytes(&bytes)
    }
}

impl<T, S> OptionalFromRequest<S> for MsgPack<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = MsgPackRejection;

    async fn from_request(req: Request, state: &S) -> Result<Option<Self>, Self::Rejection> {
        let headers = req.headers();
        if headers.get(header::CONTENT_TYPE).is_some() {
            if msgpack_content_type(headers) {
                let bytes = Bytes::from_request(req, state).await?;
                Ok(Some(Self::from_bytes(&bytes)?))
            } else {
                Err(MissingMsgPackContentType.into())
            }
        } else {
            Ok(None)
        }
    }
}

fn msgpack_content_type(headers: &HeaderMap) -> bool {
    headers
        .get(header::CONTENT_TYPE)
        .and_then(|content_type| content_type.to_str().ok())
        .and_then(|content_type| content_type.parse::<mime::Mime>().ok())
        .is_some_and(|mime| mime.type_() == "application" && mime.subtype() == "msgpack")
}

impl<T> std::ops::Deref for MsgPack<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for MsgPack<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<T> for MsgPack<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}

impl<T> MsgPack<T>
where
    T: DeserializeOwned,
{
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, MsgPackRejection> {
        rmp_serde::from_slice(bytes)
            .map_err(|e| MsgPackRejection::MsgPackParseError(MsgPackParseError(e)))
            .map(Self)
    }
}

impl<T> IntoResponse for MsgPack<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        // Extracted into separate fn so it's only compiled once for all T.
        fn make_response(
            buf: BytesMut,
            ser_result: Result<(), rmp_serde::encode::Error>,
        ) -> Response {
            match ser_result {
                Ok(()) => (
                    [(
                        header::CONTENT_TYPE,
                        HeaderValue::from_static(mime::APPLICATION_MSGPACK.as_ref()),
                    )],
                    buf.freeze(),
                )
                    .into_response(),
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

        let mut buf = BytesMut::with_capacity(128).writer();
        let res = rmp_serde::encode::write_named(&mut buf, &self.0);
        make_response(buf.into_inner(), res)
    }
}
