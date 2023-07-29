use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use futures_util::StreamExt;
use std::{net::SocketAddr, sync::Arc};
use tokio::select;

//allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;

use crate::AppState;

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
