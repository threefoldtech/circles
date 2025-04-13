use crate::models::circle::Circle;
use crate::models::dummy_data::{self, CircleFeatureData};
use crate::models::user::User;
use crate::ui::app_layout;
use std::collections::HashMap;
use uuid::Uuid;

/// Main application state with an elegant and modern design approach
#[derive(Debug)]
pub struct CircleApp {
    /// Current authenticated user
    pub user: Option<User>,
    /// Collection of user circles
    pub circles: Vec<Circle>,
    /// ID of the currently selected circle
    pub active_circle_id: Option<Uuid>,
    /// Currently selected feature in the UI
    pub active_feature: ActiveFeature,
    /// Search query for filtering circles
    pub search_query: String,
    /// Feature data for each circle
    pub circle_feature_data: HashMap<Uuid, CircleFeatureData>,
    /// Currently active feature data
    pub active_feature_data: Option<CircleFeatureData>,
    /// Circle creation dialog state
    pub circle_dialog_state: crate::ui::components::circle_dialog::CircleDialogState,
    /// Flag to track if this is the first time the user is opening the app
    pub is_first_time: bool,
}

/// Enum representing available features in the application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveFeature {
    Mail,
    Calendar,
    Chat,
    Documents,
    AITools,
    VideoConference,
    Settings,
    /// Special welcome screen for first-time users
    Welcome,
    /// Special feature for the Circles Bot Channel
    BotChannel,
}

impl Default for ActiveFeature {
    fn default() -> Self {
        Self::Mail // Default to Mail for a welcoming entry point
    }
}

impl CircleApp {
    /// Initialize a new instance of the application with elegant defaults
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {
        // Initialize with a sophisticated default user
        let user_id = Uuid::new_v4();
        let user = Some(User {
            id: user_id,
            name: "Default User".to_string(), // More polished default name
            email: "user@circleapp.com".to_string(), // Branded email
            created_at: chrono::Utc::now(),
        });

        // Start with an empty circles list
        let mut circles = Vec::new();

        // Create a HashMap to store feature data for each circle
        let mut circle_feature_data = HashMap::new();

        // Create default circles for new users
        let welcome_circle = Self::create_welcome_circle(user_id);
        let bot_circle = Self::create_bot_circle(user_id);

        // Generate feature data for default circles
        let welcome_feature_data = dummy_data::generate_dummy_data_for_circle(
            welcome_circle.id,
            &welcome_circle.name,
            welcome_circle.circle_type,
        );

        let bot_feature_data = dummy_data::generate_dummy_data_for_circle(
            bot_circle.id,
            &bot_circle.name,
            bot_circle.circle_type,
        );

        // Add feature data to the map
        circle_feature_data.insert(welcome_circle.id, welcome_feature_data);
        circle_feature_data.insert(bot_circle.id, bot_feature_data);

        // Add default circles to the list
        circles.push(welcome_circle);
        circles.push(bot_circle);

        // Set active circle to the welcome circle
        let active_circle_id = Some(circles[0].id);
        let active_feature_data = circle_feature_data.get(&circles[0].id).cloned();

        // For first-time users, we'll show a welcome screen instead of the default mail feature
        let is_first_time = true; // Always true for new instances

        Self {
            user,
            circles,
            active_circle_id,
            // For first-time users, we'll use a special Welcome feature instead of the default Mail
            active_feature: if is_first_time {
                ActiveFeature::Welcome
            } else {
                ActiveFeature::default()
            },
            search_query: String::new(),
            circle_feature_data,
            active_feature_data,
            circle_dialog_state: crate::ui::components::circle_dialog::CircleDialogState::new(),
            is_first_time,
        }
    }

    /// Create a welcome circle with instructions for new users
    fn create_welcome_circle(creator_id: Uuid) -> Circle {
        use crate::models::circle::{
            Circle, CircleType, JoinPolicy, NotificationSettings, Visibility,
        };

        let mut circle = Circle::new("WelcomeBot".to_string(), CircleType::Private, creator_id);

        // We'll customize the welcome circle data in the dummy_data generation
        // The document will be added there

        circle.settings.visibility = Visibility::Private;
        circle.settings.join_policy = JoinPolicy::InviteOnly;
        circle.settings.notification_settings = NotificationSettings {
            email_notifications: true,
            push_notifications: true,
            in_app_notifications: true,
        };

        circle
    }

    /// Create a bot circle for system updates
    fn create_bot_circle(creator_id: Uuid) -> Circle {
        use crate::models::circle::{
            Circle, CircleType, JoinPolicy, NotificationSettings, Visibility,
        };

        let mut circle = Circle::new("CirclesBot".to_string(), CircleType::Private, creator_id);

        // We'll customize the bot circle data in the dummy_data generation
        // The chat messages will be added there

        circle.settings.visibility = Visibility::Private;
        circle.settings.join_policy = JoinPolicy::InviteOnly;
        circle.settings.notification_settings = NotificationSettings {
            email_notifications: true,
            push_notifications: true,
            in_app_notifications: true,
        };

        circle
    }

    /// Retrieve the currently active circle in a refined manner
    pub fn active_circle(&self) -> Option<&Circle> {
        self.active_circle_id
            .and_then(|id| self.circles.iter().find(|c| c.id == id))
    }

    /// Gracefully set the active circle and load its feature data
    pub fn set_active_circle(&mut self, circle_id: Uuid) {
        self.active_circle_id = Some(circle_id);

        // Load the feature data for the selected circle
        self.active_feature_data = self.circle_feature_data.get(&circle_id).cloned();

        // If the circle doesn't have feature data yet, generate it
        if self.active_feature_data.is_none() {
            if let Some(circle) = self.circles.iter().find(|c| c.id == circle_id) {
                let feature_data = dummy_data::generate_dummy_data_for_circle(
                    circle_id,
                    &circle.name,
                    circle.circle_type,
                );
                self.circle_feature_data
                    .insert(circle_id, feature_data.clone());
                self.active_feature_data = Some(feature_data);
            }
        }
    }

    /// Smoothly transition to a new active feature
    pub fn set_active_feature(&mut self, feature: ActiveFeature) {
        self.active_feature = feature;
    }

    /// Add a new circle to the application
    pub fn add_circle(&mut self, circle: Circle) {
        // Generate feature data for the new circle
        let feature_data =
            dummy_data::generate_dummy_data_for_circle(circle.id, &circle.name, circle.circle_type);

        // Add the feature data to the map
        self.circle_feature_data.insert(circle.id, feature_data);

        // Add the circle to the list
        self.circles.push(circle);

        // If this is the first circle, make it active
        if self.active_circle_id.is_none() && !self.circles.is_empty() {
            self.set_active_circle(self.circles[0].id);
        }
    }

    /// Open the circle creation dialog
    pub fn open_circle_dialog(&mut self) {
        self.circle_dialog_state.is_open = true;
    }
}

impl eframe::App for CircleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        app_layout::render(self, ctx);
    }
}
