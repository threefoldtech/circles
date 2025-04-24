use crate::ui::components::button::render_button;
use crate::{app::CircleApp, models::dummy_data::Email, utils::config::Theme};
use egui::{FontFamily, FontId, Margin, RichText, Stroke};

use super::compose::{create_forward_draft, create_reply_draft};

#[allow(dead_code)]
/// Render the email details
pub fn render_email_detail(app: &mut CircleApp, ui: &mut egui::Ui, email: &Email, theme: &Theme) {
    ui.vertical(|ui| {
        // Header section
        egui::Frame::new()
            .fill(theme.secondary_background)
            .inner_margin(Margin::same(16))
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    // From field
                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.add_space(8.0);
                            ui.label(
                                RichText::new("From")
                                    .font(FontId::new(13.0, FontFamily::Proportional))
                                    .color(theme.header_text)
                                    .strong(),
                            );
                            ui.add_space(32.0); // Consistent spacing for alignment
                            ui.label(
                                RichText::new(&email.sender)
                                    .font(FontId::new(13.0, FontFamily::Proportional))
                                    .color(theme.text),
                            );
                        });
                    });

                    ui.add_space(8.0);

                    // To field
                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.add_space(8.0);
                            ui.label(
                                RichText::new("To")
                                    .font(FontId::new(13.0, FontFamily::Proportional))
                                    .color(theme.header_text)
                                    .strong(),
                            );
                            ui.add_space(40.0); // Consistent spacing for alignment
                            ui.label(
                                RichText::new(email.recipients.join(", "))
                                    .font(FontId::new(13.0, FontFamily::Proportional))
                                    .color(theme.text),
                            );
                        });
                    });

                    ui.add_space(8.0);

                    // Date field
                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.add_space(8.0);
                            ui.label(
                                RichText::new("Date")
                                    .font(FontId::new(13.0, FontFamily::Proportional))
                                    .color(theme.header_text)
                                    .strong(),
                            );
                            ui.add_space(32.0); // Consistent spacing for alignment
                            let date_str = email.timestamp.format("%d %b %Y, %H:%M").to_string();
                            ui.label(
                                RichText::new(date_str)
                                    .font(FontId::new(13.0, FontFamily::Proportional))
                                    .color(theme.text),
                            );
                        });
                    });

                    ui.add_space(16.0);

                    // Subject field with larger text
                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.add_space(8.0);
                            ui.label(
                                RichText::new("Subject")
                                    .font(FontId::new(13.0, FontFamily::Proportional))
                                    .color(theme.header_text)
                                    .strong(),
                            );
                            ui.add_space(16.0);
                            ui.label(
                                RichText::new(&email.subject)
                                    .font(FontId::new(16.0, FontFamily::Proportional))
                                    .color(theme.text)
                                    .strong(),
                            );
                        });
                    });
                });
            });

        ui.add_space(16.0);

        // Email content
        egui::Frame::new()
            .fill(theme.panel)
            .inner_margin(Margin::same(24))
            .stroke(Stroke::new(1.0, theme.border))
            .corner_radius(8.0)
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .max_height(400.0)
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new(&email.content)
                                .font(FontId::new(14.0, FontFamily::Proportional))
                                .color(theme.text),
                        );
                    });
            });

        ui.add_space(16.0);

        // Action buttons
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Reply button
                if render_button(ui, "Reply", true, theme, None).clicked() {
                    app.compose_draft = Some(create_reply_draft(email));
                    app.compose_dialog_open = true;
                    app.email_dialog_open = false;
                }

                ui.add_space(8.0);

                // Forward button
                if render_button(ui, "Forward", true, theme, None).clicked() {
                    app.compose_draft = Some(create_forward_draft(email));
                    app.compose_dialog_open = true;
                    app.email_dialog_open = false;
                }
            });
        });
    });
}
