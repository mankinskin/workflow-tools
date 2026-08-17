use std::net::SocketAddr;

use transport_harness::{
    HarnessError,
    http::{
        Router,
        axum::routing::get,
    },
};

fn main() -> Result<(), HarnessError> {
    let address = std::env::var("EXAMPLE_HTTP_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:3000".into())
        .parse::<SocketAddr>()
        .map_err(HarnessError::domain)?;
    let router = Router::new().route(
        "/health",
        get(|| async { format!("{}-http", example::domain_name()) }),
    );
    transport_harness::http::run(address, router)
}
