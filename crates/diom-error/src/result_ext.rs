use std::fmt;

use super::{Error, Result};

pub trait ResultExt<T, E> {
    /// If this `Result` is an `Err`, converts to error to a generic svix-server error.
    ///
    /// Use this instead of `.map_err(Error::generic)` to get proper backtraces.
    #[track_caller]
    fn map_err_generic(self) -> Result<T>
    where
        E: fmt::Display;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn map_err_generic(self) -> Result<T>
    where
        E: fmt::Display,
    {
        // Using `map_err` would lose `#[track_caller]` information
        match self {
            Ok(ok) => Ok(ok),
            Err(e) => Err(Error::generic(e)),
        }
    }
}
