use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PermissionLevel {
    /// No permissions
    None = 0,
    /// Able to send broadcasts
    SendMessages = 1,
    /// Lets you add/remove topics and manage the permission level of other users
    Manage = 2,
    /// Lets you manage the permission level of everyone (including other owners)
    Owner = 3,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AuthProvider {
    Reddit,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub auth_provider: AuthProvider,
    pub permission_level: PermissionLevel,
    pub auth_id: String,
    pub auth_username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Topic {
    pub id: String,
    pub description: String,
}

#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum WebSocketEvent {
    NewNotification = 1,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPost {
    pub topic: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    /// Event of the notification
    pub e: WebSocketEvent,
    /// Topic of the notification
    pub t: String,
    /// Content of the notification
    pub c: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyResponse {
    pub count: usize,
}
