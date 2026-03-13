use std::fmt;

use super::{Error, Result};

pub trait ResultExt<T, E> {
    /// If this `Result` is an `Err`, converts to error to an internal svix-server error.
    ///
    /// Use this instead of `.map_err(Error::internal)` to get proper backtraces.
    #[track_caller]
    fn or_internal_error(self) -> Result<T>
    where
        E: fmt::Display;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn or_internal_error(self) -> Result<T>
    where
        E: fmt::Display,
    {
        // Using `map_err` would lose `#[track_caller]` information
        match self {
            Ok(ok) => Ok(ok),
            Err(e) => Err(Error::internal(e)),
        }
    }
}
