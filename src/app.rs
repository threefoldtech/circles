use crate::models::circle::Circle;
use crate::models::dummy_data::{self, CircleFeatureData};
use crate::models::user::{User, UserPreferences};
use crate::ui::app_layout;
use crate::ui::components::calendar::event::Event;
use crate::ui::components::notifications::NotificationManager;
use crate::ui::footer::UserMenuState;
use crate::utils::config::Theme;
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
    /// ID of the currently selected mail folder
    pub active_mail_folder_id: Option<Uuid>,
    /// ID of the currently selected email for viewing in dialog
    pub selected_email_id: Option<Uuid>,
    /// Flag to track if the email dialog is open
    pub email_dialog_open: bool,
    /// Flag to track if the compose dialog is open
    pub compose_dialog_open: bool,
    /// Draft for the compose dialog
    pub compose_draft: Option<crate::models::features::ComposeDraft>,
    /// Notification manager
    pub notification_manager: NotificationManager,
    /// Flag to track if the app is currently refreshing
    pub is_refreshing: bool,
    /// Start time of the last refresh action
    pub refresh_start_time: std::time::Instant,
    /// Calendar events
    pub calendar_events: Vec<Event>,
    /// Delete confirmation dialog state
    pub delete_confirmation_state: crate::models::features::DeleteConfirmationState,
    /// Add member dialog state
    pub add_member_state: crate::models::features::AddMemberState,
    /// Circle actions state
    pub circle_actions: HashMap<Uuid, crate::models::features::CircleActionState>,
    /// User menu state
    pub user_menu_state: UserMenuState,
    /// Logout confirmation dialog state
    pub logout_confirmation_state: bool,
    /// Documents feature state
    pub documents_state: crate::models::features::documents::DocumentsState,
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
    /// App-wide settings screen
    AppSettings,
    /// Special welcome screen for first-time users
    Welcome,
    /// Special feature for the Circles Bot Channel
    BotChannel,
    /// Authentication screen for sign up and sign in
    Auth,
}

impl Default for ActiveFeature {
    fn default() -> Self {
        Self::Mail // Default to Mail for a welcoming entry point
    }
}

impl CircleApp {
    /// Initialize a new instance of the application with elegant defaults
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {
        // Check if credentials exist
        let user = if let Ok(credentials) = crate::ui::features::auth::load_credentials() {
            // Create user from credentials
            Some(User {
                id: Uuid::new_v4(),
                name: credentials.name,
                email: credentials.email,
                created_at: chrono::Utc::now(),
                preferences: UserPreferences::default(),
            })
        } else {
            None
        };

        // Get user ID for creating circles
        let user_id = user.as_ref().map(|u| u.id).unwrap_or_else(Uuid::new_v4);

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

        // Determine if this is the first time or if we need to show the auth screen
        let is_first_time = user.is_none();

        // Set active mail folder to inbox if available
        let active_mail_folder_id = active_feature_data.as_ref().and_then(|data| {
            data.mail_data
                .folders
                .iter()
                .find(|folder| folder.name == "Inbox")
                .map(|folder| folder.id)
        });

        Self {
            user,
            circles,
            active_circle_id,
            // For first-time users, we'll show the auth screen
            active_feature: if is_first_time {
                ActiveFeature::Auth
            } else {
                ActiveFeature::default()
            },
            search_query: String::new(),
            circle_feature_data,
            active_feature_data,
            circle_dialog_state: crate::ui::components::circle_dialog::CircleDialogState::new(),
            is_first_time,
            active_mail_folder_id,
            selected_email_id: None,
            email_dialog_open: false,
            compose_dialog_open: false,
            compose_draft: None,
            notification_manager: NotificationManager::new(50), // Keep last 50 notifications
            is_refreshing: false,
            calendar_events: Vec::new(),
            refresh_start_time: std::time::Instant::now(),
            delete_confirmation_state: crate::models::features::DeleteConfirmationState::default(),
            add_member_state: crate::models::features::AddMemberState::default(),
            circle_actions: HashMap::new(),
            user_menu_state: UserMenuState::default(),
            logout_confirmation_state: false,
            documents_state: crate::models::features::documents::DocumentsState::default(),
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

        circle.is_system_circle = true;
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

        circle.is_system_circle = true;
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

        // Set active mail folder to inbox if available
        self.active_mail_folder_id = self.active_feature_data.as_ref().and_then(|data| {
            data.mail_data
                .folders
                .iter()
                .find(|folder| folder.name == "Inbox")
                .map(|folder| folder.id)
        });
    }

    /// Smoothly transition to a new active feature
    pub fn set_active_feature(&mut self, feature: ActiveFeature) {
        self.active_feature = feature;
    }

    /// Get the currently active feature
    pub fn get_active_feature(&mut self) -> ActiveFeature {
        return self.active_feature;
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

    pub fn apply_theme(&mut self, ctx: &egui::Context) {
        // Force a UI refresh to apply the new theme
        self.is_refreshing = true;

        // Use the context-aware theme method to properly detect system theme
        let theme = self.get_current_theme_with_context(ctx);
        ctx.set_visuals(theme.to_visuals());
    }

    pub fn save_user_preferences(&mut self, ctx: &egui::Context) {
        if let Some(_) = &self.user {
            // Here you would typically save to disk or database
            // For now, we'll just ensure the theme is applied
            self.apply_theme(ctx);
        }
    }

    /// Get the current theme of the application
    pub fn get_current_theme(&self) -> Theme {
        if let Some(user) = &self.user {
            Theme::from_mode(&user.preferences.theme, None)
        } else {
            Theme::light() // Default to light theme when no user is logged in
        }
    }

    /// Get the current theme of the application with context for system theme detection
    pub fn get_current_theme_with_context(&self, ctx: &egui::Context) -> Theme {
        if let Some(user) = &self.user {
            Theme::from_mode(&user.preferences.theme, Some(ctx))
        } else {
            Theme::light() // Default to light theme when no user is logged in
        }
    }
}

impl eframe::App for CircleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        app_layout::render(self, ctx);
    }
}
