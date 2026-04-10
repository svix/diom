use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("Kubernetes API error: {0}")]
    Kube(#[from] kube::Error),

    #[error("Missing field: {0}")]
    MissingField(&'static str),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Storage error: {0}")]
    Storage(String),
}

pub(crate) type Result<T> = std::result::Result<T, Error>;
