use egui::{Color32, FontFamily, FontId, Margin, RichText, Stroke, Vec2};

use crate::{app::CircleApp, models::features::ComposeDraft, utils::config::Theme};

/// Open the compose screen
pub fn open_compose_screen(app: &mut CircleApp, _: &mut egui::Ui) {
    app.compose_dialog_open = true;
    app.compose_draft = Some(ComposeDraft {
        to: String::new(),
        subject: String::new(),
        body: String::new(),
        attachments: Vec::new(),
    });
}

/// Render the compose screen
pub fn render_compose_screen(ui: &mut egui::Ui, app: &mut CircleApp, theme: &Theme) {
    ui.vertical(|ui| {
        // Header
        ui.horizontal(|ui| {
            ui.heading(
                RichText::new("Draft a new email")
                    .size(20.0)
                    .color(theme.header_text)
                    .strong(),
            );
        });
        ui.add_space(20.0);

        if let Some(draft) = &mut app.compose_draft {
            // To field
            ui.horizontal(|ui| {
                ui.label(RichText::new("To:").size(14.0).color(theme.text).strong());

                let input_frame = egui::Frame::new()
                    .fill(theme.hover)
                    .corner_radius(20.0)
                    .inner_margin(Margin::same(10))
                    .stroke(Stroke::NONE);

                input_frame.show(ui, |ui| {
                    let original_style = ui.style().clone();
                    ui.style_mut().visuals.widgets.inactive.bg_fill = Color32::TRANSPARENT;
                    ui.style_mut().visuals.widgets.active.bg_fill = Color32::TRANSPARENT;
                    ui.style_mut().visuals.widgets.hovered.bg_fill = Color32::TRANSPARENT;

                    ui.add(
                        egui::TextEdit::singleline(&mut draft.to)
                            .desired_width(f32::INFINITY)
                            .font(FontId::new(14.0, FontFamily::Proportional))
                            .frame(false)
                            .margin(Vec2::new(0.0, 0.0)),
                    );

                    ui.set_style(original_style);
                });
            });
            ui.add_space(12.0);

            // Subject field
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("Subject:")
                        .size(14.0)
                        .color(theme.text)
                        .strong(),
                );

                let input_frame = egui::Frame::new()
                    .fill(theme.hover)
                    .corner_radius(20.0)
                    .inner_margin(Margin::same(10))
                    .stroke(Stroke::NONE);

                input_frame.show(ui, |ui| {
                    let original_style = ui.style().clone();
                    ui.style_mut().visuals.widgets.inactive.bg_fill = Color32::TRANSPARENT;
                    ui.style_mut().visuals.widgets.active.bg_fill = Color32::TRANSPARENT;
                    ui.style_mut().visuals.widgets.hovered.bg_fill = Color32::TRANSPARENT;

                    ui.add(
                        egui::TextEdit::singleline(&mut draft.subject)
                            .desired_width(f32::INFINITY)
                            .font(FontId::new(14.0, FontFamily::Proportional))
                            .frame(false)
                            .margin(Vec2::new(0.0, 0.0)),
                    );

                    ui.set_style(original_style);
                });
            });
            ui.add_space(12.0);

            // Body field
            ui.horizontal(|ui| {
                ui.label(RichText::new("Body:").size(14.0).color(theme.text).strong());

                let body_frame = egui::Frame::new()
                    .fill(theme.hover)
                    .corner_radius(20.0)
                    .inner_margin(Margin::same(10))
                    .stroke(Stroke::NONE);

                body_frame.show(ui, |ui| {
                    let original_style = ui.style().clone();
                    ui.style_mut().visuals.widgets.inactive.bg_fill = Color32::TRANSPARENT;
                    ui.style_mut().visuals.widgets.active.bg_fill = Color32::TRANSPARENT;
                    ui.style_mut().visuals.widgets.hovered.bg_fill = Color32::TRANSPARENT;

                    ui.add(
                        egui::TextEdit::multiline(&mut draft.body)
                            .desired_width(f32::INFINITY)
                            .desired_rows(15)
                            .font(FontId::new(14.0, FontFamily::Proportional))
                            .frame(false)
                            .margin(Vec2::new(0.0, 0.0)),
                    );

                    ui.set_style(original_style);
                });
            });

            ui.add_space(20.0);

            // Bottom buttons
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    if ui
                        .add(
                            egui::Button::new(
                                RichText::new("📎 Attach").size(14.0).color(theme.accent),
                            )
                            .fill(theme.background)
                            .stroke(Stroke::new(1.0, theme.accent))
                            .min_size(Vec2::new(100.0, 36.0))
                            .corner_radius(6.0),
                        )
                        .clicked()
                    {
                        // TODO: Implement attachment functionality
                    }
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .add(
                            egui::Button::new(
                                RichText::new("Send").size(14.0).color(Color32::WHITE),
                            )
                            .fill(theme.accent)
                            .min_size(Vec2::new(100.0, 36.0))
                            .corner_radius(6.0),
                        )
                        .clicked()
                    {
                        // TODO: Implement send functionality
                        app.compose_dialog_open = false;
                        app.compose_draft = None;
                    }

                    if ui
                        .add(
                            egui::Button::new(
                                RichText::new("Discard")
                                    .size(14.0)
                                    .color(Color32::from_rgb(200, 50, 50)),
                            )
                            .fill(theme.background)
                            .stroke(Stroke::new(1.0, Color32::from_rgb(200, 50, 50)))
                            .min_size(Vec2::new(100.0, 36.0))
                            .corner_radius(6.0),
                        )
                        .clicked()
                    {
                        app.compose_dialog_open = false;
                        app.compose_draft = None;
                    }
                    ui.add_space(8.0);
                });
            });
        }
    });
}
