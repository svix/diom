#![allow(clippy::disallowed_types)]

pub trait JsonFastAndLoose {
    fn assert_u64(&self) -> u64;
    fn assert_str(&self) -> &str;
    fn assert_array(&self) -> &[serde_json::Value];
    fn assert_bytes(&self) -> Vec<u8>;
    fn assert_bool(&self) -> bool;
}

impl JsonFastAndLoose for serde_json::Value {
    #[track_caller]
    fn assert_u64(&self) -> u64 {
        self.as_u64().unwrap()
    }

    #[track_caller]
    fn assert_bytes(&self) -> Vec<u8> {
        self.assert_array()
            .iter()
            .map(|a| a.assert_u64().try_into().unwrap())
            .collect()
    }

    #[track_caller]
    fn assert_str(&self) -> &str {
        self.as_str().unwrap()
    }

    #[track_caller]
    fn assert_array(&self) -> &[serde_json::Value] {
        self.as_array().unwrap()
    }

    #[track_caller]
    fn assert_bool(&self) -> bool {
        self.as_bool().unwrap()
    }
}
