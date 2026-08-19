//! Minimal viewer binary for the `example` domain reference.
//!
//! Demonstrates the smallest viable viewer shape: `viewer-api`'s shared
//! server runtime plus a single domain route, with no dedicated frontend
//! build step.

use std::path::PathBuf;

use example::domain_name;
use viewer_api::{
    McpServerFactory,
    ServerConfig,
    axum::{
        Router,
        routing::get,
    },
    run_server,
};

#[derive(Clone)]
struct ExampleViewerState;

fn create_routes(
    state: ExampleViewerState,
    _static_dir: Option<PathBuf>,
) -> Router {
    Router::new()
        .route(
            "/health",
            get(|| async { format!("{}-viewer", domain_name()) }),
        )
        .with_state(state)
}

#[tokio::main]
async fn main() {
    let config = ServerConfig::new("example-viewer", 3099);
    let state = ExampleViewerState;
    run_server(
        config,
        state,
        create_routes,
        None::<McpServerFactory<ExampleViewerState>>,
    )
    .await
    .unwrap();
}
