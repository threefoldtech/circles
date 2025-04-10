use crate::models::circle::{Circle, CircleType};
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

        // Create a HashMap to store feature data for each circle
        let mut circle_feature_data = HashMap::new();

        // Generate dummy data for each circle
        for circle in &circles {
            let feature_data = dummy_data::generate_dummy_data_for_circle(
                circle.id,
                &circle.name,
                circle.circle_type,
            );
            circle_feature_data.insert(circle.id, feature_data);
        }

        // Elegantly select the first circle as active
        let active_circle_id = circles.first().map(|c| c.id);

        // Get the active feature data
        let active_feature_data =
            active_circle_id.and_then(|id| circle_feature_data.get(&id).cloned());

        Self {
            user,
            circles,
            active_circle_id,
            active_feature: ActiveFeature::default(),
            search_query: String::new(),
            circle_feature_data,
            active_feature_data,
        }
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
}

impl eframe::App for CircleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        app_layout::render(self, ctx);
    }
}
