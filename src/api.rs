use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::{error::DatabaseError, SqlitePool};
use std::{net::SocketAddr, sync::Arc};

//allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;

use crate::{
    types::{Notification, NotificationPost, NotifyResponse, Topic, WebSocketEvent},
    AppState,
};

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
    // TODO: ADD ID VALIDATION
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

pub async fn delete_topics(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Topic>>, StatusCode> {
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get_users(State(state): State<Arc<AppState>>) -> impl IntoResponse {}

pub async fn post_users(State(state): State<Arc<AppState>>) -> impl IntoResponse {}

pub async fn get_users_me(State(state): State<Arc<AppState>>) -> impl IntoResponse {}

pub async fn delete_users_me(State(state): State<Arc<AppState>>) -> impl IntoResponse {}

#[axum_macros::debug_handler]
pub async fn post_notify(
    State(state): State<Arc<AppState>>,
    Json(notification): Json<NotificationPost>,
) -> Result<Json<NotifyResponse>, StatusCode> {
    // TODO: AUTHENTICATION

    // Reject notifications longer than 1000 characters
    if notification.content.len() > 1000 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let notif = Notification {
        e: WebSocketEvent::NewNotification,
        t: notification.topic,
        c: notification.content
    };

    // pre-serialize ahead of time
    match serde_json::to_string(&notif) {
        Ok(serialized) => {
            let count_or_err = state.tx.send(serialized);

            match count_or_err {
                Ok(c) => Ok(Json(NotifyResponse { count: c })),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, addr, state))
}

async fn handle_socket(mut socket: WebSocket, who: SocketAddr, state: Arc<AppState>) {
    let mut recv = state.tx.subscribe();

    while let Ok(msg) = recv.recv().await {
        socket.send(Message::Text(msg)).await;
    }

    let _ = socket.close().await;
}
