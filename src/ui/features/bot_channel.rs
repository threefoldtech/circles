use crate::app::CircleApp;
use crate::ui::app_layout::create_content_frame;
use crate::ui::components::bot_message::{BotMessage, BotMessageType};
use eframe::egui::{self, Color32};
use egui::{RichText, Stroke};

/// Render the bot channel UI
pub fn render_bot_channel(app: &CircleApp, ui: &mut egui::Ui) {
    ui.add_space(16.0);

    let theme = app.get_current_theme();
    create_content_frame(&theme).show(ui, |ui| {
        ui.vertical(|ui| {
            // Message history area
            // Use a much smaller height to ensure messages are visible above the footer
            let available_height = ui.available_height() - 100.0;

            // Create a scrollable area for messages
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .stick_to_bottom(false)
                .max_height(available_height)
                .id_salt("bot_messages_scroll_area")
                .show(ui, |ui| {
                    // Create a custom frame for the bot messages area with theme-aware styling
                    let message_area_frame = egui::Frame::new()
                        .fill(theme.background)
                        .stroke(Stroke::new(1.0, theme.border))
                        .inner_margin(egui::Margin::same(0))
                        .outer_margin(egui::Margin::same(0));

                    message_area_frame.show(ui, |ui| {
                        // Apply theme-aware styling to the UI
                        ui.style_mut().visuals.override_text_color = Some(theme.text);

                        // Override all frame colors to use theme colors
                        ui.style_mut().visuals.widgets.noninteractive.bg_fill = theme.panel;
                        ui.style_mut().visuals.widgets.inactive.bg_fill =
                            theme.secondary_background;
                        ui.style_mut().visuals.widgets.hovered.bg_fill = theme.hover;
                        ui.style_mut().visuals.widgets.active.bg_fill = theme.active;
                        // Display bot messages
                        render_bot_messages(ui, &theme);
                    });
                });

            ui.add_space(16.0);

            // Bottom info area
            ui.separator();
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 10.0;

                // Bot status indicator
                let bot_status_frame = egui::Frame::new()
                    .fill(theme.success) // Use theme success color instead of hardcoded green
                    .corner_radius(4)
                    .inner_margin(egui::Margin::same(4));

                bot_status_frame.show(ui, |ui| {
                    ui.label(RichText::new("●").size(10.0).color(Color32::WHITE));
                });

                ui.label(
                    RichText::new("Circles Bot is active")
                        .size(14.0)
                        .color(theme.text),
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new("You can only receive messages in this channel")
                            .size(14.0)
                            .color(theme.secondary_text)
                            .italics(),
                    );
                });
            });
        });
    });
}

/// Render the bot messages
fn render_bot_messages(ui: &mut egui::Ui, theme: &crate::utils::config::Theme) {
    // Create example messages using our component
    let messages = get_example_messages();

    // Render each message
    for (i, message) in messages.iter().enumerate() {
        // Create a frame for each message with a consistent background using theme colors
        let frame = egui::Frame::new()
            .fill(theme.panel) // Use panel color from theme
            .stroke(Stroke::new(1.0, theme.border)) // Use border color from theme
            .corner_radius(8.0)
            .inner_margin(egui::Margin::same(16));

        frame.show(ui, |ui| {
            message.render(ui);
        });

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
        // Connection message
        BotMessage::new(BotMessageType::Connection {
            circle_name: "Project Alpha Team".to_string(),
        }),
        // Feature announcement
        BotMessage::new(BotMessageType::FeatureAnnouncement {
            feature_name: "Video Conferencing".to_string(),
            description: "You can now start video calls directly from chat conversations!"
                .to_string(),
        }),
        // Tip message
        BotMessage::new(BotMessageType::Tip {
            tip: "Did you know you can organize your circles into favorites?".to_string(),
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
