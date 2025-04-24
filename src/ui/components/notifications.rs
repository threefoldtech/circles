use chrono::{DateTime, Local, Utc};
use egui::{Align, Button, Color32, Frame, Layout, Margin, RichText, Stroke, Vec2};
use std::collections::VecDeque;

use crate::{
    models::notification::AppNotification,
    ui::components::notification_dialog::NotificationDialogState, utils::config::Theme,
};

#[derive(Debug)]
pub struct NotificationManager {
    notifications: VecDeque<AppNotification>,
    max_notifications: usize,
    pub show_panel: bool,
    pub notification_dialog: NotificationDialogState,
}

impl NotificationManager {
    pub fn new(max_notifications: usize) -> Self {
        Self {
            notifications: VecDeque::new(),
            max_notifications,
            show_panel: false,
            notification_dialog: NotificationDialogState::new(),
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

    // Helper function to truncate text with ellipsis
    fn truncate_text(text: &str, max_len: usize) -> String {
        if text.len() <= max_len {
            text.to_string()
        } else {
            format!("{}...", &text[..max_len])
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        // Dialog is now rendered at the app level in app_layout.rs
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

        // Create a scrollable area for notifications
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                // Display notifications
                for notification in self.notifications.iter() {
                    ui.add_space(8.0);
                    let response = self.render_notification_card(ui, notification, theme);

                    // Handle click to open dialog
                    if response.clicked() {
                        self.notification_dialog.open(notification.clone());
                    }
                }

                // Add space at the bottom for buttons
                ui.add_space(16.0);
            });

        // Add separator before buttons
        ui.separator();

        // Buttons at the bottom left of the panel
        ui.horizontal(|ui| {
            // Left side - Clear All button
            let clear_button = ui.add(
                egui::Button::new(RichText::new("Clear All").size(13.0).color(Color32::WHITE))
                    .corner_radius(6)
                    .fill(theme.error)
                    .stroke(Stroke::NONE)
                    .min_size(Vec2::new(100.0, 36.0)),
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
                .min_size(Vec2::new(100.0, 36.0)),
            );
            if mark_read_button.hovered() {
                ui.output_mut(|o| o.cursor_icon = eframe::egui::CursorIcon::PointingHand);
            }
            if mark_read_button.clicked() {
                self.mark_all_read();
            }
        });
    }

    // Helper method to render a notification card
    fn render_notification_card(
        &self,
        ui: &mut egui::Ui,
        notification: &AppNotification,
        theme: &Theme,
    ) -> egui::Response {
        // Make the card take the full width with padding
        let available_width = ui.available_width() - 10.0; // 5px padding on each side

        let card = Frame::new()
            .fill(theme.hover)
            .inner_margin(Margin::symmetric(12, 8))
            .corner_radius(6.0)
            .show(ui, |ui| {
                // Set a fixed width for the card content to prevent panel expansion
                ui.set_min_width(available_width);
                ui.set_max_width(available_width);

                ui.vertical(|ui| {
                    // Title with timestamp
                    ui.horizontal(|ui| {
                        // Use the existing truncate_text helper function
                        let truncated_title = Self::truncate_text(&notification.title, 30);

                        // Add the title with fixed width
                        ui.label(
                            RichText::new(truncated_title)
                                .size(14.0)
                                .strong()
                                .color(theme.text),
                        );

                        // Add timestamp on the right
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

                    // Message with truncation
                    ui.horizontal(|ui| {
                        // Use the existing truncate_text helper function
                        let truncated_message = Self::truncate_text(&notification.message, 60);

                        // Add the message with fixed width
                        ui.label(
                            RichText::new(truncated_message)
                                .size(13.0)
                                .color(theme.text),
                        );
                    });

                    // Action button if URL exists
                    if let Some(action_url) = &notification.action_url {
                        ui.add_space(4.0);
                        if ui
                            .add(
                                Button::new(
                                    RichText::new("View Details").size(12.0).color(theme.accent),
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

        // Make the entire card clickable
        card.response.interact(egui::Sense::click())
    }
}
