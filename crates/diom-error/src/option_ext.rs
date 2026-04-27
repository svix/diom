use std::fmt::Display;

use crate::{Error, Result};

pub trait OptionExt<T>: Sized {
    fn ok_or_not_found(self, entity: &'static str) -> Result<T>;
    fn ok_or_internal_error(self, msg: impl Display) -> Result<T>;
}

impl<T> OptionExt<T> for Option<T> {
    fn ok_or_not_found(self, entity: &'static str) -> Result<T> {
        self.ok_or_else(|| Error::entity_not_found(entity))
    }

    #[track_caller]
    fn ok_or_internal_error(self, msg: impl Display) -> Result<T> {
        // not using ok_or_else to not lose track_caller location
        match self {
            Some(v) => Ok(v),
            None => Err(Error::internal(msg)),
        }
    }
}
