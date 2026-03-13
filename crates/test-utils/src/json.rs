#![allow(clippy::disallowed_types)]

pub trait JsonFastAndLoose {
    fn assert_u64(&self) -> u64;
    fn assert_str(&self) -> &str;
    fn assert_array(&self) -> &[serde_json::Value];
}

impl JsonFastAndLoose for serde_json::Value {
    #[track_caller]
    fn assert_u64(&self) -> u64 {
        self.as_u64().unwrap()
    }

    #[track_caller]
    fn assert_str(&self) -> &str {
        self.as_str().unwrap()
    }

    #[track_caller]
    fn assert_array(&self) -> &[serde_json::Value] {
        self.as_array().unwrap()
    }
}
