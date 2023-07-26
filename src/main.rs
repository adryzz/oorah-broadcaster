mod api;
mod types;

use std::time::Duration;

use axum::{
    routing::{delete, get, post},
    Router,
};
use sqlx::sqlite::SqlitePoolOptions;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    match run().await {
        Ok(_) => tracing::info!("Program exited successfully."),
        Err(e) => tracing::error!("Error: {}", e),
    }
}

async fn run() -> anyhow::Result<()> {
    let db_connection_str =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://db.sqlite".to_string());

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await?;

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
        .with_state(pool);

    axum::Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
