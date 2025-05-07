use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ui::components::mail::compose::Attachment;

// Export the modules
pub mod add_member;
pub mod documents;
pub mod video_conf;

// We'll implement these modules later
// pub mod calendar;
// pub mod chat;
// pub mod mail;

/// Features available in a circle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Features {
    /// Mail feature
    pub mail: MailFeature,
    /// Calendar feature
    pub calendar: CalendarFeature,
    /// Chat feature
    pub chat: ChatFeature,
    /// Documents feature
    pub documents: DocumentFeature,
    /// AI tools feature
    pub ai_tools: AIToolsFeature,
    /// Video conferencing feature
    pub video_conf: VideoConfFeature,
}

/// Mail feature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailFeature {
    /// Whether the feature is enabled
    pub enabled: bool,
    /// Feature settings
    pub settings: MailSettings,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ComposeDraftMode {
    New,
    Reply,
    Forward,
}

// ComposeDraft
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ComposeDraft {
    pub to: String,
    pub subject: String,
    pub body: String,
    pub attachments: Vec<Attachment>,
    pub reply_to: Option<String>,         // Original sender email
    pub forward_from: Option<String>,     // Original sender for forwarded emails
    pub original_subject: Option<String>, // Original email subject
    pub original_date: Option<String>,    // Original email date
    pub original_content: Option<String>, // Original email content
    pub mode: ComposeDraftMode,           // New, Reply, or Forward
}

impl ComposeDraft {
    pub fn new() -> Self {
        Self {
            to: String::new(),
            subject: String::new(),
            body: String::new(),
            attachments: Vec::new(),
            reply_to: None,
            forward_from: None,
            original_subject: None,
            original_date: None,
            original_content: None,
            mode: ComposeDraftMode::New,
        }
    }
}

/// Calendar feature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarFeature {
    /// Whether the feature is enabled
    pub enabled: bool,
    /// Feature settings
    pub settings: CalendarSettings,
}

/// Chat feature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatFeature {
    /// Whether the feature is enabled
    pub enabled: bool,
    /// Feature settings
    pub settings: ChatSettings,
}

/// Documents feature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentFeature {
    /// Whether the feature is enabled
    pub enabled: bool,
    /// Feature settings
    pub settings: DocumentSettings,
}

/// AI tools feature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIToolsFeature {
    /// Whether the feature is enabled
    pub enabled: bool,
    /// Feature settings
    pub settings: AIToolsSettings,
}

/// Video conferencing feature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoConfFeature {
    /// Whether the feature is enabled
    pub enabled: bool,
    /// Feature settings
    pub settings: VideoConfSettings,
}

/// Mail settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailSettings {
    /// Whether to show mail notifications
    pub show_notifications: bool,
}

/// Calendar settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarSettings {
    /// Default view (day, week, month)
    pub default_view: String,
    /// Whether to show calendar notifications
    pub show_notifications: bool,
}

/// Chat settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSettings {
    /// Whether to show chat notifications
    pub show_notifications: bool,
    /// Whether to show read receipts
    pub show_read_receipts: bool,
}

/// Document settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentSettings {
    /// Default view (list, grid)
    pub default_view: String,
}

/// AI tools settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIToolsSettings {
    /// Whether to enable AI suggestions
    pub enable_suggestions: bool,
}

/// Video conferencing settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoConfSettings {
    /// Whether to automatically mute on join
    pub auto_mute: bool,
    /// Whether to automatically enable video on join
    pub auto_video: bool,
    /// Default video quality
    pub default_video_quality: video_conf::VideoQuality,
    /// Whether to enable waiting room by default
    pub enable_waiting_room: bool,
    /// Whether to allow participants to unmute themselves
    pub participants_can_unmute: bool,
    /// Whether to allow participants to share screen
    pub participants_can_share_screen: bool,
}

impl Default for Features {
    fn default() -> Self {
        Self {
            mail: MailFeature {
                enabled: true,
                settings: MailSettings {
                    show_notifications: true,
                },
            },
            calendar: CalendarFeature {
                enabled: true,
                settings: CalendarSettings {
                    default_view: "week".to_string(),
                    show_notifications: true,
                },
            },
            chat: ChatFeature {
                enabled: true,
                settings: ChatSettings {
                    show_notifications: true,
                    show_read_receipts: true,
                },
            },
            documents: DocumentFeature {
                enabled: true,
                settings: DocumentSettings {
                    default_view: "list".to_string(),
                },
            },
            ai_tools: AIToolsFeature {
                enabled: true,
                settings: AIToolsSettings {
                    enable_suggestions: true,
                },
            },
            video_conf: VideoConfFeature {
                enabled: true,
                settings: VideoConfSettings {
                    auto_mute: true,
                    auto_video: false,
                    default_video_quality: video_conf::VideoQuality::Medium,
                    enable_waiting_room: false,
                    participants_can_unmute: true,
                    participants_can_share_screen: true,
                },
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// State for circle actions (moved from global)
pub struct CircleActionState {
    pub is_favorite: bool,
    pub is_muted: bool,
}

impl Default for CircleActionState {
    fn default() -> Self {
        Self {
            is_favorite: false,
            is_muted: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// State for delete confirmation dialog (moved from global)
pub struct DeleteConfirmationState {
    pub open: bool,
    pub circle_id: Option<Uuid>,
    pub circle_name: String,
}

impl Default for DeleteConfirmationState {
    fn default() -> Self {
        Self {
            open: false,
            circle_id: None,
            circle_name: String::new(),
        }
    }
}

// Re-export the AddMemberState for easier access
pub use add_member::AddMemberState;
// Re-export the VideoConferenceState for easier access
pub use video_conf::VideoConferenceState;
