use eframe::egui::{self, Color32, RichText, Stroke, Vec2};
use chrono::{DateTime, Utc};

/// Different types of bot messages
#[derive(Debug, Clone)]
pub enum BotMessageType {
    Welcome,
    Update {
        version: String,
        changes: Vec<String>,
    },
    Connection {
        circle_name: String,
    },
    FeatureAnnouncement {
        feature_name: String,
        description: String,
    },
    Tip {
        tip: String,
    },
    CatMessage {
        message: String,
        image_emoji: String,
    },
}

/// A message from the bot
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct BotMessage {
    pub message_type: BotMessageType,
    pub timestamp: DateTime<Utc>,
    pub read: bool,
}

impl BotMessage {
    /// Create a new bot message
    pub fn new(message_type: BotMessageType) -> Self {
        Self {
            message_type,
            timestamp: Utc::now(),
            read: false,
        }
    }

    /// Render the bot message
    pub fn render(&self, ui: &mut egui::Ui) {
        // Get the theme from the app
        let is_dark_mode = ui.ctx().style().visuals.dark_mode;
        
        // Apply theme-appropriate styles to the UI
        if is_dark_mode {
            ui.style_mut().visuals.override_text_color = Some(Color32::from_rgb(241, 245, 249));
        }
        
        match &self.message_type {
            BotMessageType::Welcome => render_welcome_message(ui),
            BotMessageType::Update { version, changes } => {
                let changes_str: Vec<&str> = changes.iter().map(|s| s.as_str()).collect();
                render_update_message(ui, version, &format_timestamp(self.timestamp), &changes_str)
            }
            BotMessageType::Connection { circle_name } => {
                render_connection_message(ui, circle_name, &format_timestamp(self.timestamp))
            }
            BotMessageType::FeatureAnnouncement { feature_name, description } => {
                render_feature_announcement(ui, feature_name, &format_timestamp(self.timestamp), description)
            }
            BotMessageType::Tip { tip } => {
                render_tip_message(ui, tip)
            }
            BotMessageType::CatMessage { message, image_emoji } => {
                render_cat_message(ui, message, image_emoji, &format_timestamp(self.timestamp))
            }
        }
    }
}

/// Format a timestamp for display
fn format_timestamp(timestamp: DateTime<Utc>) -> String {
    // For simplicity, just return a fixed string
    // In a real app, you would format the timestamp based on how recent it is
    let now = Utc::now();
    if (now - timestamp).num_days() == 0 {
        "Today".to_string()
    } else if (now - timestamp).num_days() == 1 {
        "Yesterday".to_string()
    } else {
        // Format as "Month Day"
        timestamp.format("%B %d").to_string()
    }
}

/// Render a welcome message
fn render_welcome_message(ui: &mut egui::Ui) {
    // Determine if we're in dark mode
    let is_dark_mode = ui.ctx().style().visuals.dark_mode;
    
    // Choose appropriate colors based on theme
    let bg_color = if is_dark_mode {
        Color32::from_rgb(30, 41, 59) // Dark mode background
    } else {
        Color32::from_rgb(240, 249, 255) // Light mode background
    };
    
    let text_color = if is_dark_mode {
        Color32::from_rgb(241, 245, 249) // Dark mode text
    } else {
        Color32::from_rgb(40, 50, 60) // Light mode text
    };
    
    let secondary_text_color = if is_dark_mode {
        Color32::from_rgb(148, 163, 184) // Dark mode secondary text
    } else {
        Color32::from_rgb(120, 130, 140) // Light mode secondary text
    };
    
    let message_frame = egui::Frame::new()
        .fill(bg_color)
        .stroke(Stroke::new(1.0, Color32::from_rgba_premultiplied(100, 100, 100, 100)))
        .corner_radius(8.0)
        .inner_margin(egui::Margin::same(16));
    
    message_frame.show(ui, |ui| {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("🤖")
                        .size(20.0)
                );
                
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("Circles Bot")
                            .size(16.0)
                            .strong()
                            .color(text_color)
                    );
                    
                    ui.label(
                        RichText::new("Welcome to the Circles Bot Channel!")
                            .size(14.0)
                            .color(text_color)
                    );
                });
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    ui.label(
                        RichText::new("Just now")
                            .size(12.0)
                            .color(secondary_text_color)
                    );
                });
            });
            
            ui.add_space(8.0);
            
            ui.label(
                RichText::new("I'll keep you updated on system changes, new connections, and helpful tips. You'll only receive messages from me in this channel - it's a one-way communication channel for important announcements.")
                    .size(14.0)
                    .color(text_color)
            );
        });
    });
}

/// Render an update message
fn render_update_message(ui: &mut egui::Ui, version: &str, date: &str, changes: &[&str]) {
    let message_frame = egui::Frame::new()
        .fill(Color32::from_rgb(237, 247, 237))
        .stroke(Stroke::new(1.0, Color32::from_rgb(200, 230, 201)))
        .corner_radius(8.0)
        .inner_margin(egui::Margin::same(16));
    
    message_frame.show(ui, |ui| {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("🚀")
                        .size(20.0)
                );
                
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("Circles Bot")
                            .size(16.0)
                            .strong()
                            .color(Color32::from_rgb(46, 125, 50))
                    );
                    
                    ui.label(
                        RichText::new(format!("System Update: Version {}", version))
                            .size(14.0)
                            .strong()
                            .color(Color32::from_rgb(40, 50, 60))
                    );
                });
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    ui.label(
                        RichText::new(date)
                            .size(12.0)
                            .color(Color32::from_rgb(120, 130, 140))
                    );
                });
            });
            
            ui.add_space(8.0);
            
            ui.label(
                RichText::new("What's new:")
                    .size(14.0)
                    .strong()
                    .color(Color32::from_rgb(70, 80, 90))
            );
            
            for change in changes {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("•")
                            .size(14.0)
                            .color(Color32::from_rgb(46, 125, 50))
                    );
                    
                    ui.label(
                        RichText::new(*change)
                            .size(14.0)
                            .color(Color32::from_rgb(70, 80, 90))
                    );
                });
            }
        });
    });
}

/// Render a connection message
fn render_connection_message(ui: &mut egui::Ui, circle_name: &str, date: &str) {
    let message_frame = egui::Frame::new()
        .fill(Color32::from_rgb(232, 245, 253))
        .stroke(Stroke::new(1.0, Color32::from_rgb(187, 222, 251)))
        .corner_radius(8.0)
        .inner_margin(egui::Margin::same(16));
    
    message_frame.show(ui, |ui| {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("🔗")
                        .size(20.0)
                );
                
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("Circles Bot")
                            .size(16.0)
                            .strong()
                            .color(Color32::from_rgb(2, 119, 189))
                    );
                    
                    ui.label(
                        RichText::new("New Connection")
                            .size(14.0)
                            .strong()
                            .color(Color32::from_rgb(40, 50, 60))
                    );
                });
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    ui.label(
                        RichText::new(date)
                            .size(12.0)
                            .color(Color32::from_rgb(120, 130, 140))
                    );
                });
            });
            
            ui.add_space(8.0);
            
            ui.label(
                RichText::new(format!("You've been connected to the \"{}\" circle. You can now collaborate with other members of this circle.", circle_name))
                    .size(14.0)
                    .color(Color32::from_rgb(70, 80, 90))
            );
        });
    });
}

/// Render a feature announcement
fn render_feature_announcement(ui: &mut egui::Ui, feature_name: &str, date: &str, description: &str) {
    let message_frame = egui::Frame::new()
        .fill(Color32::from_rgb(243, 229, 245))
        .stroke(Stroke::new(1.0, Color32::from_rgb(206, 147, 216)))
        .corner_radius(8.0)
        .inner_margin(egui::Margin::same(16));
    
    message_frame.show(ui, |ui| {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("✨")
                        .size(20.0)
                );
                
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("Circles Bot")
                            .size(16.0)
                            .strong()
                            .color(Color32::from_rgb(123, 31, 162))
                    );
                    
                    ui.label(
                        RichText::new(format!("New Feature: {}", feature_name))
                            .size(14.0)
                            .strong()
                            .color(Color32::from_rgb(40, 50, 60))
                    );
                });
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    ui.label(
                        RichText::new(date)
                            .size(12.0)
                            .color(Color32::from_rgb(120, 130, 140))
                    );
                });
            });
            
            ui.add_space(8.0);
            
            ui.label(
                RichText::new(description)
                    .size(14.0)
                    .color(Color32::from_rgb(70, 80, 90))
            );
            
            ui.add_space(8.0);
            
            // Feature button
            let feature_button = egui::Button::new(
                RichText::new(format!("Try {} Now", feature_name))
                    .size(14.0)
                    .color(Color32::WHITE)
            )
            .min_size(Vec2::new(150.0, 32.0))
            .corner_radius(16)
            .fill(Color32::from_rgb(123, 31, 162))
            .stroke(Stroke::new(1.0, Color32::from_rgb(106, 27, 154)));
            
            ui.add(feature_button);
        });
    });
}

/// Render a tip message
fn render_tip_message(ui: &mut egui::Ui, tip: &str) {
    let message_frame = egui::Frame::new()
        .fill(Color32::from_rgb(255, 243, 224))
        .stroke(Stroke::new(1.0, Color32::from_rgb(255, 224, 178)))
        .corner_radius(8.0)
        .inner_margin(egui::Margin::same(16));
    
    message_frame.show(ui, |ui| {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("💡")
                        .size(20.0)
                );
                
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("Circles Bot")
                            .size(16.0)
                            .strong()
                            .color(Color32::from_rgb(230, 81, 0))
                    );
                    
                    ui.label(
                        RichText::new("Tip")
                            .size(14.0)
                            .strong()
                            .color(Color32::from_rgb(40, 50, 60))
                    );
                });
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    ui.label(
                        RichText::new("Today")
                            .size(12.0)
                            .color(Color32::from_rgb(120, 130, 140))
                    );
                });
            });
            
            ui.add_space(8.0);
            
            ui.label(
                RichText::new(tip)
                    .size(14.0)
                    .color(Color32::from_rgb(70, 80, 90))
            );
        });
    });
}

/// Render a cat message
fn render_cat_message(ui: &mut egui::Ui, message: &str, image_emoji: &str, date: &str) {
    let message_frame = egui::Frame::new()
        .fill(Color32::from_rgb(255, 240, 245)) // Light pink background
        .stroke(Stroke::new(1.0, Color32::from_rgb(255, 182, 193))) // Pink border
        .corner_radius(8.0)
        .inner_margin(egui::Margin::same(16));
    
    message_frame.show(ui, |ui| {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(image_emoji) // Cat emoji or other image representation
                        .size(20.0)
                );
                
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("Cat Bot")
                            .size(16.0)
                            .strong()
                            .color(Color32::from_rgb(219, 68, 119)) // Pink color for cat bot
                    );
                    
                    ui.label(
                        RichText::new("Meow Message")
                            .size(14.0)
                            .strong()
                            .color(Color32::from_rgb(40, 50, 60))
                    );
                });
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    ui.label(
                        RichText::new(date)
                            .size(12.0)
                            .color(Color32::from_rgb(120, 130, 140))
                    );
                });
            });
            
            ui.add_space(8.0);
            
            ui.label(
                RichText::new(message)
                    .size(14.0)
                    .color(Color32::from_rgb(70, 80, 90))
            );
            
            // Add a playful element - paw prints
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new("🐾 🐾 🐾")
                            .size(14.0)
                            .color(Color32::from_rgb(219, 68, 119))
                    );
                });
            });
        });
    });
}