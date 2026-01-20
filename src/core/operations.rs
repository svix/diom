use serde::{Deserialize, Serialize};

use crate::AppState;

/// Operations as defined in the RFC https://github.com/svix/rfc/pull/30/files#diff-1f8e708b840474d3072b1c965eb090a3e30b26e8b7036c6e0ae47ef36ffb09abR67.
pub trait Operation<'a>: Serialize + Deserialize<'a> {
    type ApplyOutput;
    type ApplyError;

    #[allow(async_fn_in_trait)]
    async fn apply(self, state: &AppState) -> Self::ApplyOutput;
}
