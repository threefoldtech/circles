use crate::models::circle::{Circle, CircleType};
use crate::models::user::User;
use crate::ui::app_layout;
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
        let user = Some(User {
            id: Uuid::new_v4(),
            name: "Default User".to_string(), // More polished default name
            email: "user@circleapp.com".to_string(), // Branded email
            created_at: chrono::Utc::now(),
        });

        // Curated demo circles with meaningful names
        let circles = vec![
            Circle::new(
                "My Personal Space".to_string(),
                CircleType::Personal,
                user.as_ref().unwrap().id, // Safe unwrap since user is Some
            ),
            Circle::new(
                "Team Collaboration".to_string(),
                CircleType::Team,
                user.as_ref().unwrap().id,
            ),
            Circle::new(
                "Private Thoughts".to_string(),
                CircleType::Private,
                user.as_ref().unwrap().id,
            ),
        ];

        // Elegantly select the first circle as active
        let active_circle_id = circles.first().map(|c| c.id);

        Self {
            user,
            circles,
            active_circle_id,
            active_feature: ActiveFeature::default(),
            search_query: String::new(),
        }
    }

    /// Retrieve the currently active circle in a refined manner
    pub fn active_circle(&self) -> Option<&Circle> {
        self.active_circle_id
            .and_then(|id| self.circles.iter().find(|c| c.id == id))
    }

    /// Gracefully set the active circle
    pub fn set_active_circle(&mut self, circle_id: Uuid) {
        self.active_circle_id = Some(circle_id);
    }

    /// Smoothly transition to a new active feature
    pub fn set_active_feature(&mut self, feature: ActiveFeature) {
        self.active_feature = feature;
    }
}

impl eframe::App for CircleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        app_layout::render(self, ctx);
    }
}
