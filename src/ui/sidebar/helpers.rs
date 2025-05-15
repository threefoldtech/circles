use crate::app::CircleApp;
use crate::models::dummy_data::{Conversation, Message};
use crate::utils::config::Theme;
use egui::{Button, RichText, Stroke, Vec2};
use uuid::Uuid;

// Helper function to add a message to CirclesBot
pub fn add_log_to_circles_bot(app: &mut CircleApp, message: String) {
    let circles_bot = app.circles.iter().find(|c| c.name == "CirclesBot");
    if let Some(bot) = circles_bot {
        if let Some(feature_data) = app.circle_feature_data.get_mut(&bot.id) {
            let conversations = &mut feature_data.chat_data.conversations;
            if conversations.is_empty() {
                conversations.push(Conversation {
                    id: Uuid::new_v4(),
                    name: "Circles Bot".to_string(),
                    participants: vec!["Circles Bot".to_string(), "Me".to_string()],
                    messages: Vec::new(),
                });
            }
            conversations[0].messages.push(Message {
                id: Uuid::new_v4(),
                sender: "CirclesBot".to_string(),
                content: message,
                timestamp: chrono::Utc::now(),
                read: false,
            });
        }
    }
}

// Helper function for creating add button
pub fn create_add_button(theme: &Theme) -> Button {
    Button::new(RichText::new("➕").size(16.0).color(theme.white))
        .min_size(Vec2::new(32.0, 32.0))
        .corner_radius(8.0)
        .fill(theme.accent)
        .stroke(Stroke::NONE)
}

// Helper function for creating settings button
pub fn create_settings_button(theme: &Theme) -> Button {
    Button::new(RichText::new("⚙️ Settings").size(14.0).color(theme.text))
        .min_size(Vec2::new(200.0, 36.0))
        .corner_radius(8.0)
        .fill(theme.secondary_background)
        .stroke(Stroke::new(1.0, theme.border))
}

// Helper function for circle type name
pub fn circle_type_name(circle_type: crate::models::circle::CircleType) -> &'static str {
    match circle_type {
        crate::models::circle::CircleType::Personal => "Personal",
        crate::models::circle::CircleType::Team => "Team",
        crate::models::circle::CircleType::Private => "Private",
    }
}
