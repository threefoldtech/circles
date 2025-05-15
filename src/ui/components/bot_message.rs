use crate::utils::config::Theme;
use chrono::{DateTime, Utc};
use eframe::egui::{self, RichText, Vec2};

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

    /// Render the bot message content (without the frame)
    pub fn render(&self, ui: &mut egui::Ui, theme: &Theme) {
        // Render the message content based on type
        match &self.message_type {
            BotMessageType::Welcome => {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("🤖").size(20.0));

                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("Circles Bot")
                                .size(16.0)
                                .strong()
                                .color(theme.text),
                        );
                        ui.label(
                            RichText::new("Welcome to the Circles Bot Channel!")
                                .size(14.0)
                                .color(theme.text),
                        );
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        ui.label(
                            RichText::new("Just now")
                                .size(12.0)
                                .color(theme.secondary_text),
                        );
                    });
                });

                ui.add_space(8.0);

                ui.label(
                    RichText::new("I'll keep you updated on system changes, new connections, and helpful tips. You'll only receive messages from me in this channel - it's a one-way communication channel for important announcements.")
                        .size(14.0)
                        .color(theme.text)
                );
            }
            BotMessageType::Update { version, changes } => {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("🚀").size(20.0));

                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("Circles Bot")
                                .size(16.0)
                                .strong()
                                .color(theme.text),
                        );
                        ui.label(
                            RichText::new(format!("System Update: Version {}", version))
                                .size(14.0)
                                .strong()
                                .color(theme.text),
                        );
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        ui.label(
                            RichText::new(format_timestamp(self.timestamp))
                                .size(12.0)
                                .color(theme.secondary_text),
                        );
                    });
                });

                ui.add_space(8.0);

                ui.label(
                    RichText::new("What's new:")
                        .size(14.0)
                        .strong()
                        .color(theme.text),
                );

                for change in changes {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("•").size(14.0).color(theme.text));
                        ui.label(RichText::new(change).size(14.0).color(theme.text));
                    });
                }
            }
            BotMessageType::Connection { circle_name } => {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("🔗").size(20.0));

                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("Circles Bot")
                                .size(16.0)
                                .strong()
                                .color(theme.text),
                        );
                        ui.label(
                            RichText::new("New Connection")
                                .size(14.0)
                                .strong()
                                .color(theme.text),
                        );
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        ui.label(
                            RichText::new(format_timestamp(self.timestamp))
                                .size(12.0)
                                .color(theme.secondary_text),
                        );
                    });
                });

                ui.add_space(8.0);

                ui.label(
                    RichText::new(format!("You've been connected to the \"{}\" circle. You can now collaborate with other members of this circle.", circle_name))
                        .size(14.0)
                        .color(theme.text)
                );
            }
            BotMessageType::FeatureAnnouncement {
                feature_name,
                description,
            } => {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("✨").size(20.0));

                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("Circles Bot")
                                .size(16.0)
                                .strong()
                                .color(theme.text),
                        );
                        ui.label(
                            RichText::new(format!("New Feature: {}", feature_name))
                                .size(14.0)
                                .strong()
                                .color(theme.text),
                        );
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        ui.label(
                            RichText::new(format_timestamp(self.timestamp))
                                .size(12.0)
                                .color(theme.secondary_text),
                        );
                    });
                });

                ui.add_space(8.0);

                ui.label(RichText::new(description).size(14.0).color(theme.text));

                ui.add_space(8.0);

                // Feature button
                let feature_button = egui::Button::new(
                    RichText::new(format!("Try {} Now", feature_name))
                        .size(14.0)
                        .color(theme.white),
                )
                .min_size(Vec2::new(150.0, 32.0))
                .corner_radius(16)
                .fill(ui.style().visuals.selection.bg_fill); // Use theme accent color

                ui.add(feature_button);
            }
            BotMessageType::Tip { tip } => {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("💡").size(20.0));

                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("Circles Bot")
                                .size(16.0)
                                .strong()
                                .color(theme.text),
                        );
                        ui.label(RichText::new("Tip").size(14.0).strong().color(theme.text));
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        ui.label(
                            RichText::new("Today")
                                .size(12.0)
                                .color(theme.secondary_text),
                        );
                    });
                });

                ui.add_space(8.0);

                ui.label(RichText::new(tip).size(14.0).color(theme.text));
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
