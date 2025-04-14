use crate::app::CircleApp;
use crate::ui::app_layout::create_content_frame;
use crate::ui::components::bot_message::{BotMessage, BotMessageType};
use eframe::egui::{self, Color32, RichText, Rounding};

/// Render the bot channel UI
pub fn render_bot_channel(_app: &CircleApp, ui: &mut egui::Ui) {
    ui.add_space(16.0);
    
    create_content_frame().show(ui, |ui| {
        ui.vertical(|ui| {
            // Message history area
            let available_height = ui.available_height() - 80.0; // Reserve space for the bottom info
            
            // Create a scrollable area for messages
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .stick_to_bottom(true)
                .max_height(available_height)
                .show(ui, |ui| {
                    // Display bot messages
                    render_bot_messages(ui);
                });
            
            ui.add_space(16.0);
            
            // Bottom info area
            ui.separator();
            ui.add_space(8.0);
            
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 10.0;
                
                // Bot status indicator
                let bot_status_frame = egui::Frame::none()
                    .fill(Color32::from_rgb(76, 175, 80)) // Green for active
                    .rounding(Rounding::same(4.0))
                    .inner_margin(egui::Margin::same(4.0));
                
                bot_status_frame.show(ui, |ui| {
                    ui.label(
                        RichText::new("●")
                            .size(10.0)
                            .color(Color32::WHITE)
                    );
                });
                
                ui.label(
                    RichText::new("Circles Bot is active")
                        .size(14.0)
                        .color(Color32::from_rgb(100, 110, 120))
                );
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new("You can only receive messages in this channel")
                            .size(14.0)
                            .color(Color32::from_rgb(100, 110, 120))
                            .italics()
                    );
                });
            });
        });
    });
}

/// Render the bot messages
fn render_bot_messages(ui: &mut egui::Ui) {
    // Create example messages using our component
    let messages = get_example_messages();
    
    // Render each message
    for (i, message) in messages.iter().enumerate() {
        message.render(ui);
        
        // Add space between messages, but not after the last one
        if i < messages.len() - 1 {
            ui.add_space(16.0);
        }
    }
}

/// Get example messages for demonstration
fn get_example_messages() -> Vec<BotMessage> {
    vec![
        // Welcome message
        BotMessage::new(BotMessageType::Welcome),
        
        // Update message
        BotMessage::new(BotMessageType::Update {
            version: "1.0.1".to_string(),
            changes: vec![
                "Fixed a bug in the calendar feature".to_string(),
                "Improved performance for large documents".to_string(),
                "Added new emoji support in chat".to_string(),
            ],
        }),
        
        // Cat message - our new message type!
        BotMessage::new(BotMessageType::CatMessage {
            message: "Meow! I'm the new cat bot assistant. I'll help you navigate through the application with purr-fect precision!".to_string(),
            image_emoji: "🐱".to_string(),
        }),
        
        // Connection message
        BotMessage::new(BotMessageType::Connection {
            circle_name: "Project Alpha Team".to_string(),
        }),
        
        // Feature announcement
        BotMessage::new(BotMessageType::FeatureAnnouncement {
            feature_name: "Video Conferencing".to_string(),
            description: "You can now start video calls directly from chat conversations!".to_string(),
        }),
        
        // Tip message
        BotMessage::new(BotMessageType::Tip {
            tip: "Did you know you can organize your circles into favorites?".to_string(),
        }),
        
        // Another cat message
        BotMessage::new(BotMessageType::CatMessage {
            message: "Remember to take breaks! A good stretch every hour keeps you feline fine! 😺".to_string(),
            image_emoji: "😸".to_string(),
        }),
        
        // Update message for initial release
        BotMessage::new(BotMessageType::Update {
            version: "1.0.0".to_string(),
            changes: vec![
                "Initial release of Circles".to_string(),
                "Basic collaboration features available".to_string(),
                "Mail, Calendar, Chat, and Documents features enabled".to_string(),
            ],
        }),
    ]
}
