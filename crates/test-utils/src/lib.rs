use std::fmt::{Debug, Display};

mod test_client;

pub use test_client::{TestClient, TestRequestBuilder, TestResponse};

#[derive(Debug)] // needed to be able to return TestResult from #[test] fns
pub enum TestError {}

// If the bound is just `T: Debug`, this impl conflicts with the blanket
// `impl From<T> for T` in the standard library.
//
// Can't use `T: std::error::Error` either because anyhow doesn't implement
// `std::error::Error` (also to avoid a conflicting From impl).
//
// Using `Display + Debug`, even though we only use `Debug`, works around this.
impl<T: Display + Debug> From<T> for TestError {
    #[track_caller]
    fn from(value: T) -> Self {
        panic!("error: {value:?}")
    }
}

pub type TestResult<T = ()> = Result<T, TestError>;
