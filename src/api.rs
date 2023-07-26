use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use std::net::SocketAddr;

//allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;

pub async fn get_topics() -> impl IntoResponse {}

pub async fn post_topics() -> impl IntoResponse {}

pub async fn delete_topics() -> impl IntoResponse {}

pub async fn get_users() -> impl IntoResponse {}

pub async fn post_users() -> impl IntoResponse {}

pub async fn get_users_me() -> impl IntoResponse {}

pub async fn delete_users_me() -> impl IntoResponse {}

pub async fn post_notify() -> impl IntoResponse {}

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
