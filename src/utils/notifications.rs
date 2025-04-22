use notify_rust::Notification;
use std::error::Error;

pub enum NotificationType {
    Success,
    Error,
    Info,
    Warning,
}

pub struct NotificationConfig {
    pub title: String,
    pub body: String,
    pub notification_type: NotificationType,
    pub timeout: i32, // in milliseconds
}

impl NotificationConfig {
    pub fn new(title: impl Into<String>, body: impl Into<String>, notification_type: NotificationType) -> Self {
        Self {
            title: title.into(),
            body: body.into(),
            notification_type,
            timeout: 5000, // Default 5 seconds
        }
    }
}

pub fn send_desktop_notification(config: &NotificationConfig) -> Result<(), Box<dyn Error>> {
    let icon = match config.notification_type {
        NotificationType::Success => "dialog-ok",
        NotificationType::Error => "dialog-error",
        NotificationType::Info => "dialog-information",
        NotificationType::Warning => "dialog-warning",
    };

    Notification::new()
        .summary(&config.title)
        .body(&config.body)
        .icon(icon)
        .timeout(config.timeout) // milliseconds
        .show()?;

    Ok(())
}