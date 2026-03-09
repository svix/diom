use crate::{HttpError, Result};

pub trait OptionExt<T>: Sized {
    fn ok_or_not_found(self) -> Result<T>;
}

impl<T> OptionExt<T> for Option<T> {
    fn ok_or_not_found(self) -> Result<T> {
        self.ok_or_else(|| HttpError::not_found(None::<String>))
            .map_err(Into::into)
    }
}
