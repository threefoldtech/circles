use crate::{models::dummy_data::Email, utils::config::Theme};
use egui::{Color32, FontFamily, FontId, Margin, RichText, Stroke, Vec2};

#[allow(dead_code)]
/// Render the email details
pub fn render_email_detail(ui: &mut egui::Ui, email: &Email, theme: &Theme) {
    ui.vertical(|ui| {
        // Header section
        egui::Frame::none()
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
        egui::Frame::none()
            .fill(Color32::WHITE)
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
                let button_size = Vec2::new(100.0, 36.0);

                // Reply button
                let reply_button = egui::Button::new(
                    RichText::new("Reply")
                        .font(FontId::new(14.0, FontFamily::Proportional))
                        .color(Color32::WHITE),
                )
                .fill(theme.accent)
                .corner_radius(6.0)
                .min_size(button_size);

                if ui.add(reply_button).clicked() {
                    println!("Reply to email: {}", email.subject);
                }

                ui.add_space(8.0);

                // Forward button
                let forward_button = egui::Button::new(
                    RichText::new("Forward")
                        .font(FontId::new(14.0, FontFamily::Proportional))
                        .color(Color32::WHITE),
                )
                .fill(theme.accent)
                .corner_radius(6.0)
                .min_size(button_size);

                if ui.add(forward_button).clicked() {
                    println!("Forward email: {}", email.subject);
                }
            });
        });
    });
}