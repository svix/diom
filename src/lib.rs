use axum::Router;
use tower_http::trace::TraceLayer;

mod routes;

#[derive(Clone)]
pub struct AppState {}

pub async fn run() {
    let app = Router::new()
        .nest_service("/api/v1", routes::router())
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8050").await.unwrap();
    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
