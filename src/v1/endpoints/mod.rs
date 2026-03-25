// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

macro_rules! admin_request_input {
    ($ty:ty) => {
        impl coyote_proto::RequestInput for $ty {
            fn access_metadata(&self) -> coyote_proto::AccessMetadata<'_> {
                coyote_proto::AccessMetadata::AdminOnly
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
pub mod queue;
pub mod rate_limit;
