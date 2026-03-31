// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

macro_rules! namespace_request_input {
    ($ty:ty, $action:literal) => {
        impl coyote_proto::RequestInput for $ty {
            fn access_metadata(&self) -> coyote_proto::AccessMetadata<'_> {
                // Subject to change.
                // https://github.com/svix/coyote-private/issues/758
                coyote_proto::AccessMetadata::RuleProtected(
                    coyote_authorization::RequestedOperation {
                        module: coyote_id::Module::AdminNamespace,
                        namespace: None,
                        key: Some(&self.name),
                        action: $action,
                    },
                )
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
