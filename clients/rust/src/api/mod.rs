// this file is @generated
use crate::CoyoteClient;

mod cache;
mod health;
mod kv;
mod rate_limiter;
mod stream;

pub use self::{
    cache::{Cache, CacheDeleteOptions, CacheGetOptions, CacheSetOptions},
    health::Health,
    kv::{Kv, KvDeleteOptions, KvGetOptions, KvSetOptions},
    rate_limiter::{RateLimiter, RateLimiterGetRemainingOptions, RateLimiterLimitOptions},
    stream::{
        Stream, StreamAckOptions, StreamAckRangeOptions, StreamAppendOptions, StreamCreateOptions,
        StreamDlqOptions, StreamFetchLockingOptions, StreamFetchOptions, StreamRedriveOptions,
    },
};

impl CoyoteClient {
    pub fn cache(&self) -> Cache<'_> {
        Cache::new(&self.cfg)
    }

    pub fn health(&self) -> Health<'_> {
        Health::new(&self.cfg)
    }

    pub fn kv(&self) -> Kv<'_> {
        Kv::new(&self.cfg)
    }

    pub fn rate_limiter(&self) -> RateLimiter<'_> {
        RateLimiter::new(&self.cfg)
    }

    pub fn stream(&self) -> Stream<'_> {
        Stream::new(&self.cfg)
    }
}
