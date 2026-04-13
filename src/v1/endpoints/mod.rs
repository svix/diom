macro_rules! namespace_request_input {
    ($ty:ty, $action:literal) => {
        impl diom_proto::RequestInput for $ty {
            fn access_metadata(&self) -> diom_proto::AccessMetadata<'_> {
                // Subject to change.
                // https://github.com/svix/diom-private/issues/758
                diom_proto::AccessMetadata::RuleProtected(diom_authorization::RequestedOperation {
                    module: diom_id::Module::AdminNamespace,
                    namespace: None,
                    key: Some(&self.name),
                    action: $action,
                })
            }
        }
    };
}

pub mod admin;
pub mod auth_token;
pub mod cache;
pub mod health;
pub mod idempotency;
pub mod kv;
pub mod msgs;
pub mod rate_limit;
