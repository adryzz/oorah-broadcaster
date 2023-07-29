use axum::{extract::State, http::StatusCode, Json};
use std::sync::Arc;

use crate::{types::Topic, utils::is_valid_topic_id, AppState};

#[axum_macros::debug_handler]
pub async fn get_topics(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Topic>>, StatusCode> {
    let topics_or_err = sqlx::query_as::<_, Topic>("SELECT id, description FROM topics")
        .fetch_all(&state.db)
        .await;

    match topics_or_err {
        Ok(topics) => Ok(Json(topics)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[axum_macros::debug_handler]
pub async fn post_topics(
    State(state): State<Arc<AppState>>,
    Json(topic): Json<Topic>,
) -> Result<Json<Vec<Topic>>, StatusCode> {
    // TODO: ADD AUTH

    if let None = topic.description {
        return Err(StatusCode::BAD_REQUEST);
    }

    if !is_valid_topic_id(&topic.id) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let a = sqlx::query("INSERT INTO topics (id, description) VALUES ($1, $2)")
        .bind(topic.id)
        .bind(topic.description)
        .execute(&state.db)
        .await;

    match a {
        Err(sqlx::Error::Database(_)) => return Err(StatusCode::BAD_REQUEST),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        _ => {}
    }

    let topics_or_err = sqlx::query_as::<_, Topic>("SELECT id, description FROM topics")
        .fetch_all(&state.db)
        .await;

    match topics_or_err {
        Ok(topics) => Ok(Json(topics)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[axum_macros::debug_handler]
pub async fn delete_topics(
    State(state): State<Arc<AppState>>,
    Json(topic): Json<Topic>,
) -> Result<Json<Vec<Topic>>, StatusCode> {
    // TODO: ADD AUTHENTICATION
    let a = sqlx::query("DELETE FROM topics WHERE id = $1")
        .bind(topic.id)
        .execute(&state.db)
        .await;

    match a {
        Err(sqlx::Error::Database(_)) => return Err(StatusCode::BAD_REQUEST),
        Err(sqlx::Error::RowNotFound) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        _ => {}
    }

    let topics_or_err = sqlx::query_as::<_, Topic>("SELECT id, description FROM topics")
        .fetch_all(&state.db)
        .await;

    match topics_or_err {
        Ok(topics) => Ok(Json(topics)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
