use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("Kubernetes API error: {0}")]
    Kube(#[from] kube::Error),

    #[error("Missing field: {0}")]
    MissingField(&'static str),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Invalid storage size: {0}")]
    InvalidStorageSize(String),

    #[error("Unexpected PVC storage state: {0}")]
    PvcStorageState(String),
}

pub(crate) type Result<T> = std::result::Result<T, Error>;
