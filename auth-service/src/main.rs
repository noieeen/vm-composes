mod db;
mod models;
mod monitoring;
mod services;
mod utils;
use axum::{
    Router,
    routing::{get, post},
};
use db::Db;
use dotenvy::dotenv;
use monitoring::metrics::{metrics_handler, setup_metrics};
use monitoring::tracing::tracing_start;
use services::auth::{get_users, login, register};
use std::env;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use metrics_exporter_prometheus::PrometheusBuilder; // Added import for PrometheusBuilder
use axum::{response::IntoResponse, response::Response}; // Added imports for response handling
use std::sync::OnceLock; // Added import for OnceLock

#[tokio::main]
async fn main() {
    // 1. Initialize Prometheus metrics exporter
    setup_metrics();
    tracing_start();

    dotenv().ok();

    // tracing_subscriber::registry()
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();

    let db = match Db::new().await {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Database initialization failed: {}", e);
            std::process::exit(1); // Optional: exit app if DB is critical
        }
    };

    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");

    let app = Router::new()
        .layer(TraceLayer::new_for_http())
        .route("/health", post(|| async { "OK" }))
        .route("/", get(|| async { "Hello, World!" }))
        .route("/metrics", get(metrics_handler)) // expose metrics here
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/users", get(get_users))
        .with_state(db.clone());

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port))
        .await
        .unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
