use crate::{models::dummy_data::Email, utils::config::Theme};
use egui::{Color32, FontFamily, FontId, Margin, RichText, Stroke, Vec2};

#[allow(dead_code)]
/// Render the email details
pub fn render_email_detail(ui: &mut egui::Ui, email: &Email, theme: &Theme) {
    ui.vertical(|ui| {
        egui::Frame::new()
            .fill(theme.secondary_background)
            .corner_radius(0.8)
            .inner_margin(Margin::same(12))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("From:")
                            .font(FontId::new(13.0, FontFamily::Proportional))
                            .color(theme.header_text)
                            .strong(),
                    );
                    ui.add_space(8.0);
                    ui.label(
                        RichText::new(&email.sender)
                            .font(FontId::new(13.0, FontFamily::Proportional))
                            .color(theme.text),
                    );
                });

                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("To:")
                            .font(FontId::new(13.0, FontFamily::Proportional))
                            .color(theme.header_text)
                            .strong(),
                    );
                    ui.add_space(8.0);
                    ui.label(
                        RichText::new(email.recipients.join(", "))
                            .font(FontId::new(13.0, FontFamily::Proportional))
                            .color(theme.text),
                    );
                });

                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("Date:")
                            .font(FontId::new(13.0, FontFamily::Proportional))
                            .color(theme.header_text)
                            .strong(),
                    );
                    ui.add_space(8.0);
                    let date_str = email.timestamp.format("%d %b %Y, %H:%M").to_string();
                    ui.label(
                        RichText::new(date_str)
                            .font(FontId::new(13.0, FontFamily::Proportional))
                            .color(theme.text),
                    );
                });

                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("Subject:")
                            .font(FontId::new(13.0, FontFamily::Proportional))
                            .color(theme.header_text)
                            .strong(),
                    );
                    ui.add_space(8.0);
                    ui.label(
                        RichText::new(&email.subject)
                            .font(FontId::new(16.0, FontFamily::Proportional))
                            .color(theme.text)
                            .strong(),
                    );
                });
            });

        ui.add_space(16.0);

        egui::Frame::new()
            .fill(Color32::from_rgb(255, 255, 255))
            .corner_radius(8)
            .inner_margin(Margin::same(16))
            .stroke(Stroke::new(1.0, Color32::from_rgb(220, 220, 230)))
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .max_height(300.0)
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new(&email.content)
                                .font(FontId::new(14.0, FontFamily::Proportional))
                                .color(theme.text),
                        );
                    });
            });

        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let reply_button = egui::Button::new(
                    RichText::new("Reply")
                        .font(FontId::new(14.0, FontFamily::Proportional))
                        .color(theme.background),
                )
                .fill(theme.accent)
                .corner_radius(4)
                .min_size(Vec2::new(80.0, 32.0));

                if ui.add(reply_button).clicked() {
                    println!("Reply to email: {}", email.subject);
                }

                let forward_button = egui::Button::new(
                    RichText::new("Forward")
                        .font(FontId::new(14.0, FontFamily::Proportional))
                        .color(theme.background),
                )
                .fill(theme.accent)
                .corner_radius(4)
                .min_size(Vec2::new(80.0, 32.0));

                if ui.add(forward_button).clicked() {
                    println!("Forward email: {}", email.subject);
                }
            });
        });
    });
}
