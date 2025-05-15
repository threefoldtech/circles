use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::features::Features;

/// Represents a Circle in the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Circle {
    /// Unique identifier for the circle
    pub id: Uuid,
    /// Name of the circle (e.g., "project.sunflower")
    pub name: String,
    /// Type of the circle
    pub circle_type: CircleType,
    /// Whether this is a system circle (e.g., WelcomeBot, CirclesBot)
    pub is_system_circle: bool,
    /// List of members in the circle
    pub members: Vec<Member>,
    /// Features available in the circle
    pub features: Features,
    /// Circle settings
    pub settings: CircleSettings,
    /// Additional metadata for the circle
    pub metadata: HashMap<String, String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

/// Types of circles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CircleType {
    /// Personal circle (e.g., between two individuals)
    Personal,
    /// Team or organizational circle
    Team,
    /// Private circle (for individual use)
    Private,
}

/// Represents a member of a circle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    /// User ID of the member
    pub user_id: Uuid,
    /// Name of the member
    pub name: String,
    /// Role of the member in the circle
    pub role: Role,
    /// When the member joined the circle
    pub joined_at: DateTime<Utc>,
}

/// Roles that a member can have in a circle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    /// Read-only access
    Read,
    /// Can read and modify content
    Write,
    /// Full control over content and membership
    Administrator,
    /// Similar to administrator but focused on task coordination
    Coordinator,
}

/// Settings for a circle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircleSettings {
    /// Visibility of the circle
    pub visibility: Visibility,
    /// Policy for joining the circle
    pub join_policy: JoinPolicy,
    /// Notification settings for the circle
    pub notification_settings: NotificationSettings,
}

/// Visibility options for a circle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Visibility {
    /// Visible to everyone
    Public,
    /// Visible only to members
    Private,
    /// Not visible in listings
    Secret,
}

/// Join policies for a circle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JoinPolicy {
    /// Anyone can join
    Open,
    /// Requires approval from an administrator
    ApprovalRequired,
    /// Only by invitation
    InviteOnly,
}

/// Notification settings for a circle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSettings {
    /// Whether to send email notifications
    pub email_notifications: bool,
    /// Whether to send push notifications
    pub push_notifications: bool,
    /// Whether to send in-app notifications
    pub in_app_notifications: bool,
}

impl Circle {
    /// Create a new circle with default settings
    pub fn new(name: String, circle_type: CircleType, creator_id: Uuid) -> Self {
        let now = Utc::now();
        let creator = Member {
            user_id: creator_id,
            name: "Creator".to_string(), // This would be replaced with actual user name
            role: Role::Administrator,
            joined_at: now,
        };

        Self {
            id: Uuid::new_v4(),
            name,
            circle_type,
            is_system_circle: false,
            members: vec![creator],
            features: Features::default(),
            settings: CircleSettings {
                visibility: Visibility::Private,
                join_policy: JoinPolicy::InviteOnly,
                notification_settings: NotificationSettings {
                    email_notifications: true,
                    push_notifications: true,
                    in_app_notifications: true,
                },
            },
            metadata: HashMap::new(),
            created_at: now,
            updated_at: now,
        }
    }
}
