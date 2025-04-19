// use axum::{Router, response::IntoResponse, routing::get};

// use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use axum::{response::IntoResponse, response::Response};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use std::sync::OnceLock;

static PROMETHEUS_HANDLE: OnceLock<PrometheusHandle> = OnceLock::new();

pub fn setup_metrics() {
    let builder = PrometheusBuilder::new();
    let handle = builder
        .install_recorder()
        .expect("Failed to install Prometheus recorder");
    PROMETHEUS_HANDLE.set(handle).ok();
}

pub async fn metrics_handler() -> Response {
    let handle = PROMETHEUS_HANDLE
        .get()
        .expect("Prometheus handle not initialized");
    handle.render().into_response()
}
