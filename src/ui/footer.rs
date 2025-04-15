use crate::app::CircleApp;
use crate::utils::config::Theme;
use chrono::Local;
use eframe::egui::{
    Align, Color32, Context, Frame, Layout, Margin, RichText, Stroke, TopBottomPanel, Vec2,
};

// Footer rendering function
pub fn render_status_bar(app: &CircleApp, ctx: &Context, app_layout: &Frame, theme: &Theme) {
    TopBottomPanel::bottom("status_bar")
        .exact_height(40.0)
        .frame(app_layout.clone().corner_radius(0))
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Left side - Connection status
                ui.add_space(16.0);
                let _status_frame = Frame::new()
                    .fill(Color32::from_rgb(240, 245, 250))
                    .corner_radius(12)
                    .inner_margin(Margin::symmetric(10, 4))
                    .stroke(Stroke::new(1.0, Color32::from_rgb(220, 225, 230)))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            // Green dot for connected status
                            ui.painter().circle_filled(
                                ui.min_rect().left_center() + Vec2::new(6.0, 0.0),
                                4.0,
                                Color32::from_rgb(50, 180, 50),
                            );
                            ui.add_space(12.0);
                            ui.label(
                                RichText::new("Connected")
                                    .size(13.0)
                                    .color(Color32::from_rgb(70, 80, 90)),
                            );
                        });
                    });

                ui.add_space(12.0);

                // Current date and time
                let now = Local::now();
                let date_str = now.format("%d %b %Y").to_string();
                let time_str = now.format("%H:%M").to_string();

                ui.label(
                    RichText::new(format!("📅 {}", date_str))
                        .size(13.0)
                        .color(Color32::from_rgb(70, 80, 90)),
                );

                ui.add_space(8.0);

                ui.label(
                    RichText::new(format!("🕒 {}", time_str))
                        .size(13.0)
                        .color(Color32::from_rgb(70, 80, 90)),
                );

                // Right side - User info and notifications
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.add_space(16.0);

                    // Notifications indicator
                    let unread_count = 3; // This would come from app state in a real implementation
                    let notif_frame = Frame::new()
                        .fill(if unread_count > 0 {
                            Color32::from_rgb(240, 70, 70)
                        } else {
                            Color32::from_rgb(200, 210, 220)
                        })
                        .corner_radius(10)
                        .inner_margin(Margin::symmetric(8, 4))
                        .show(ui, |ui| {
                            ui.label(
                                RichText::new(format!("🔔 {}", unread_count))
                                    .size(13.0)
                                    .color(Color32::WHITE)
                                    .strong(),
                            );
                        });

                    if notif_frame.response.hovered() {
                        ui.output_mut(|o| o.cursor_icon = eframe::egui::CursorIcon::PointingHand);
                    }

                    ui.add_space(12.0);

                    // User status
                    let user_name = app.user.as_ref().map_or("Guest", |u| &u.name);
                    let user_frame = Frame::new()
                        .fill(theme.hover)
                        .corner_radius(12)
                        .inner_margin(Margin::symmetric(10, 4))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                // Status indicator - green for online
                                ui.painter().circle_filled(
                                    ui.min_rect().left_center() + Vec2::new(6.0, 0.0),
                                    4.0,
                                    theme.success,
                                );
                                ui.add_space(12.0);
                                ui.label(
                                    RichText::new(format!("{}", user_name))
                                        .size(13.0)
                                        .color(theme.text)
                                        .strong(),
                                );
                            });
                        });

                    if user_frame.response.hovered() {
                        ui.output_mut(|o| o.cursor_icon = eframe::egui::CursorIcon::PointingHand);
                    }
                });
            });
        });
}
