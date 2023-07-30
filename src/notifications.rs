use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};

use crate::{
    types::{Notification, NotificationPost, NotifyResponse, WebSocketEvent},
    AppState,
};

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
            let count_or_err = state.tx.send(Arc::new(serialized));

            match count_or_err {
                Ok(c) => Ok(Json(NotifyResponse { count: c })),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
