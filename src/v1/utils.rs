use std::time::{SystemTime, UNIX_EPOCH};

use aide::transform::{TransformOperation, TransformPathItem};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::Result;

pub fn openapi_tag<T: AsRef<str>>(
    tag: T,
) -> impl Fn(TransformPathItem<'_>) -> TransformPathItem<'_> {
    move |op| op.tag(tag.as_ref())
}

pub fn openapi_desc<T: AsRef<str>>(
    desc: T,
) -> impl Fn(TransformOperation<'_>) -> TransformOperation<'_> {
    move |op| op.description(desc.as_ref())
}

pub fn get_unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub trait ListResponseItem {
    fn id(&self) -> String;
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "ListResponse{T}")]
pub struct ListResponse<T> {
    pub data: Vec<T>,
    pub iterator: Option<String>,
    pub prev_iterator: Option<String>,
    pub done: bool,
}

impl<T: ListResponseItem> ListResponse<T> {
    pub fn empty() -> Self {
        Self {
            data: Vec::new(),
            iterator: None,
            prev_iterator: None,
            done: true,
        }
    }

    pub fn create(
        mut data: Vec<T>,
        limit: usize,
        used_iterator: Option<String>,
    ) -> ListResponse<T> {
        // Our queries use a LIMIT of (limit + 1), so if there is more data than
        // the user requested, `data.len()` is going to be larger than limit.
        let done = data.len() <= limit;

        // Drop the excess element(s). Should be only one.
        data.truncate(limit);

        let iterator = data
            .last()
            .map(|x| x.id())
            .or_else(|| used_iterator.clone());
        let prev_iterator = data
            .first()
            .map(|x| x.id())
            .or_else(|| iterator.clone())
            .map(|x| format!("-{x}"));

        ListResponse {
            data,
            iterator,
            prev_iterator,
            done,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
pub struct Pagination<T> {
    /// Limit the number of returned items
    #[serde(default)]
    // This needs to be manually kept in sync with the `Deserialize` impl,
    // since schemars requires a literal
    #[schemars(range(min = 1, max = 250))]
    pub limit: PaginationLimit,
    /// The iterator returned from a prior invocation
    pub iterator: Option<T>,
}

#[derive(Clone, Serialize, Copy, Debug, JsonSchema)]
#[schemars(transparent)]
pub struct PaginationLimit(pub u64);

impl Default for PaginationLimit {
    fn default() -> Self {
        Self(50)
    }
}

impl From<PaginationLimit> for usize {
    fn from(value: PaginationLimit) -> Self {
        value
            .0
            .try_into()
            .expect("u64 to usize should be lossless on our platforms")
    }
}

impl<'de> Deserialize<'de> for PaginationLimit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let limit = u64::deserialize(deserializer)?;

        if limit < 1 {
            return Err(serde::de::Error::custom(
                "Pagination limit must be at least 1",
            ));
        }
        if limit > 250 {
            return Err(serde::de::Error::custom(
                "Pagination limit be no larger than 250",
            ));
        }

        Ok(PaginationLimit(limit))
    }
}
