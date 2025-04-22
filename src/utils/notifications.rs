use notify_rust::{Notification, Timeout, Urgency};
use std::error::Error;

#[allow(dead_code)]
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
    pub fn new(
        title: impl Into<String>,
        body: impl Into<String>,
        notification_type: NotificationType,
    ) -> Self {
        Self {
            title: title.into(),
            body: body.into(),
            notification_type,
            timeout: 10000, // Default 5 seconds
        }
    }

    pub fn with_timeout(mut self, timeout_ms: i32) -> Self {
        self.timeout = timeout_ms;
        self
    }
}

pub fn send_desktop_notification(config: &NotificationConfig) -> Result<(), Box<dyn Error>> {
    let (icon, urgency) = match config.notification_type {
        NotificationType::Success => ("dialog-ok", Urgency::Low),
        NotificationType::Error => ("dialog-error", Urgency::Critical),
        NotificationType::Info => ("dialog-information", Urgency::Normal),
        NotificationType::Warning => ("dialog-warning", Urgency::Normal),
    };

    let adjusted_timeout = 30000; // 30 seconds for testing

    let handle = Notification::new()
        .appname("Circle")
        .summary(&config.title)
        .body(&config.body)
        .icon(icon)
        .timeout(Timeout::Milliseconds(adjusted_timeout as u32)) // ✅ fix here
        .urgency(urgency)
        .show()?;

    println!(
        "Notification sent with ID: {} (timeout: {}ms)",
        handle.id(),
        adjusted_timeout
    );

    Ok(())
}
