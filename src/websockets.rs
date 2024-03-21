use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use futures_util::StreamExt;
use tracing::instrument;
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

#[instrument(name = "websocket_connection")]
async fn handle_socket(mut socket: WebSocket, who: SocketAddr, state: Arc<AppState>) {
    let mut recv = state.tx.subscribe();

    // TODO: find a cleaner way to implement this

    /*
        TODO: investigate high memory usage

        After a lot of websocket connections (~20K) get opened, the memory usage increases
        to about ~200MiB (as it should), but when those connections close, the OS doesn't
        reclaim that memory.

        When the websockets get reopened the memory usage stays the same though,
        so i suspect tungstenite is doing something funky with memory pools (which is fine).

        If that's the case, then tungstenite's memory pool doesn't let the OS reclaim any memory,
        no matter how much time has passed from the last time it used it.
    */
    loop {
        select! {
            Ok(msg) = recv.recv() => {
                /*
                    TODO: look for a way to remove this clone

                    Maybe the tungstenite-rs will support Bytes in the future (e.g. Arc<[u8]>)
                    Or just Arc<str> or something, but this clone alone takes half of the CPU time
                    and i suspect the rest of the CPU time is taken by the internal buffer cloning

                    Either way, the memory clones are what is taking the most CPU time,
                    and removing them will turn down the CPU time spent per websocket connection,
                    allowing for more and more connections at a time.

                    In any case, these are all CPU *spikes* that happen on every broadcast message,
                    and thus even if we reach 100% CPU time, it'll only increase the latency slightly,
                    and it won't completely tank performance.

                    This happens at about 50K open websockets on a Ryzen 5 3600, so pretty decent.
                */
                if let Err(_) = socket.send(Message::Text(msg.to_string())).await {
                    break;
                }
            }

            msg = socket.next() => {
                if let Some(Ok(_)) = msg {
                    // we dont care (for now)
                } else {
                    // websocket closed
                    break;
                }
            }
        }
    }
}
