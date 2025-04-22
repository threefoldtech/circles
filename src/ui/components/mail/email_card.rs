use crate::{app::CircleApp, models::dummy_data::Email};
use chrono::{DateTime, Utc};
use egui::{Color32, FontFamily, FontId, Margin, RichText, Stroke, epaint};

#[allow(dead_code)]
/// Renders an email card, the email card is a card that contains the email subject and the email body used in the mail feature to list the emails
pub fn render_email_card(
    ui: &mut egui::Ui,
    email: &Email,
    index: usize,
    app: &mut CircleApp,
) -> bool {
    let mut clicked = false;

    let frame = egui::Frame::new()
        .fill(Color32::from_rgb(255, 255, 255))
        .stroke(Stroke::new(1.0, Color32::from_rgb(200, 200, 210)))
        .corner_radius(6)
        .inner_margin(Margin::same(10))
        .outer_margin(Margin::same(2));

    frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            if !email.read {
                ui.add_space(4.0);
                ui.painter().circle_filled(
                    ui.cursor().min + egui::vec2(4.0, 8.0),
                    3.0,
                    Color32::from_rgb(150, 200, 255),
                );
                ui.add_space(8.0);
            } else {
                ui.add_space(16.0);
            }

            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    let sender_text = RichText::new(format!("👤 {}", email.sender))
                        .font(FontId::new(13.0, FontFamily::Proportional))
                        .color(Color32::from_rgb(80, 80, 100))
                        .strong();
                    ui.label(sender_text).on_hover_ui(|ui| {
                        ui.style_mut().visuals.popup_shadow = epaint::Shadow::NONE;
                        ui.style_mut().visuals.override_text_color =
                            Some(Color32::from_rgb(255, 255, 255));
                        ui.style_mut().visuals.window_fill = Color32::from_rgb(255, 255, 255);
                        ui.label(
                            egui::RichText::new("Sender").color(Color32::from_rgb(255, 255, 255)),
                        );
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let time_text = RichText::new(format_time_ago(&email.timestamp))
                            .font(FontId::new(11.0, FontFamily::Proportional))
                            .color(Color32::from_rgb(120, 120, 140));
                        ui.label(time_text).on_hover_ui(|ui| {
                            ui.style_mut().visuals.popup_shadow = epaint::Shadow::NONE;
                            ui.style_mut().visuals.override_text_color =
                                Some(Color32::from_rgb(255, 255, 255));
                            ui.style_mut().visuals.window_fill = Color32::from_rgb(255, 255, 255);
                            ui.label(
                                egui::RichText::new("Sent time")
                                    .color(Color32::from_rgb(255, 255, 255)),
                            );
                        });
                    });
                });

                let subject_color = if !email.read {
                    Color32::from_rgb(60, 60, 80)
                } else {
                    Color32::from_rgb(80, 80, 100)
                };
                let subject_text = RichText::new(&email.subject)
                    .font(FontId::new(12.0, FontFamily::Proportional))
                    .color(subject_color);
                ui.label(subject_text).on_hover_ui(|ui| {
                    ui.style_mut().visuals.popup_shadow = epaint::Shadow::NONE;
                    ui.style_mut().visuals.override_text_color =
                        Some(Color32::from_rgb(255, 255, 255));
                    ui.style_mut().visuals.window_fill = Color32::from_rgb(255, 255, 255);
                    ui.label(
                        egui::RichText::new("Subject").color(Color32::from_rgb(255, 255, 255)),
                    );
                });

                let preview = if email.content.len() > 60 {
                    format!("{}...", &email.content[..60])
                } else {
                    email.content.clone()
                };
                let preview_text = RichText::new(preview)
                    .font(FontId::new(11.0, FontFamily::Proportional))
                    .color(Color32::from_rgb(120, 120, 140))
                    .weak();
                ui.label(preview_text).on_hover_ui(|ui| {
                    ui.style_mut().visuals.popup_shadow = epaint::Shadow::NONE;
                    ui.style_mut().visuals.override_text_color =
                        Some(Color32::from_rgb(255, 255, 255));
                    ui.style_mut().visuals.window_fill = Color32::from_rgb(255, 255, 255);
                    ui.label(
                        egui::RichText::new("Message preview")
                            .color(Color32::from_rgb(255, 255, 255)),
                    );
                });
            });
        });

        let response = ui.interact(
            ui.min_rect().expand2(egui::vec2(
                frame.inner_margin.left as f32,
                frame.inner_margin.right as f32,
            )),
            ui.id().with(email.id),
            egui::Sense::click(),
        );

        if response.clicked() {
            clicked = true;
            if let Some(feature_data) = &mut app.active_feature_data {
                let mut email = feature_data.mail_data.emails[index].clone();
                if !email.read {
                    email.read = true;
                    feature_data.mail_data.emails[index] = email.clone();
                    if let Some(folder) = feature_data
                        .mail_data
                        .folders
                        .iter_mut()
                        .find(|f| f.id == email.folder_id)
                    {
                        if folder.unread_count > 0 {
                            folder.unread_count -= 1;
                        }
                    }
                }
                app.selected_email_id = Some(email.id);
                app.email_dialog_open = true;
            }
        }

        if response.hovered() {
            ui.painter().rect_filled(
                response.rect.expand(2.0),
                6,
                Color32::from_rgb(200, 220, 255).gamma_multiply(0.2),
            );
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }
    });

    clicked
}

/// Format a timestamp for display
fn format_time_ago(timestamp: &DateTime<Utc>) -> String {
    let now = Utc::now();
    let duration = now.signed_duration_since(*timestamp);

    if duration.num_days() > 365 {
        format!("{} years ago", duration.num_days() / 365)
    } else if duration.num_days() > 30 {
        format!("{} months ago", duration.num_days() / 30)
    } else if duration.num_days() > 0 {
        format!("{} days ago", duration.num_days())
    } else if duration.num_hours() > 0 {
        format!("{}h ago", duration.num_hours())
    } else if duration.num_minutes() > 0 {
        format!("{}m ago", duration.num_minutes())
    } else {
        "Just now".to_string()
    }
}

#[allow(dead_code)]
/// Get the folder icon based on the folder name
pub fn get_folder_icon(folder_name: &str) -> &'static str {
    match folder_name.to_lowercase().as_str() {
        "inbox" => "📥",
        "sent" => "📤",
        "drafts" => "📝",
        "trash" => "🗑️",
        _ => "📁",
    }
}
