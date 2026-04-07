use std::time::Duration;

use jiff::Timestamp;

pub trait U64Ext {
    fn as_duration_ms(self) -> Duration;
    fn as_timestamp_ms(self) -> Timestamp;
}

impl U64Ext for u64 {
    fn as_duration_ms(self) -> Duration {
        Duration::from_millis(self)
    }

    #[track_caller]
    fn as_timestamp_ms(self) -> Timestamp {
        Timestamp::from_millisecond(self.try_into().unwrap()).unwrap()
    }
}
