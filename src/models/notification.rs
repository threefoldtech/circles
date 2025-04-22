use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

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

    pub fn with_action(mut self, action_url: impl Into<String>) -> Self {
        self.action_url = Some(action_url.into());
        self
    }

    pub fn age(&self) -> Duration {
        SystemTime::now()
            .duration_since(self.created_at)
            .unwrap_or(Duration::from_secs(0))
    }
}