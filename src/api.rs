use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Query, State,
    },
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use futures_util::StreamExt;
use std::{net::SocketAddr, sync::Arc};
use tokio::select;

//allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;

use crate::{
    types::{Notification, NotificationPost, NotifyResponse, Topic, User, WebSocketEvent},
    utils::is_valid_topic_id,
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
        c: notification.content,
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
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
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

    // TODO: find a cleaner way to implement this

    loop {
        select! {
            Ok(msg) = recv.recv() => {
                if let Err(_) = socket.send(Message::Text(msg)).await {
                    break;
                }
            }

            msg = socket.next() => {
                if let Some(Ok(_)) = msg {
                    // we dont care
                } else {
                    // websocket closed
                    break;
                }
            }
        }
    }
}
