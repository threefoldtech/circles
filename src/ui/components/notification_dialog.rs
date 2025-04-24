use crate::models::notification::AppNotification;
use crate::ui::components::button::render_button;
use crate::utils::config::Theme;
use chrono::{DateTime, Local, Utc};
use eframe::egui::{Color32, RichText};

/// State for the notification dialog
#[derive(Debug)]
pub struct NotificationDialogState {
    pub is_open: bool,
    pub notification: Option<AppNotification>,
}

impl NotificationDialogState {
    /// Create a new dialog state
    pub fn new() -> Self {
        Self {
            is_open: false,
            notification: None,
        }
    }

    /// Open the dialog with a notification
    pub fn open(&mut self, notification: AppNotification) {
        self.notification = Some(notification);
        self.is_open = true;
    }

    /// Reset the dialog state
    pub fn reset(&mut self) {
        self.notification = None;
        self.is_open = false;
    }
}

/// Render the notification dialog
pub fn render_notification_dialog(
    state: &mut NotificationDialogState,
    ctx: &egui::Context,
    theme: &Theme,
) {
    if !state.is_open || state.notification.is_none() {
        return;
    }

    let notification = state.notification.as_ref().unwrap();
    let mut should_close = false;

    // Create a standardized modal dialog
    egui::Window::new("Notification Details")
        .fixed_size([600.0, 620.0]) // Fixed size to match other dialogs
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .collapsible(false)
        .resizable(false)
        .frame(
            egui::Frame::window(&ctx.style())
                .fill(theme.background)
                .corner_radius(16)
                .shadow(egui::epaint::Shadow {
                    color: Color32::from_black_alpha(25),
                    offset: [0, 4],
                    blur: 8,
                    spread: 0,
                })
                .inner_margin(egui::Margin::same(24)),
        )
        .show(ctx, |ui| {
            // Main layout
            ui.vertical(|ui| {
                // Heading with title
                ui.vertical_centered(|ui| {
                    // Display the full title without truncation
                    ui.heading(
                        RichText::new(&notification.title)
                            .size(24.0)
                            .strong()
                            .color(theme.text),
                    );
                    ui.add_space(8.0);
                });

                // Timestamp
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

                ui.vertical_centered(|ui| {
                    ui.label(
                        RichText::new(timestamp.format("%d %b %Y at %H:%M").to_string())
                            .size(14.0)
                            .color(Color32::from_rgb(120, 130, 140))
                            .italics(),
                    );
                });

                ui.add_space(16.0);

                // Priority indicator
                let priority_color = match notification.priority {
                    crate::models::notification::NotificationPriority::Low => {
                        Color32::from_rgb(100, 180, 100)
                    }
                    crate::models::notification::NotificationPriority::Normal => {
                        Color32::from_rgb(100, 150, 220)
                    }
                    crate::models::notification::NotificationPriority::High => {
                        Color32::from_rgb(220, 100, 100)
                    }
                };

                let priority_text = match notification.priority {
                    crate::models::notification::NotificationPriority::Low => "Low Priority",
                    crate::models::notification::NotificationPriority::Normal => "Normal Priority",
                    crate::models::notification::NotificationPriority::High => "High Priority",
                };

                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(priority_text)
                            .size(14.0)
                            .color(priority_color)
                            .strong(),
                    );
                });

                ui.add_space(16.0);

                // Message content - display full message without truncation
                // Use a scrollable area if the message is very long
                egui::ScrollArea::vertical()
                    .max_height(150.0)
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new(&notification.message)
                                .size(16.0)
                                .color(theme.text),
                        );
                    });

                ui.add_space(16.0);

                // Action URL if available
                if let Some(action_url) = &notification.action_url {
                    if render_button(ui, "Open Link", true, theme, None)
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        if let Err(e) = open::that(action_url) {
                            eprintln!("Failed to open URL: {}", e);
                        }
                    }
                }

                ui.add_space(16.0);

                // Close button at the bottom
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    if render_button(ui, "Close", false, theme, None)
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        should_close = true;
                    }
                });
            });
        });

    if should_close {
        state.reset();
    }
}
