use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;

use crate::{types::User, AppState};

pub async fn get_users(
    State(state): State<Arc<AppState>>,
    Query(u): Query<User>,
) -> Result<Json<User>, StatusCode> {
    // TODO: AUTHENTICATION
    if let Some(s) = u.auth_id {
        let user_or_err = sqlx::query_as::<_, User>("SELECT permission_level, auth_provider, auth_id, auth_username FROM users WHERE auth_id = $1")
        .bind(s)
        .fetch_one(&state.db)
        .await;

        match user_or_err {
            Ok(user) => Ok(Json(user)),
            Err(sqlx::Error::RowNotFound) => Err(StatusCode::NOT_FOUND),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    } else if let Some(s) = u.auth_username {
        let user_or_err = sqlx::query_as::<_, User>("SELECT permission_level, auth_provider, auth_id, auth_username FROM users WHERE auth_username = $1")
        .bind(s)
        .fetch_one(&state.db)
        .await;

        match user_or_err {
            Ok(user) => Ok(Json(user)),
            Err(sqlx::Error::RowNotFound) => Err(StatusCode::NOT_FOUND),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

pub async fn post_users(
    State(state): State<Arc<AppState>>,
    Json(user): Json<User>,
) -> Result<Json<User>, StatusCode> {
    // TODO: AUTH
    // TODO: api call
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get_users_me(State(state): State<Arc<AppState>>) -> Result<Json<User>, StatusCode> {
    // TODO: AUTH
    // TODO: api call
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn delete_users_me(State(state): State<Arc<AppState>>) -> Result<(), StatusCode> {
    // TODO: AUTH
    // TODO: api call
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}
