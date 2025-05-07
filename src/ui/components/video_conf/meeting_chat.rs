use eframe::egui::{self, Align, Align2, Button, Layout, RichText, ScrollArea};
use uuid::Uuid;

use crate::models::features::video_conf::{ChatMessage, VideoConferenceState};
use crate::utils::config::Theme;
use chrono::Utc;

use super::utils::format_timestamp;

/// Chat tab component for the video conference meeting
pub struct MeetingChat;

impl MeetingChat {
    /// Render the meeting chat tab
    pub fn render(ui: &mut egui::Ui, state: &mut VideoConferenceState, theme: &Theme, user_id: Uuid) {
        if let Some(meeting) = state.current_meeting.as_mut() {
            ui.vertical(|ui| {
                ui.heading(RichText::new("Chat").color(theme.header_text));
                ui.add_space(10.0);

                // Chat messages area
                let available_height = ui.available_height() - 60.0; // Reserve space for input
                ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .stick_to_bottom(true)
                    .max_height(available_height)
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            if meeting.chat_messages.is_empty() {
                                ui.vertical_centered(|ui| {
                                    ui.add_space(20.0);
                                    ui.label(
                                        RichText::new("No messages yet")
                                            .size(14.0)
                                            .color(theme.secondary_text),
                                    );
                                });
                            } else {
                                for message in &meeting.chat_messages {
                                    let is_from_me = message.sender_id == user_id;
                                    Self::render_chat_message(ui, message, is_from_me, theme);
                                    ui.add_space(8.0);
                                }
                            }
                        });
                    });

                ui.separator();

                // Input area
                ui.horizontal(|ui| {
                    let text_edit_response = ui.add(
                        egui::TextEdit::singleline(&mut state.chat_input)
                            .hint_text("Type a message...")
                            .desired_width(ui.available_width() - 60.0),
                    );

                    let send_button = ui.button("Send");

                    // Check for Enter key or button click
                    let send_message = (ui.ctx().input(|i| i.key_pressed(egui::Key::Enter))
                        && !state.chat_input.is_empty())
                        || (send_button.clicked() && !state.chat_input.is_empty());

                    if send_message {
                        // Get the current user's name from the participants list
                        let sender_name = meeting
                            .participants
                            .iter()
                            .find(|p| p.id == user_id)
                            .map(|p| p.name.clone())
                            .unwrap_or_else(|| "You".to_string());

                        // Create a new chat message
                        let message = ChatMessage {
                            id: Uuid::new_v4(),
                            sender_id: user_id,
                            sender_name,
                            content: state.chat_input.clone(),
                            sent_at: Utc::now(),
                            is_private: false,
                            recipient_id: None,
                            attachment: None,
                        };

                        // Add the message to the meeting
                        meeting.add_chat_message(message);

                        // Clear the input
                        state.chat_input.clear();
                    }
                });
            });
        }
    }

    /// Render a single chat message
    fn render_chat_message(ui: &mut egui::Ui, message: &ChatMessage, is_from_me: bool, theme: &Theme) {
        let align = if is_from_me {
            Align::RIGHT
        } else {
            Align::LEFT
        };

        let bg_color = if is_from_me {
            theme.accent
        } else {
            theme.secondary_background
        };

        let text_color = if is_from_me {
            theme.light_color
        } else {
            theme.text
        };

        ui.with_layout(Layout::top_down(align), |ui| {
            // Add sender name for messages from others
            if !is_from_me {
                ui.label(
                    RichText::new(&message.sender_name)
                        .size(12.0)
                        .strong()
                        .color(theme.secondary_text),
                );
                ui.add_space(2.0);
            }

            // Message bubble
            egui::Frame::none()
                .fill(bg_color)
                .rounding(8.0)
                .inner_margin(8.0)
                .show(ui, |ui| {
                    ui.label(RichText::new(&message.content).color(text_color));
                });

            // Timestamp
            ui.label(
                RichText::new(format_timestamp(message.sent_at))
                    .size(10.0)
                    .color(theme.secondary_text),
            );
        });
    }
}
