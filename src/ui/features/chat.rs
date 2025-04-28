use chrono::Utc;
use eframe::egui::{self, RichText, ScrollArea, TextEdit, Ui};
use uuid::Uuid;

use crate::app::CircleApp;
use crate::models::dummy_data::{Conversation, Message};
use crate::ui::app_layout::create_content_frame;
use crate::ui::components::button;
use crate::utils::config::Theme;

// Constants for AI chat
const AI_CHAT_ID: &str = "AI Assistant";
const AI_USER: &str = "AI Assistant";
const USER_NAME: &str = "You";

// Simple implementation of the chat feature
pub fn render_chat(app: &mut CircleApp, ui: &mut egui::Ui, theme: &Theme) {
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        if ui
            .add(button::create_button("New Chat", "➕", theme))
            .clicked()
        {
            // Clear chat history by creating a new conversation
            if let Some(active_data) = &mut app.active_feature_data {
                // Find AI chat conversation
                let ai_chat_index = active_data.chat_data.conversations.iter()
                    .position(|c| c.name == AI_CHAT_ID);
                
                if let Some(index) = ai_chat_index {
                    // Clear messages in existing conversation
                    active_data.chat_data.conversations[index].messages.clear();
                } else {
                    // Create new AI chat conversation
                    let new_conversation = Conversation {
                        id: Uuid::new_v4(),
                        name: AI_CHAT_ID.to_string(),
                        participants: vec![USER_NAME.to_string(), AI_USER.to_string()],
                        messages: Vec::new(),
                    };
                    active_data.chat_data.conversations.push(new_conversation);
                }
            }
        }
        ui.add_space(8.0);
    });
    ui.add_space(16.0);
    
    // Create a frame with bottom and right margin
    let frame = create_content_frame(theme)
        .outer_margin(egui::Margin {
            left: 0,
            right: 20, // Add 20px right margin
            top: 0,
            bottom: 30, // Add 30px bottom margin
        });
    
    frame.show(ui, |ui| {
        let available_height = ui.available_height();
        let input_area_height = 60.0;
        let bottom_margin = 55.0; // Add bottom margin
        let messages_area_height = available_height - input_area_height - bottom_margin;
        
        if let Some(active_data) = &mut app.active_feature_data {
            // Find or create AI chat conversation
            let ai_chat_index = active_data.chat_data.conversations.iter()
                .position(|c| c.name == AI_CHAT_ID);
            
            let ai_chat_index = if let Some(index) = ai_chat_index {
                index
            } else {
                // Create new AI chat conversation
                let new_conversation = Conversation {
                    id: Uuid::new_v4(),
                    name: AI_CHAT_ID.to_string(),
                    participants: vec![USER_NAME.to_string(), AI_USER.to_string()],
                    messages: Vec::new(),
                };
                active_data.chat_data.conversations.push(new_conversation);
                active_data.chat_data.conversations.len() - 1
            };
            
            // Get the AI chat conversation
            let conversation = &mut active_data.chat_data.conversations[ai_chat_index];
            
            // Messages area
            ScrollArea::vertical()
                .auto_shrink([false, false])
                .stick_to_bottom(true)
                .max_height(messages_area_height)
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(16.0);
                        
                        if conversation.messages.is_empty() {
                            ui.label(
                                RichText::new("Start a conversation with the AI assistant")
                                    .size(16.0)
                                    .color(theme.secondary_text),
                            );
                        } else {
                            for message in &conversation.messages {
                                render_chat_message(ui, message, theme);
                                ui.add_space(16.0);
                            }
                        }
                    });
                });
            
            ui.add_space(8.0);
            
            // Input area at the bottom
            ui.horizontal(|ui| {
                // Store input text in the conversation's name field temporarily
                // This is a hack but allows us to persist the input text between frames
                let input_text_id = format!("input_text_{}", conversation.id);
                
                // Get or create the input text from memory
                let mut input_text = ui.ctx().memory_mut(|mem| {
                    mem.data.get_temp::<String>(egui::Id::new(&input_text_id))
                        .unwrap_or_default()
                });
                
                let input_text_edit = TextEdit::multiline(&mut input_text)
                    .desired_width(ui.available_width() - 80.0)
                    .desired_rows(1)
                    .hint_text("Type a message...")
                    .margin(egui::vec2(8.0, 8.0));
                
                let text_edit_response = ui.add(input_text_edit);
                text_edit_response.request_focus();
                
                // Store the input text in memory
                ui.ctx().memory_mut(|mem| {
                    mem.data.insert_temp(egui::Id::new(&input_text_id), input_text.clone());
                });
                
                // Send button
                let send_button = button::render_button(ui, "Send", false, theme, Some("📤"));
                
                // Check for Enter key or button click
                let send_message =
                    (ui.ctx().input(|i| i.key_pressed(egui::Key::Enter) && !i.modifiers.shift) && !input_text.is_empty()) ||
                    (send_button.clicked() && !input_text.is_empty());
                
                if send_message {
                    // Add user message
                    let user_message = Message {
                        id: Uuid::new_v4(),
                        sender: USER_NAME.to_string(),
                        content: input_text.clone(),
                        timestamp: Utc::now(),
                        read: true,
                    };
                    
                    conversation.messages.push(user_message);
                    
                    // Add AI response immediately (no animation for simplicity)
                    let ai_message = Message {
                        id: Uuid::new_v4(),
                        sender: AI_USER.to_string(),
                        content: "I'm an AI assistant. How can I help you today? I can provide information, answer questions, or assist with various tasks.".to_string(),
                        timestamp: Utc::now(),
                        read: true,
                    };
                    
                    conversation.messages.push(ai_message);
                    
                    // Clear the input text
                    ui.ctx().memory_mut(|mem| {
                        mem.data.insert_temp(egui::Id::new(&input_text_id), String::new());
                    });
                    
                    // Request a repaint to show the new messages
                    ui.ctx().request_repaint();
                }
            });
            
            // Add explicit bottom margin
            ui.add_space(30.0);
        }
    });
    ui.add_space(22.0);
}

fn render_chat_message(ui: &mut Ui, message: &Message, theme: &Theme) {
    // Different styling based on sender
    let is_user = message.sender == USER_NAME;
    let (bg_color, align, text_color) = if is_user {
        (theme.accent, egui::Align::RIGHT, theme.white)
    } else {
        (theme.panel, egui::Align::LEFT, theme.text)
    };
    
    // Add some vertical spacing between messages
    ui.add_space(8.0);
    
    // Use different layout based on sender
    ui.with_layout(egui::Layout::top_down(align), |ui| {
        let max_width = ui.available_width() * 0.75; // Limit message width to 75% of available width
        
        // Add sender name for AI messages
        if !is_user {
            ui.label(
                RichText::new(&message.sender)
                    .size(12.0)
                    .strong()
                    .color(theme.secondary_text)
            );
            ui.add_space(2.0);
        }
        
        // Create a frame for the message with enhanced styling
        let message_frame = egui::Frame::new()
            .fill(bg_color)
            .stroke(egui::Stroke::new(1.0, if is_user { theme.accent } else { theme.border }))
            .corner_radius(12) // Larger corner radius for more rounded bubbles
            .inner_margin(egui::Margin::same(12));
        
        message_frame.show(ui, |ui| {
            ui.set_max_width(max_width);
            
            // Show the message content with better text styling
            ui.label(
                RichText::new(&message.content)
                    .size(15.0) // Slightly larger text
                    .color(text_color)
                    .text_style(egui::TextStyle::Body)
            );
        });
        
        // Show timestamp below message with better styling
        ui.add_space(4.0);
        ui.label(
            RichText::new(format_timestamp(message.timestamp))
                .size(10.0)
                .italics()
                .color(theme.secondary_text),
        );
    });
}

fn format_timestamp(timestamp: chrono::DateTime<Utc>) -> String {
    // Format timestamp as HH:MM
    timestamp.format("%H:%M").to_string()
}
