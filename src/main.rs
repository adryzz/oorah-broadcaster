mod notifications;
mod topics;
mod types;
mod users;
mod utils;
mod websockets;

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
    if std::env::var("TOKIO_CONSOLE").ok().is_some() {
        console_subscriber::init();
    } else {
        tracing_subscriber::fmt::init();
    }
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
        .route("/listen", get(websockets::ws_handler))
        .route("/topics", get(topics::get_topics))
        .route("/topics", post(topics::post_topics))
        .route("/topics", delete(topics::delete_topics))
        .route("/users", get(users::get_users))
        .route("/users", post(users::post_users))
        .route("/users/me", get(users::get_users_me))
        .route("/users/me", delete(users::delete_users_me))
        .route("/notify", post(notifications::post_notify))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = std::net::TcpListener::bind("0.0.0.0:3000")?;
    tracing::info!("Listening on {}...", listener.local_addr()?);

    axum::Server::from_tcp(listener)?
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
    tx: Sender<Arc<String>>,
}
