use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::SqlitePool;
use std::net::SocketAddr;

//allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;

use crate::types::Topic;

#[axum_macros::debug_handler]
pub async fn get_topics(State(pool): State<SqlitePool>) -> Result<Json<Vec<Topic>>, StatusCode> {
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn post_topics(State(pool): State<SqlitePool>) -> Result<Json<Vec<Topic>>, StatusCode> {
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn delete_topics(State(pool): State<SqlitePool>) -> Result<Json<Vec<Topic>>, StatusCode> {
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get_users(State(pool): State<SqlitePool>) -> impl IntoResponse {}

pub async fn post_users(State(pool): State<SqlitePool>) -> impl IntoResponse {}

pub async fn get_users_me(State(pool): State<SqlitePool>) -> impl IntoResponse {}

pub async fn delete_users_me(State(pool): State<SqlitePool>) -> impl IntoResponse {}

pub async fn post_notify(State(pool): State<SqlitePool>) -> impl IntoResponse {}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

async fn handle_socket(mut socket: WebSocket, who: SocketAddr) {
    socket
        .send(Message::Text("YES SIR OORAH BUT WEBSOCKET".to_string()))
        .await
        .unwrap()
}
