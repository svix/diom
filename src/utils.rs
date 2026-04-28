use std::{
    fmt,
    net::SocketAddr,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use axum::{
    middleware,
    response::IntoResponse as _,
    serve::{Listener, ListenerExt as _},
};
use diom_core::shutdown::{shutting_down_token, start_shut_down};
use diom_error::Error;
use tokio::net::TcpListener;

use crate::core::metrics::{ConnectionMetrics, ConnectionType};

#[derive(Debug, Clone)]
pub struct Initialized {
    inner: Arc<tokio::sync::SetOnce<()>>,
}

impl Default for Initialized {
    fn default() -> Self {
        Self::new()
    }
}

impl Initialized {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(tokio::sync::SetOnce::new()),
        }
    }

    pub(crate) fn set(&self) -> Result<(), tokio::sync::SetOnceError<()>> {
        self.inner.set(())
    }

    // Wait until initialization is finished or the server shuts down
    pub async fn wait(self) -> diom_error::Result<()> {
        let shutting_down_token = shutting_down_token();
        shutting_down_token
            .run_until_cancelled(self.inner.wait())
            .await
            .copied()
            .ok_or_else(Error::shutting_down)
    }
}

pub(crate) static BOOTSTRAPPED: AtomicBool = AtomicBool::new(false);

pub(crate) async fn fail_until_bootstrapped(
    path: axum::extract::MatchedPath,
    request: axum::extract::Request,
    next: middleware::Next,
) -> axum::response::Response {
    let is_admin_route = path.as_str().starts_with("/api/v1.admin.cluster.");
    if !(is_admin_route || BOOTSTRAPPED.load(Ordering::Relaxed)) {
        return Error::not_ready("this node has not yet finished bootstrapping").into_response();
    }

    next.run(request).await
}

pub(crate) async fn graceful_shutdown_handler() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let sigterm = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let sigterm = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = sigterm => {},
    }

    tracing::info!("Received shutdown signal. Shutting down gracefully...");
    start_shut_down();
}

pub(crate) fn handle_panic(
    err: Box<dyn std::any::Any + Send + 'static>,
) -> axum::response::Response {
    if let Some(err) = err.downcast_ref::<String>() {
        tracing::error!(?err, "Unhandled panic");
    } else if let Some(err) = err.downcast_ref::<&'static str>() {
        tracing::error!(?err, "Unhandled panic");
    } else {
        tracing::error!("Unhandled non-string panic");
    }
    Error::internal("unhandled internal panic").into_response()
}

pub(crate) async fn axum_tcp_listener(
    listener: Option<TcpListener>,
    listen_address: SocketAddr,
    conn_metrics: ConnectionMetrics,
    connection_type: ConnectionType,
) -> impl Listener<Addr: fmt::Display + fmt::Debug> {
    let listener = match listener {
        Some(l) => l,
        None => TcpListener::bind(listen_address)
            .await
            .expect("Error binding to listen_address"),
    };

    listener.tap_io(move |tcp_stream| {
        if let Err(err) = tcp_stream.set_nodelay(true) {
            tracing::warn!("failed to set TCP_NODELAY on incoming connection: {err:#}");
        }
        conn_metrics.accepted(connection_type);
    })
}
