use std::{fmt, str::FromStr};

use diom_core::types::DurationMs;
use tap::Pipe;

pub(super) fn env_var<T, S>(name: S) -> anyhow::Result<Option<T>>
where
    S: AsRef<str>,
    T: FromStr<Err: fmt::Display>,
{
    env_var_parse(name, FromStr::from_str)
}

pub(super) fn env_var_ms<S>(name: S) -> anyhow::Result<Option<DurationMs>>
where
    S: AsRef<str>,
{
    env_var::<u64, S>(name)?.map(DurationMs::from).pipe(Ok)
}

pub(super) fn env_var_comma_separated<T, S>(name: S) -> anyhow::Result<Option<Vec<T>>>
where
    T: FromStr<Err: fmt::Display>,
    S: AsRef<str>,
{
    env_var_parse(name.as_ref(), |value| {
        value
            .split(',')
            .map(str::trim_ascii)
            .map(FromStr::from_str)
            .collect()
    })
}

pub(super) fn env_var_parse<S, T, E>(
    name: S,
    parse: impl FnOnce(&str) -> Result<T, E>,
) -> anyhow::Result<Option<T>>
where
    S: AsRef<str>,
    E: fmt::Display,
{
    let name = name.as_ref();
    match std::env::var(name) {
        Ok(value) => parse(&value)
            .map_err(|e| anyhow::anyhow!("invalid format for `{name}`: {e}"))
            .map(Some),
        Err(std::env::VarError::NotPresent) => Ok(None),
        Err(std::env::VarError::NotUnicode(_)) => Err(anyhow::anyhow!(
            "invalid format for `{name}`: invalid UTF-8"
        )),
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct Variable {
    pub env_var: String,
    pub docstring: Option<&'static str>,
}

const ENV_VAR_PREFIX: &str = "DIOM";

pub(crate) trait EnvOverridable {
    fn load_environment_with_prefix(&mut self, prefix: String) -> anyhow::Result<()>;

    fn load_environment(&mut self) -> anyhow::Result<()> {
        self.load_environment_with_prefix(ENV_VAR_PREFIX.to_owned())
    }

    fn list_environment_variables_with_prefix(prefix: String) -> Vec<Variable>;

    fn list_environment_variables() -> Vec<Variable> {
        Self::list_environment_variables_with_prefix(ENV_VAR_PREFIX.to_string())
    }
}
