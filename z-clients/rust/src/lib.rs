pub mod api;
mod client;
mod connector;
mod duration_ms_serde;
mod error;
pub mod models;
mod request;
mod serde_bytes_opt;
mod unix_timestamp_ms_serde;

pub(crate) use self::client::Configuration;
pub use self::{
    client::{DEFAULT_URL, DiomClient, DiomOptions},
    error::{
        ConnectionError, Error, ErrorKind, InvalidInput, OperationError, OtherError, Result,
        ServerError,
    },
};

#[cfg(test)]
mod tests {
    use crate::models::{
        AdminAuthTokenCreateIn, AdminAuthTokenExpireIn, AdminAuthTokenUpdateIn, KvSetIn, MsgIn,
        MsgQueueExtendLeaseIn, MsgQueueReceiveIn, MsgStreamReceiveIn, MsgStreamSeekIn,
        RateLimitConfig, Retention,
    };

    #[test]
    fn test_stream_receive_in_defaults() {
        let v: MsgStreamReceiveIn = serde_json::from_str("{}").unwrap();
        assert!(v.lease_duration.is_none());
        assert!(v.batch_wait.is_none());
    }

    #[test]
    fn test_queue_receive_in_defaults() {
        let v: MsgQueueReceiveIn = serde_json::from_str("{}").unwrap();
        assert!(v.lease_duration.is_none());
        assert!(v.batch_wait.is_none());
    }

    #[test]
    fn test_msg_in_defaults() {
        let v: MsgIn = serde_json::from_str(r#"{"value": ""}"#).unwrap();
        assert!(v.delay.is_none());
    }

    #[test]
    fn test_kv_set_in_defaults() {
        let v: KvSetIn = serde_json::from_str(r#"{"key": "k", "value": ""}"#).unwrap();
        assert!(v.ttl.is_none());
    }

    #[test]
    fn test_msg_queue_extend_lease_in_defaults() {
        let v: MsgQueueExtendLeaseIn = serde_json::from_str(r#"{"msg_ids": []}"#).unwrap();
        assert!(v.lease_duration.is_none());
    }

    #[test]
    fn test_msg_stream_seek_in_defaults() {
        let v: MsgStreamSeekIn = serde_json::from_str("{}").unwrap();
        assert!(v.timestamp.is_none());
    }

    #[test]
    fn test_admin_auth_token_create_in_defaults() {
        let v: AdminAuthTokenCreateIn =
            serde_json::from_str(r#"{"name": "n", "role": "r"}"#).unwrap();
        assert!(v.expiry.is_none());
    }

    #[test]
    fn test_admin_auth_token_update_in_defaults() {
        let v: AdminAuthTokenUpdateIn = serde_json::from_str(r#"{"id": "x"}"#).unwrap();
        assert!(v.expiry.is_none());
    }

    #[test]
    fn test_admin_auth_token_expire_in_defaults() {
        let v: AdminAuthTokenExpireIn = serde_json::from_str(r#"{"id": "x"}"#).unwrap();
        assert!(v.expiry.is_none());
    }

    #[test]
    fn test_retention_defaults() {
        let v: Retention = serde_json::from_str("{}").unwrap();
        assert!(v.period.is_none());
    }

    #[test]
    fn test_rate_limit_config_defaults() {
        let v: RateLimitConfig =
            serde_json::from_str(r#"{"capacity": 10, "refill_amount": 1}"#).unwrap();
        assert!(v.refill_interval.is_none());
    }
}
