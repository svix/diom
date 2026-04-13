use diom_core::types::{ByteString, Metadata};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum IdempotencyState {
    /// Request is in progress (locked)
    InProgress,
    /// Request completed successfully with a response
    Completed {
        response: ByteString,
        context: Option<Metadata>,
    },
}

impl From<IdempotencyState> for ByteString {
    fn from(state: IdempotencyState) -> Self {
        postcard::to_allocvec(&fjall_utils::V0Wrapper::V0(&state))
            .expect("Failed to serialize IdempotencyState")
            .into()
    }
}

impl From<ByteString> for IdempotencyState {
    fn from(value: ByteString) -> Self {
        postcard::from_bytes::<fjall_utils::V0Wrapper<IdempotencyState>>(&value)
            .map(|fjall_utils::V0Wrapper::V0(inner)| inner)
            .expect("Failed to deserialize IdempotencyState")
    }
}
