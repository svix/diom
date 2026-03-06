// this file is @generated
mod cache;
mod cache_namespace;
mod health;
mod idempotency;
mod idempotency_namespace;
mod kv;
mod kv_namespace;
mod msgs;
mod msgs_namespace;
mod msgs_queue;
mod msgs_stream;
mod msgs_topic;
mod rate_limiter;

pub(crate) use self::{
    cache::CacheArgs, cache_namespace::CacheNamespaceArgs, health::HealthArgs,
    idempotency::IdempotencyArgs, idempotency_namespace::IdempotencyNamespaceArgs, kv::KvArgs,
    kv_namespace::KvNamespaceArgs, msgs::MsgsArgs, msgs_namespace::MsgsNamespaceArgs,
    msgs_queue::MsgsQueueArgs, msgs_stream::MsgsStreamArgs, msgs_topic::MsgsTopicArgs,
    rate_limiter::RateLimiterArgs,
};
