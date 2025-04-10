use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a user in the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unique identifier for the user
    pub id: Uuid,
    /// Name of the user (e.g., "user.john")
    pub name: String,
    /// Email address of the user
    pub email: String,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

/// User preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    /// Theme preference
    pub theme: Theme,
    /// Language preference
    pub language: String,
    /// Timezone preference
    pub timezone: String,
    /// Notification preferences
    pub notification_preferences: NotificationPreferences,
}

/// Theme options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Theme {
    /// Light theme
    Light,
    /// Dark theme
    Dark,
    /// System theme
    System,
}

/// Notification preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPreferences {
    /// Whether to send email notifications
    pub email_notifications: bool,
    /// Whether to send push notifications
    pub push_notifications: bool,
    /// Whether to send in-app notifications
    pub in_app_notifications: bool,
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            theme: Theme::System,
            language: "en".to_string(),
            timezone: "UTC".to_string(),
            notification_preferences: NotificationPreferences {
                email_notifications: true,
                push_notifications: true,
                in_app_notifications: true,
            },
        }
    }
}
