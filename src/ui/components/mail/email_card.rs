use crate::{app::CircleApp, models::dummy_data::Email, utils::config::Theme};
use chrono::{DateTime, Utc};
use egui::{FontFamily, FontId, Margin, RichText, Stroke, epaint};

#[allow(dead_code)]
/// Renders an email card, the email card is a card that contains the email subject and the email body used in the mail feature to list the emails
pub fn render_email_card(
    ui: &mut egui::Ui,
    email: &Email,
    index: usize,
    app: &mut CircleApp,
    theme: &Theme,
) -> bool {
    let mut clicked = false;

    let frame = egui::Frame::new()
        .fill(theme.panel)
        .stroke(Stroke::new(1.0, theme.border))
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
                    theme.accent,
                );
                ui.add_space(8.0);
            } else {
                ui.add_space(16.0);
            }

            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    let sender_text = RichText::new(format!("👤 {}", email.sender))
                        .font(FontId::new(13.0, FontFamily::Proportional))
                        .color(theme.text)
                        .strong();
                    ui.label(sender_text).on_hover_ui(|ui| {
                        ui.style_mut().visuals.popup_shadow = epaint::Shadow::NONE;
                        ui.style_mut().visuals.override_text_color = Some(theme.text);
                        ui.style_mut().visuals.window_fill = theme.panel;
                        ui.label(egui::RichText::new("Sender").color(theme.text));
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let time_text = RichText::new(format_time_ago(&email.timestamp))
                            .font(FontId::new(11.0, FontFamily::Proportional))
                            .color(theme.secondary_text);
                        ui.label(time_text).on_hover_ui(|ui| {
                            ui.style_mut().visuals.popup_shadow = epaint::Shadow::NONE;
                            ui.style_mut().visuals.override_text_color = Some(theme.text);
                            ui.style_mut().visuals.window_fill = theme.panel;
                            ui.label(egui::RichText::new("Sent time").color(theme.text));
                        });
                    });
                });

                let subject_color = if !email.read {
                    theme.text
                } else {
                    theme.secondary_text
                };
                let subject_text = RichText::new(&email.subject)
                    .font(FontId::new(12.0, FontFamily::Proportional))
                    .color(subject_color);
                ui.label(subject_text).on_hover_ui(|ui| {
                    ui.style_mut().visuals.popup_shadow = epaint::Shadow::NONE;
                    ui.style_mut().visuals.override_text_color = Some(theme.text);
                    ui.style_mut().visuals.window_fill = theme.panel;
                    ui.label(egui::RichText::new("Subject").color(theme.text));
                });

                let preview = if email.content.len() > 60 {
                    format!("{}...", &email.content[..60])
                } else {
                    email.content.clone()
                };
                let preview_text = RichText::new(preview)
                    .font(FontId::new(11.0, FontFamily::Proportional))
                    .color(theme.secondary_text)
                    .weak();
                ui.label(preview_text).on_hover_ui(|ui| {
                    ui.style_mut().visuals.popup_shadow = epaint::Shadow::NONE;
                    ui.style_mut().visuals.override_text_color = Some(theme.text);
                    ui.style_mut().visuals.window_fill = theme.panel;
                    ui.label(egui::RichText::new("Message preview").color(theme.text));
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
            // Use a semi-transparent hover color to ensure text remains visible
            let hover_color = theme.hover.gamma_multiply(0.5);
            ui.painter()
                .rect_filled(response.rect.expand(2.0), 6, hover_color);
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
