use chrono::{DateTime, Local, Utc};
use egui::{Align, Button, Color32, Direction, Frame, Layout, Margin, RichText, Stroke, Vec2};
use std::collections::VecDeque;

use crate::{models::notification::AppNotification, utils::config::Theme};

#[derive(Debug)]
pub struct NotificationManager {
    notifications: VecDeque<AppNotification>,
    max_notifications: usize,
    pub show_panel: bool,
}

impl NotificationManager {
    pub fn new(max_notifications: usize) -> Self {
        Self {
            notifications: VecDeque::new(),
            max_notifications,
            show_panel: false,
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
        if self.notifications.is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.label(
                    RichText::new("No notifications")
                        .size(14.0)
                        .color(theme.secondary_text),
                );
            });
            return;
        }

        // Buttons
        ui.horizontal(|ui| {
            ui.with_layout(
                Layout::centered_and_justified(Direction::LeftToRight),
                |ui| {
                    ui.add_space(16.0);
                    // Clear All button
                    let clear_button = ui.add(
                        egui::Button::new(
                            RichText::new("Clear All").size(13.0).color(Color32::WHITE),
                        )
                        .corner_radius(6)
                        .fill(theme.error)
                        .stroke(Stroke::NONE)
                        .min_size(Vec2::new(40.0, 36.0)),
                    );

                    if clear_button.hovered() {
                        ui.output_mut(|o| o.cursor_icon = eframe::egui::CursorIcon::PointingHand);
                    }
                    if clear_button.clicked() {
                        self.clear();
                    }

                    ui.add_space(8.0);

                    // Mark All Read button
                    let mark_read_button = ui.add(
                        egui::Button::new(
                            RichText::new("Mark All Read")
                                .size(13.0)
                                .color(Color32::WHITE),
                        )
                        .corner_radius(6)
                        .fill(theme.active)
                        .stroke(Stroke::NONE)
                        .min_size(Vec2::new(40.0, 36.0)),
                    );
                    if mark_read_button.hovered() {
                        ui.output_mut(|o| o.cursor_icon = eframe::egui::CursorIcon::PointingHand);
                    }
                    if mark_read_button.clicked() {
                        self.mark_all_read();
                    }
                },
            );
        });

        ui.separator();

        for notification in self.notifications.iter() {
            ui.add_space(8.0);
            Frame::new()
                .fill(theme.hover)
                .inner_margin(Margin::symmetric(12, 8))
                .corner_radius(6.0)
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        // Title with timestamp
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(&notification.title)
                                    .size(14.0)
                                    .strong()
                                    .color(theme.text),
                            );
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                // Convert SystemTime to chrono::DateTime<Local>
                                let duration_since_epoch = notification
                                    .created_at
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap();
                                let timestamp = DateTime::<Utc>::from_timestamp(
                                    duration_since_epoch.as_secs() as i64,
                                    duration_since_epoch.subsec_nanos(),
                                )
                                .unwrap()
                                .with_timezone(&Local);
                                ui.label(
                                    RichText::new(timestamp.format("%H:%M").to_string())
                                        .size(12.0)
                                        .color(Color32::from_rgb(120, 130, 140))
                                        .italics(),
                                );
                            });
                        });

                        // Message
                        ui.label(
                            RichText::new(&notification.message)
                                .size(13.0)
                                .color(theme.text),
                        );

                        // Action button if URL exists
                        if let Some(action_url) = &notification.action_url {
                            ui.add_space(4.0);
                            if ui
                                .add(
                                    Button::new(
                                        RichText::new("View Details")
                                            .size(12.0)
                                            .color(theme.accent),
                                    )
                                    .frame(false),
                                )
                                .clicked()
                            {
                                if let Err(e) = open::that(action_url) {
                                    eprintln!("Failed to open URL: {}", e);
                                }
                            }
                        }
                    });
                });
        }
    }
}
