// this file is @generated
mod admin;
mod admin_auth_policy;
mod admin_auth_role;
mod admin_auth_token;
mod admin_cluster;
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
mod rate_limit;
mod rate_limit_namespace;

pub(crate) use self::{
    admin::AdminArgs, admin_auth_policy::AdminAuthPolicyArgs, admin_auth_role::AdminAuthRoleArgs,
    admin_auth_token::AdminAuthTokenArgs, admin_cluster::AdminClusterArgs, cache::CacheArgs,
    cache_namespace::CacheNamespaceArgs, health::HealthArgs, idempotency::IdempotencyArgs,
    idempotency_namespace::IdempotencyNamespaceArgs, kv::KvArgs, kv_namespace::KvNamespaceArgs,
    msgs::MsgsArgs, msgs_namespace::MsgsNamespaceArgs, msgs_queue::MsgsQueueArgs,
    msgs_stream::MsgsStreamArgs, msgs_topic::MsgsTopicArgs, rate_limit::RateLimitArgs,
    rate_limit_namespace::RateLimitNamespaceArgs,
};
