
use std::{
    sync::LazyLock,
    time::{SystemTime, UNIX_EPOCH},
};

use aide::transform::{TransformOperation, TransformPathItem};
use diom_core::validation::validation_error;
use regex::Regex;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::error::Result;

pub fn validate_no_control_characters(str: &str) -> Result<(), ValidationError> {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[\x00-\x08]").unwrap());
    if RE.is_match(str) {
        return Err(validation_error(
            Some("illegal_character"),
            Some("Control characters 0x00-0x08 not allowed."),
        ));
    }
    Ok(())
}

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

#[derive(Clone, Serialize, Deserialize, Validate, JsonSchema)]
pub struct Pagination<T> {
    /// Limit the number of returned items
    #[validate(nested)]
    #[serde(default)]
    // This needs to be manually kept in sync with 'PAGINATION_LIMIT_CAP_LIMIT',
    // since schemars requires a literal
    #[schemars(range(min = 1, max = 250))]
    pub limit: PaginationLimit,
    /// The iterator returned from a prior invocation
    // FIXME: #[validate(nested)]
    pub iterator: Option<T>,
}

pub const PAGINATION_LIMIT_CAP_LIMIT: u64 = 250;
static PAGINATION_LIMIT_ERROR: LazyLock<String> =
    LazyLock::new(|| format!("Given limit must not exceed {PAGINATION_LIMIT_CAP_LIMIT}"));

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

        Ok(PaginationLimit(limit.min(PAGINATION_LIMIT_CAP_LIMIT)))
    }
}

impl Validate for PaginationLimit {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        let mut errs = validator::ValidationErrors::new();

        if self.0 > PAGINATION_LIMIT_CAP_LIMIT {
            errs.add(
                "limit",
                validation_error(Some("pagination"), Some(&PAGINATION_LIMIT_ERROR)),
            );
        }

        if errs.is_empty() { Ok(()) } else { Err(errs) }
    }
}

#[cfg(test)]
mod tests {
    use diom_core::validation::{ValidationErrorItem, validation_errors};
    use validator::Validate;

    use super::validate_no_control_characters;

    #[derive(Debug, Validate)]
    struct ValidationErrorTestStruct {
        #[validate(range(min = 10, message = "Below 10"))]
        a: u32,

        #[validate(nested)]
        b: ValidationErrorTestStructInner,

        #[validate(nested)]
        c: Vec<ValidationErrorTestStructInner>,
    }

    #[derive(Debug, Validate)]
    struct ValidationErrorTestStructInner {
        #[validate(range(max = 10, message = "Above 10"))]
        inner: u8,
    }

    #[test]
    fn test_validation_errors_fn() {
        let valid = ValidationErrorTestStruct {
            a: 11,
            b: ValidationErrorTestStructInner { inner: 1 },
            c: vec![
                ValidationErrorTestStructInner { inner: 2 },
                ValidationErrorTestStructInner { inner: 3 },
            ],
        };
        let invalid = ValidationErrorTestStruct {
            a: 9,
            b: ValidationErrorTestStructInner { inner: 11 },
            c: vec![
                ValidationErrorTestStructInner { inner: 12 },
                ValidationErrorTestStructInner { inner: 13 },
            ],
        };

        assert_eq!(valid.validate(), Ok(()));

        let errs = invalid.validate().unwrap_err();
        let errs = validation_errors(vec![], errs);

        assert_eq!(errs.len(), 4);

        assert!(errs.contains(&ValidationErrorItem {
            loc: vec!["a".to_owned()],
            msg: "Below 10".to_owned(),
            ty: "value_error".to_owned(),
        }));

        assert!(errs.contains(&ValidationErrorItem {
            loc: vec!["b".to_owned(), "inner".to_owned()],
            msg: "Above 10".to_owned(),
            ty: "value_error".to_owned(),
        }));

        assert!(errs.contains(&ValidationErrorItem {
            loc: vec!["c".to_owned(), "[0]".to_owned(), "inner".to_owned()],
            msg: "Above 10".to_owned(),
            ty: "value_error".to_owned(),
        }));
        assert!(errs.contains(&ValidationErrorItem {
            loc: vec!["c".to_owned(), "[1]".to_owned(), "inner".to_owned()],
            msg: "Above 10".to_owned(),
            ty: "value_error".to_owned(),
        }));
    }

    #[test]
    fn test_validate_no_control_characters() {
        let a = "A good string";
        let b = "A\u{0000} bad string";

        assert!(validate_no_control_characters(a).is_ok());
        assert!(validate_no_control_characters(b).is_err());
    }
}
