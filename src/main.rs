mod auth;
mod routes;
mod templates;
mod schema;
mod models;
mod db;

use axum::Router;
use dotenv::dotenv;
use tower_http::{
    services::ServeDir,
    trace::TraceLayer,
};

#[tokio::main]
async fn main() {
    if cfg!(debug_assertions) {
        dotenv().ok();
    }

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app = Router::new()
        .layer(TraceLayer::new_for_http())
        .merge(public_assets())
        .merge(routes::routes());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    tracing::info!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn public_assets() -> Router {
    Router::new()
        .nest_service("/public", ServeDir::new("static"))
}
