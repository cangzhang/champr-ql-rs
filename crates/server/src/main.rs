use axum::{extract::Extension, routing::get, Router};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection};
use dotenvy::dotenv;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use std::{env, net::SocketAddr, time::Duration};

pub mod config;
pub mod errors;
pub mod handler;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<AsyncPgConnection>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    // initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let pool = db::make_db_pool()?;

    let agent: ureq::Agent = ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(10))
        .timeout_write(Duration::from_secs(10))
        .build();

    let api_routes = Router::new()
        .route("/sources", get(handler::list_sources))
        .route(
            "/source/:source/builds/:champion",
            get(handler::get_lastest_build),
        )
        .route(
            "/source/:source/runes/:champion",
            get(handler::get_lastest_build),
        )
        .route("/data-dragon/champions", get(handler::list_champion_map))
        .route("/data-dragon/runes", get(handler::list_runes_reforged))
        .layer(Extension(pool))
        .layer(Extension(agent));

    let app = Router::new().nest("/api", api_routes).layer(
        TraceLayer::new_for_http()
            .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
            .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
    );

    let port = env::var("PORT")
        .unwrap_or_else(|_| "3030".to_string())
        .parse()
        .unwrap();
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
