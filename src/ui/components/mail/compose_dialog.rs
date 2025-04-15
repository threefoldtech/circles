use chrono::Utc;
use egui::{Color32, FontFamily, FontId, Margin, RichText, Rounding, Stroke, Vec2};
use uuid::Uuid;

use crate::{
    app::CircleApp,
    models::{dummy_data::Email, features::ComposeDraft},
    utils::config::Theme,
};

/// Open the compose dialog
pub fn open_compose_dialog(app: &mut CircleApp, _: &mut egui::Ui) {
    app.compose_dialog_open = true;
    app.compose_draft = Some(ComposeDraft {
        to: String::new(),
        subject: String::new(),
        body: String::new(),
        attachments: Vec::new(),
    });
}

/// Render the compose dialog
/// Includes the compose form
pub fn render_compose_dialog(ui: &mut egui::Ui, app: &mut CircleApp, theme: &Theme) {
    ui.vertical(|ui| {
        // Back button
        let back_button = egui::Button::new(
            RichText::new("← Back to emails")
                .font(FontId::new(13.0, FontFamily::Proportional))
                .color(theme.accent),
        )
        .fill(theme.background)
        .rounding(Rounding::same(4.0));
        if ui.add(back_button).clicked() {
            app.compose_dialog_open = false;
            app.compose_draft = None;
        }
        ui.add_space(12.0);

        // Compose form
        egui::Frame::none()
            .fill(theme.secondary_background)
            .rounding(Rounding::same(8.0))
            .inner_margin(Margin::same(12.0))
            .show(ui, |ui| {
                if let Some(draft) = &mut app.compose_draft {
                    // To
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("To:")
                                .font(FontId::new(13.0, FontFamily::Proportional))
                                .color(theme.header_text)
                                .strong(),
                        );
                        ui.add_space(8.0);
                        ui.text_edit_singleline(&mut draft.to)
                            .on_hover_text("Enter recipient email addresses, separated by commas");
                    });

                    // Subject
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("Subject:")
                                .font(FontId::new(13.0, FontFamily::Proportional))
                                .color(theme.header_text)
                                .strong(),
                        );
                        ui.add_space(8.0);
                        ui.text_edit_singleline(&mut draft.subject)
                            .on_hover_text("Enter the email subject");
                    });
                }
            });

        ui.add_space(16.0);

        // Body
        egui::Frame::none()
            .fill(Color32::from_rgb(255, 255, 255))
            .rounding(Rounding::same(8.0))
            .inner_margin(Margin::same(16.0))
            .stroke(Stroke::new(1.0, Color32::from_rgb(220, 220, 230)))
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .max_height(300.0)
                    .show(ui, |ui| {
                        if let Some(draft) = &mut app.compose_draft {
                            ui.text_edit_multiline(&mut draft.body)
                                .on_hover_text("Enter the email body");
                        }
                    });
            });

        ui.add_space(16.0);

        // Attachments
        egui::Frame::none()
            .fill(theme.secondary_background)
            .inner_margin(Margin::same(12.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    let attach_button = egui::Button::new(
                        RichText::new("📎 Attach file")
                            .font(FontId::new(13.0, FontFamily::Proportional))
                            .color(theme.text),
                    )
                    .fill(theme.background)
                    .rounding(Rounding::same(4.0));
                    if ui.add(attach_button).clicked() {
                        if let Some(draft) = &mut app.compose_draft {
                            // Simulate file attachment (replace with actual file picker if available)
                            draft
                                .attachments
                                .push(format!("attachment_{}.txt", draft.attachments.len() + 1));
                        }
                    }

                    if let Some(draft) = &app.compose_draft {
                        for attachment in &draft.attachments {
                            ui.label(
                                RichText::new(attachment)
                                    .font(FontId::new(12.0, FontFamily::Proportional))
                                    .color(theme.secondary_text),
                            );
                            ui.add_space(8.0);
                        }
                    }
                });
            });

        ui.add_space(16.0);

        // Action buttons
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Send button
                let send_button = egui::Button::new(
                    RichText::new("Send")
                        .font(FontId::new(14.0, FontFamily::Proportional))
                        .color(theme.background),
                )
                .fill(theme.accent)
                .rounding(Rounding::same(4.0))
                .min_size(Vec2::new(80.0, 32.0));

                if ui.add(send_button).clicked() {
                    if let Some(draft) = &app.compose_draft {
                        if let Some(feature_data) = &mut app.active_feature_data {
                            let sent_folder_id = feature_data
                                .mail_data
                                .folders
                                .iter()
                                .find(|f| f.name.to_lowercase() == "sent")
                                .map(|f| f.id);

                            let new_email = Email {
                                id: Uuid::new_v4(),
                                sender: "user@example.com".to_string(), // Replace with actual user email
                                recipients: draft
                                    .to
                                    .split(',')
                                    .map(|s| s.trim().to_string())
                                    .filter(|s| !s.is_empty())
                                    .collect(),
                                subject: draft.subject.clone(),
                                content: draft.body.clone(),
                                timestamp: Utc::now(),
                                read: true,
                                folder_id: sent_folder_id
                                    .unwrap_or(feature_data.mail_data.folders[0].id),
                            };

                            feature_data.mail_data.emails.push(new_email);
                        }
                    }
                    app.compose_dialog_open = false;
                    app.compose_draft = None;
                }

                // Discard button
                let discard_button = egui::Button::new(
                    RichText::new("Discard")
                        .font(FontId::new(14.0, FontFamily::Proportional))
                        .color(theme.text),
                )
                .fill(theme.background)
                .rounding(Rounding::same(4.0))
                .min_size(Vec2::new(80.0, 32.0));

                if ui.add(discard_button).clicked() {
                    app.compose_dialog_open = false;
                    app.compose_draft = None;
                }
            });
        });
    });
}
