// this file is @generated
use crate::CoyoteClient;

mod cache;
mod kv;
mod queue;
mod rate_limiter;
mod stream;

pub use self::{
    cache::{Cache, CacheDeleteOptions, CacheGetOptions, CacheSetOptions},
    kv::{Kv, KvDeleteOptions, KvGetOptions, KvSetOptions},
    queue::{
        Queue, QueueAckOptions, QueueNackOptions, QueuePurgeOptions, QueueReceiveOptions,
        QueueRejectOptions, QueueSendOptions, QueueStatsOptions,
    },
    rate_limiter::{RateLimiter, RateLimiterGetRemainingOptions, RateLimiterLimitOptions},
    stream::{Stream, StreamAppendOptions, StreamCreateOptions},
};

impl CoyoteClient {
    pub fn cache(&self) -> Cache<'_> {
        Cache::new(&self.cfg)
    }
    pub fn kv(&self) -> Kv<'_> {
        Kv::new(&self.cfg)
    }
    pub fn queue(&self) -> Queue<'_> {
        Queue::new(&self.cfg)
    }
    pub fn rate_limiter(&self) -> RateLimiter<'_> {
        RateLimiter::new(&self.cfg)
    }
    pub fn stream(&self) -> Stream<'_> {
        Stream::new(&self.cfg)
    }
}
