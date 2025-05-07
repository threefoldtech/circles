use crate::models::user::User;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Represents a video conference meeting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meeting {
    /// Unique identifier for the meeting
    pub id: Uuid,
    /// Meeting name
    pub name: String,
    /// Meeting ID that users can use to join (e.g., 123-456-789)
    pub meeting_id: String,
    /// Optional password for the meeting
    pub password: Option<String>,
    /// Host user ID
    pub host_id: Uuid,
    /// Co-host user IDs
    pub co_host_ids: Vec<Uuid>,
    /// Participants in the meeting
    pub participants: Vec<Participant>,
    /// Chat messages in the meeting
    pub chat_messages: Vec<ChatMessage>,
    /// Meeting settings
    pub settings: MeetingSettings,
    /// Meeting start time
    pub start_time: DateTime<Utc>,
    /// Meeting duration in minutes (0 for unlimited)
    pub duration: i64,
    /// Whether the meeting is locked (no new participants can join)
    pub is_locked: bool,
    /// Whether the waiting room is enabled
    pub waiting_room_enabled: bool,
    /// Users in the waiting room
    pub waiting_room: Vec<Participant>,
    /// Whether the meeting has ended
    pub has_ended: bool,
}

/// Represents a participant in a meeting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    /// User ID
    pub id: Uuid,
    /// User name
    pub name: String,
    /// Whether the participant's camera is on
    pub camera_on: bool,
    /// Whether the participant's microphone is on
    pub mic_on: bool,
    /// Whether the participant is sharing their screen
    pub is_sharing_screen: bool,
    /// Whether the participant has raised their hand
    pub hand_raised: bool,
    /// Whether the participant is the active speaker
    pub is_active_speaker: bool,
    /// Participant's role in the meeting
    pub role: ParticipantRole,
    /// Time when the participant joined the meeting
    pub joined_at: DateTime<Utc>,
}

/// Represents a participant's role in a meeting
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParticipantRole {
    /// Host of the meeting
    Host,
    /// Co-host of the meeting
    CoHost,
    /// Regular participant
    Participant,
}

/// Represents a chat message in a meeting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    /// Message ID
    pub id: Uuid,
    /// Sender ID
    pub sender_id: Uuid,
    /// Sender name
    pub sender_name: String,
    /// Message content
    pub content: String,
    /// Time when the message was sent
    pub sent_at: DateTime<Utc>,
    /// Whether the message is private
    pub is_private: bool,
    /// Recipient ID if the message is private
    pub recipient_id: Option<Uuid>,
    /// Attached file if any
    pub attachment: Option<Attachment>,
}

/// Represents an attachment in a chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    /// File name
    pub name: String,
    /// File type
    pub file_type: String,
    /// File size in bytes
    pub size: u64,
    /// File data (base64 encoded)
    pub data: String,
}

/// Represents meeting settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeetingSettings {
    /// Whether participants can unmute themselves
    pub participants_can_unmute: bool,
    /// Whether participants can turn on their camera
    pub participants_can_turn_on_camera: bool,
    /// Whether participants can share their screen
    pub participants_can_share_screen: bool,
    /// Whether multiple participants can share their screen simultaneously
    pub allow_multiple_screen_shares: bool,
    /// Whether participants can send chat messages
    pub participants_can_chat: bool,
    /// Whether participants can send private messages
    pub allow_private_chat: bool,
    /// Whether participants can raise their hand
    pub allow_raise_hand: bool,
    /// Whether participants can use reactions
    pub allow_reactions: bool,
    /// Maximum number of participants allowed (0 for unlimited)
    pub max_participants: usize,
    /// Video quality setting
    pub video_quality: VideoQuality,
}

/// Represents video quality settings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VideoQuality {
    /// Low quality (360p)
    Low,
    /// Medium quality (720p)
    Medium,
    /// High quality (1080p)
    High,
}

/// Represents a reaction in a meeting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reaction {
    /// User ID who sent the reaction
    pub user_id: Uuid,
    /// Reaction emoji
    pub emoji: String,
    /// Time when the reaction was sent
    pub sent_at: DateTime<Utc>,
}

/// Represents the state of the video conference feature
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VideoConferenceState {
    /// Current meeting if any
    pub current_meeting: Option<Meeting>,
    /// Whether the user is in a meeting
    pub in_meeting: bool,
    /// Whether the join meeting dialog is open
    pub join_dialog_open: bool,
    /// Whether the create meeting dialog is open
    pub create_dialog_open: bool,
    /// Whether the leave meeting confirmation dialog is open
    pub leave_confirmation_open: bool,
    /// Whether the end meeting confirmation dialog is open
    pub end_confirmation_open: bool,
    /// Whether the side panel is open
    pub side_panel_open: bool,
    /// Current side panel tab
    pub side_panel_tab: SidePanelTab,
    /// Whether the user's camera is on
    pub camera_on: bool,
    /// Whether the user's microphone is on
    pub mic_on: bool,
    /// Whether the user is sharing their screen
    pub is_sharing_screen: bool,
    /// Whether the user has raised their hand
    pub hand_raised: bool,
    /// Active reactions with their expiration time
    pub active_reactions: HashMap<Uuid, (Reaction, DateTime<Utc>)>,
    /// Meeting ID input for joining a meeting
    pub meeting_id_input: String,
    /// Password input for joining a meeting
    pub password_input: String,
    /// Meeting name input for creating a meeting
    pub meeting_name_input: String,
    /// Whether to require a password for the meeting
    pub require_password: bool,
    /// Password input for creating a meeting
    pub create_password_input: String,
    /// Chat message input
    pub chat_input: String,
    /// Whether the settings dialog is open
    pub settings_dialog_open: bool,
    /// Whether the user has granted camera permission
    pub camera_permission_granted: Option<bool>,
    /// Whether the user has granted microphone permission
    pub mic_permission_granted: Option<bool>,
    /// Whether the user has granted screen sharing permission
    pub screen_share_permission_granted: Option<bool>,
}

/// Represents the side panel tab in the video conference
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SidePanelTab {
    /// Participants tab
    Participants,
    /// Chat tab
    Chat,
}

impl Default for SidePanelTab {
    fn default() -> Self {
        Self::Participants
    }
}

impl Meeting {
    /// Create a new meeting
    pub fn new(name: String, host: &User) -> Self {
        let id = Uuid::new_v4();
        let meeting_id = generate_meeting_id();

        Self {
            id,
            name,
            meeting_id,
            password: None,
            host_id: host.id,
            co_host_ids: Vec::new(),
            participants: vec![Participant {
                id: host.id,
                name: host.name.clone(),
                camera_on: true,
                mic_on: true,
                is_sharing_screen: false,
                hand_raised: false,
                is_active_speaker: false,
                role: ParticipantRole::Host,
                joined_at: Utc::now(),
            }],
            chat_messages: Vec::new(),
            settings: MeetingSettings::default(),
            start_time: Utc::now(),
            duration: 0, // Unlimited by default
            is_locked: false,
            waiting_room_enabled: false,
            waiting_room: Vec::new(),
            has_ended: false,
        }
    }

    /// Add a participant to the meeting
    pub fn add_participant(&mut self, participant: Participant) {
        // Check if the participant is already in the meeting
        if !self.participants.iter().any(|p| p.id == participant.id) {
            self.participants.push(participant);
        }
    }

    /// Remove a participant from the meeting
    pub fn remove_participant(&mut self, participant_id: Uuid) {
        self.participants.retain(|p| p.id != participant_id);
    }

    /// Add a chat message to the meeting
    pub fn add_chat_message(&mut self, message: ChatMessage) {
        self.chat_messages.push(message);
    }

    /// Get the meeting duration as a formatted string
    pub fn get_duration_string(&self) -> String {
        let now = Utc::now();
        let duration = now.signed_duration_since(self.start_time);

        let hours = duration.num_hours();
        let minutes = duration.num_minutes() % 60;
        let seconds = duration.num_seconds() % 60;

        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }

    /// Check if the meeting has a time limit and if it has been reached
    pub fn is_time_limit_reached(&self) -> bool {
        if self.duration == 0 {
            return false; // No time limit
        }

        let now = Utc::now();
        let duration = now.signed_duration_since(self.start_time);

        duration.num_minutes() >= self.duration
    }
}

impl Default for MeetingSettings {
    fn default() -> Self {
        Self {
            participants_can_unmute: true,
            participants_can_turn_on_camera: true,
            participants_can_share_screen: true,
            allow_multiple_screen_shares: false,
            participants_can_chat: true,
            allow_private_chat: true,
            allow_raise_hand: true,
            allow_reactions: true,
            max_participants: 100,
            video_quality: VideoQuality::Medium,
        }
    }
}

/// Generate a random meeting ID in the format XXX-XXX-XXX
fn generate_meeting_id() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let part1: u16 = rng.gen_range(100..1000);
    let part2: u16 = rng.gen_range(100..1000);
    let part3: u16 = rng.gen_range(100..1000);

    format!("{}-{}-{}", part1, part2, part3)
}
