use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationPriority {
    Low,
    Normal,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppNotification {
    pub id: String,
    pub title: String,
    pub message: String,
    pub created_at: SystemTime,
    pub read: bool,
    pub priority: NotificationPriority,
    pub action_url: Option<String>,
}

impl AppNotification {
    pub fn new(
        title: impl Into<String>,
        message: impl Into<String>,
        priority: NotificationPriority,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: title.into(),
            message: message.into(),
            created_at: SystemTime::now(),
            read: false,
            priority,
            action_url: None,
        }
    }
}
