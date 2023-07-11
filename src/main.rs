use axum::{extract::Extension, routing::get, Router};

use std::{net::SocketAddr, time::Duration};

pub mod api;
pub mod config;
pub mod errors;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let agent: ureq::Agent = ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(10))
        .timeout_write(Duration::from_secs(10))
        .build();

    let api_routes = Router::new()
        .route("/sources", get(api::list_sources))
        .route("/source/:source/builds/:champion", get(api::get_lastest_build))
        .route("/source/:source/runes/:champion", get(api::get_lastest_build))
        .route("/data-dragon/champions", get(api::list_champion_map))
        .route("/data-dragon/runes", get(api::list_runes_reforged))
        .layer(Extension(agent));

    let app = Router::new().nest("/api", api_routes);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3030));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
