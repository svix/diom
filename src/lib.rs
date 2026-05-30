#![warn(clippy::all)]

use std::sync::atomic::{AtomicBool, Ordering};

use diom_error::Error;

use crate::{cfg::Configuration, core::cluster::RaftState};
use diom_core::shutdown::{shutting_down_token, start_shut_down};

pub mod bootstrap;
pub mod cfg;
pub mod core;
mod serve;
pub use diom_error as error;
mod app_state;
pub mod metrics;
pub mod openapi;
mod utils;
pub mod v1;
mod workers;

pub(crate) use self::app_state::AppState;
pub use self::{
    serve::{run, run_with_listeners},
    utils::Initialized,
};

static TEST_TRACING_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub fn setup_tracing_for_tests() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    if TEST_TRACING_INITIALIZED.load(Ordering::Acquire) {
        return;
    }

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // Output is only printed for failing tests, but still we shouldn't overload
                // the output with unnecessary info. When debugging a specific test, it's easy
                // to override this default by setting the `RUST_LOG` environment variable.
                "diom=debug,fjall=info,it=debug,test_utils=debug".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer().with_test_writer())
        .init();
    TEST_TRACING_INITIALIZED.store(true, Ordering::Release);
}

#[cfg(test)]
#[ctor::ctor]
fn test_setup() {
    setup_tracing_for_tests();
}

mod docs {
    use aide::{axum::ApiRouter, openapi::OpenApi};
    use axum::{response::Redirect, routing::get};

    pub(crate) fn router(_docs: OpenApi) -> ApiRouter {
        cfg_select! {
            // For debug builds, read docs.html and openapi.json from disk on every request
            // so we don't have to rebuild when they change
            debug_assertions => {
                use tower_http::services::ServeFile;

                let docs_route = ServeFile::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/static/docs.html"));
                let openapi_json_route = ServeFile::new(concat!(env!("CARGO_MANIFEST_DIR"), "/openapi.json"));
            }
            // For release builds, embed both files in the binary for ease of deployment
            _ => {
                use axum::response::Html;

                let docs_route = get(|| async { Html(include_str!("static/docs.html")) });
                let openapi_json_route = get(|| async {
                    static BODY: &str = include_str!("../openapi.json");
                    ([(http::header::CONTENT_TYPE, "application/json")], BODY)
                });
            }
        }

        ApiRouter::new()
            .route("/", get(|| async { Redirect::temporary("/docs") }))
            .route_service("/docs", docs_route)
            .route_service("/api/v1/openapi.json", openapi_json_route)
            .with_state(_docs)
    }
}
