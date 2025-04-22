use egui::{Color32, RichText, Stroke};
use std::collections::VecDeque;

use crate::{
    models::notification::{AppNotification, NotificationPriority},
    utils::config::Theme,
};

#[derive(Debug)]
pub struct NotificationManager {
    notifications: VecDeque<AppNotification>,
    max_notifications: usize,
}

impl NotificationManager {
    pub fn new(max_notifications: usize) -> Self {
        Self {
            notifications: VecDeque::new(),
            max_notifications,
        }
    }

    pub fn add(&mut self, notification: AppNotification) {
        if self.notifications.len() >= self.max_notifications {
            self.notifications.pop_back();
        }
        self.notifications.push_front(notification);
    }

    pub fn clear(&mut self) {
        self.notifications.clear();
    }

    pub fn mark_all_read(&mut self) {
        for notification in &mut self.notifications {
            notification.read = true;
        }
    }

    pub fn unread_count(&self) -> usize {
        self.notifications.iter().filter(|n| !n.read).count()
    }

    pub fn render(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        for notification in &mut self.notifications {
            let frame = egui::Frame::none()
                .fill(if !notification.read {
                    theme.hover
                } else {
                    theme.background
                })
                .inner_margin(egui::Margin::symmetric(12, 8))
                .corner_radius(8.0)
                .stroke(Stroke::new(1.0, theme.border));

            frame.show(ui, |ui| {
                ui.horizontal(|ui| {
                    // Priority indicator
                    let priority_color = match notification.priority {
                        NotificationPriority::Low => Color32::from_rgb(70, 170, 70),
                        NotificationPriority::Normal => Color32::from_rgb(66, 133, 244),
                        NotificationPriority::High => Color32::from_rgb(220, 50, 50),
                    };

                    ui.add(egui::Label::new(
                        RichText::new("●").color(priority_color).size(16.0),
                    ));

                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(&notification.title)
                                    .size(14.0)
                                    .strong()
                                    .color(theme.text),
                            );

                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    let age = notification.age();
                                    let age_text = if age.as_secs() < 60 {
                                        "Just now".to_string()
                                    } else if age.as_secs() < 3600 {
                                        format!("{}m ago", age.as_secs() / 60)
                                    } else if age.as_secs() < 86400 {
                                        format!("{}h ago", age.as_secs() / 3600)
                                    } else {
                                        format!("{}d ago", age.as_secs() / 86400)
                                    };

                                    ui.label(
                                        RichText::new(age_text)
                                            .size(12.0)
                                            .color(theme.secondary_text),
                                    );
                                },
                            );
                        });

                        ui.label(
                            RichText::new(&notification.message)
                                .size(13.0)
                                .color(theme.secondary_text),
                        );

                        if let Some(action_url) = &notification.action_url {
                            ui.add_space(4.0);
                            if ui
                                .add(
                                    egui::Button::new(
                                        RichText::new("View Details")
                                            .size(12.0)
                                            .color(theme.accent),
                                    )
                                    .frame(false),
                                )
                                .clicked()
                            {
                                // Handle action URL click
                                if let Err(e) = open::that(action_url) {
                                    eprintln!("Failed to open URL: {}", e);
                                }
                            }
                        }
                    });
                });
            });

            ui.add_space(4.0);
        }
    }
}
