use chrono::{DateTime, Utc};
use eframe::egui;
use egui::RichText;

use crate::app::CircleApp;
use crate::models::dummy_data::Email;
use crate::ui::app_layout::{create_action_button, create_content_frame, render_header};

pub fn render_mail(app: &CircleApp, ui: &mut egui::Ui) {
    render_header(ui, "📧", "Mail");

    // Get the active circle name
    let circle_name = app
        .active_circle()
        .map_or("No Circle".to_string(), |c| c.name.clone());

    ui.horizontal(|ui| {
        ui.add_space(8.0);
        if ui.add(create_action_button("Compose", "✏️")).clicked() {
            // TODO: Implement new message
        }
        ui.add_space(8.0);
        if ui.add(create_action_button("Refresh", "🔄")).clicked() {
            // TODO: Implement refresh
        }
        ui.add_space(8.0);

        // Show the active circle name
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                RichText::new(format!("Circle: {}", circle_name))
                    .size(14.0)
                    .strong(),
            );
        });
    });

    ui.add_space(16.0);

    // Check if we have active feature data
    if let Some(feature_data) = &app.active_feature_data {
        create_content_frame().show(ui, |ui| {
            ui.horizontal(|ui| {
                // Render folders
                ui.vertical(|ui| {
                    ui.set_width(200.0);
                    ui.strong("Folders");
                    ui.separator();

                    for folder in &feature_data.mail_data.folders {
                        let folder_text = if folder.unread_count > 0 {
                            format!(
                                "{} ({}) ",
                                get_folder_icon(&folder.name),
                                folder.unread_count
                            )
                        } else {
                            format!("{} ", get_folder_icon(&folder.name))
                        } + &folder.name;

                        let _ = ui.selectable_label(false, folder_text);
                    }
                });

                ui.separator();

                // Render emails
                ui.vertical(|ui| {
                    ui.strong("Messages");
                    ui.separator();

                    if feature_data.mail_data.emails.is_empty() {
                        ui.label("No messages");
                    } else {
                        for email in &feature_data.mail_data.emails {
                            render_email(ui, email);
                            ui.add_space(4.0);
                        }
                    }
                });
            });
        });
    } else {
        // No active feature data
        create_content_frame().show(ui, |ui| {
            ui.centered_and_justified(|ui| {
                ui.label(
                    RichText::new("No circle selected or data not available")
                        .size(16.0)
                        .color(egui::Color32::from_rgb(100, 100, 100)),
                );
            });
        });
    }
}

fn render_email(ui: &mut egui::Ui, email: &Email) {
    egui::Frame::none()
        .fill(egui::Color32::from_rgb(248, 249, 250))
        .rounding(egui::Rounding::same(4.0))
        .inner_margin(egui::Margin::same(8.0))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                // Show sender
                ui.label(RichText::new(format!("👤 {}", email.sender)).strong());

                // Show time on the right
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format_time_ago(&email.timestamp));
                });
            });

            // Show subject with bold if unread
            if !email.read {
                ui.label(RichText::new(&email.subject).strong());
            } else {
                ui.label(&email.subject);
            }

            // Show preview of content
            let preview = if email.content.len() > 50 {
                format!("{}...", &email.content[..50])
            } else {
                email.content.clone()
            };
            ui.label(RichText::new(preview).weak());
        });
}

fn format_time_ago(timestamp: &DateTime<Utc>) -> String {
    let now = Utc::now();
    let duration = now.signed_duration_since(*timestamp);

    if duration.num_days() > 0 {
        format!("{} days ago", duration.num_days())
    } else if duration.num_hours() > 0 {
        format!("{} hours ago", duration.num_hours())
    } else if duration.num_minutes() > 0 {
        format!("{} minutes ago", duration.num_minutes())
    } else {
        "Just now".to_string()
    }
}

fn get_folder_icon(folder_name: &str) -> &'static str {
    match folder_name {
        "Inbox" => "📥",
        "Sent" => "📤",
        "Drafts" => "📝",
        "Trash" => "🗑️",
        _ => "📁",
    }
}
