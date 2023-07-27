mod api;
mod types;
mod utils;

use std::{net::SocketAddr, sync::Arc, time::Duration};

use axum::{
    routing::{delete, get, post},
    Router,
};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use tokio::sync::broadcast::{self, Sender};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting oorah-broadcaster...");

    match run().await {
        Ok(_) => tracing::info!("Program exited successfully."),
        Err(e) => tracing::error!("Error: {}", e),
    }
}

async fn run() -> anyhow::Result<()> {
    let db_connection_str =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| ":memory:".to_string());

    tracing::info!("Opening database at \"{}\"...", &db_connection_str);
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await?;

    init_db(&pool).await?;

    tracing::info!("Database initialized.");

    let (tx, _rx) = broadcast::channel(64);

    let state = Arc::new(AppState { db: pool, tx });

    let app = Router::new()
        .route("/", get(|| async { "YES SIR OORAH" }))
        .route("/listen", get(api::ws_handler))
        .route("/topics", get(api::get_topics))
        .route("/topics", post(api::post_topics))
        .route("/topics", delete(api::delete_topics))
        .route("/users", get(api::get_users))
        .route("/users", post(api::post_users))
        .route("/users/me", get(api::get_users_me))
        .route("/users/me", delete(api::delete_users_me))
        .route("/notify", post(api::post_notify))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    tracing::info!("Server is up.");

    axum::Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await?;
    Ok(())
}

async fn init_db(pool: &SqlitePool) -> anyhow::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS topics (
        id TEXT PRIMARY KEY,
        description TEXT
    )",
    )
    .execute(pool)
    .await?;

    // change primary key if incorrect

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
        auth_provider TEXT NOT NULL,
        permission_level INTEGER NOT NULL,
        auth_id TEXT NOT NULL PRIMARY KEY,
        auth_username TEXT
    );
    ",
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[derive(Debug, Clone)]
pub struct AppState {
    db: SqlitePool,
    tx: Sender<String>,
}
