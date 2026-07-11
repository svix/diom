use std::{io::Read, str::FromStr};

use anyhow::{Context, Error, Result};
use serde::{Serialize, de::DeserializeOwned};

mod format;

use self::format::PrettyFormatter;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct JsonOf<T>(T);

impl<T: DeserializeOwned> FromStr for JsonOf<T> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "-" {
            let mut stdin = std::io::stdin().lock();
            let mut input = String::new();
            stdin
                .read_to_string(&mut input)
                .context("Error reading stdin for '-' argument")?;
            Ok(JsonOf(serde_json::from_str(&input)?))
        } else {
            Ok(JsonOf(serde_json::from_str(s)?))
        }
    }
}

impl<T> JsonOf<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

pub fn print_json_output<T>(val: &T) -> Result<()>
where
    T: Serialize,
{
    let mut stdout = std::io::stdout().lock();
    let mut serializer =
        serde_json::Serializer::with_formatter(&mut stdout, PrettyFormatter::default());
    match val.serialize(&mut serializer) {
        Ok(_) => Ok(()),
        Err(e) if matches!(e.io_error_kind(), Some(std::io::ErrorKind::BrokenPipe)) => Ok(()),
        Err(e) => Err(e.into()),
    }
}
