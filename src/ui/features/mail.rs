use crate::app::CircleApp;
use crate::models::dummy_data::Email;
use crate::ui::app_layout::{create_action_button, create_content_frame};
use crate::utils::config::Theme;
use chrono::{DateTime, Utc};
use eframe::{egui, epaint};
use egui::{Color32, FontFamily, FontId, Margin, RichText, Rounding, Sense, Stroke, Vec2};

pub fn render_mail(app: &mut CircleApp, ui: &mut egui::Ui) {
    let theme = Theme::new();
    // Top bar
    ui.horizontal(|ui| {
        ui.add_space(12.0);
        let compose_button = create_action_button("Compose", "✏️")
            .fill(theme.accent) // Soft blue
            .rounding(Rounding::same(6.0));
        if ui.add(compose_button).clicked() {
            println!("Compose clicked"); // TODO: Implement
        }
        ui.add_space(10.0);
        let refresh_button = create_action_button("Refresh", "🔄")
            .fill(theme.accent)
            .rounding(Rounding::same(6.0));
        if ui.add(refresh_button).clicked() {
            println!("Refresh clicked"); // TODO: Implement
        }
        ui.add_space(10.0);

        // Active circle name (right-aligned)
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let circle_name = app
                .active_circle()
                .map_or("No Circle".to_string(), |c| c.name.clone());
            let circle_label = RichText::new(format!("Circle: {}", circle_name))
                .font(FontId::new(13.0, FontFamily::Proportional))
                .color(Color32::from_rgb(150, 200, 255)) // Soft sky blue
                .strong();
            ui.label(circle_label).on_hover_ui(|ui| {
                ui.style_mut().visuals.popup_shadow = epaint::Shadow::NONE;
                ui.style_mut().visuals.override_text_color = Some(Color32::from_rgb(60, 60, 80)); // Dark navy
                ui.style_mut().visuals.window_fill = Color32::from_rgb(255, 255, 255); // White background
                ui.label("Active circle");
            });
        });
    });

    ui.add_space(12.0);

    // Main content
    if let Some(feature_data) = &app.active_feature_data {
        // Extract the data we need before entering the closure
        let folders = &feature_data.mail_data.folders;
        let emails = &feature_data.mail_data.emails;

        create_content_frame()
            .fill(Color32::from_rgb(245, 245, 250)) // Soft cream
            .rounding(Rounding::same(8.0))
            .inner_margin(Margin::same(12.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    // Folders (left sidebar)
                    ui.vertical(|ui| {
                        ui.set_width(220.0);
                        ui.label(
                            RichText::new("Folders")
                                .font(FontId::new(14.0, FontFamily::Proportional))
                                .color(Color32::from_rgb(100, 100, 120)) // Dark silver
                                .strong(),
                        );
                        ui.add_space(4.0);
                        ui.separator();
                        ui.add_space(4.0);

                        for folder in folders {
                            let is_active = app.active_mail_folder_id == Some(folder.id);

                            let folder_text = if folder.unread_count > 0 {
                                format!(
                                    "{} {} ({})",
                                    get_folder_icon(&folder.name),
                                    folder.name,
                                    folder.unread_count
                                )
                            } else {
                                format!("{} {}", get_folder_icon(&folder.name), folder.name)
                            };

                            // Use different colors for active folder
                            let button_fill = if is_active {
                                Color32::from_rgb(100, 150, 255) // Darker blue for active folder
                            } else {
                                theme.accent
                            };

                            let folder_label = RichText::new(folder_text.clone())
                                .font(FontId::new(12.0, FontFamily::Proportional))
                                .color(theme.background); // White text

                            let button = ui
                                .add(
                                    egui::Button::new(folder_label)
                                        .fill(button_fill)
                                        .rounding(Rounding::same(4.0))
                                        .min_size(Vec2::new(32.0, 32.0))
                                        .min_size(egui::vec2(ui.available_width(), 24.0))
                                        .sense(Sense::click()),
                                )
                                .on_hover_ui(|ui| {
                                    ui.style_mut().visuals.popup_shadow = epaint::Shadow::NONE;
                                    ui.style_mut().visuals.override_text_color =
                                        Some(Color32::from_rgb(60, 60, 80));
                                    ui.style_mut().visuals.window_fill =
                                        Color32::from_rgb(255, 255, 255);
                                    ui.label(
                                        egui::RichText::new(format!("View {}", folder.name))
                                            .color(Color32::WHITE),
                                    );
                                })
                                .on_hover_cursor(egui::CursorIcon::PointingHand);

                            if button.hovered() {
                                ui.painter().rect_filled(
                                    button.rect,
                                    Rounding::same(4.0),
                                    Color32::from_rgb(150, 200, 255), // Sky blue on hover
                                );
                                // Repaint with white text on hover
                                ui.painter().text(
                                    button.rect.center(),
                                    egui::Align2::CENTER_CENTER,
                                    folder_text,
                                    FontId::new(12.0, FontFamily::Proportional),
                                    Color32::from_rgb(255, 255, 255), // White text
                                );
                            }

                            if button.clicked() {
                                println!("Folder clicked: {}", folder.name);
                                app.active_mail_folder_id = Some(folder.id);
                            }
                        }
                    });

                    ui.add_space(12.0);

                    // Emails (main area)
                    ui.vertical(|ui| {
                        ui.set_min_width(400.0);

                        // Get the active folder name for the header
                        let active_folder_name = app
                            .active_mail_folder_id
                            .and_then(|id| folders.iter().find(|f| f.id == id))
                            .map_or("Messages".to_string(), |f| format!("{} Messages", f.name));

                        ui.label(
                            RichText::new(active_folder_name)
                                .font(FontId::new(14.0, FontFamily::Proportional))
                                .color(Color32::from_rgb(100, 100, 120))
                                .strong(),
                        );
                        ui.add_space(4.0);
                        ui.separator();

                        // Filter emails by the active folder
                        let filtered_emails: Vec<&Email> =
                            if let Some(folder_id) = app.active_mail_folder_id {
                                emails
                                    .iter()
                                    .filter(|email| email.folder_id == folder_id)
                                    .collect()
                            } else {
                                emails.iter().collect()
                            };

                        if filtered_emails.is_empty() {
                            ui.centered_and_justified(|ui| {
                                ui.label(
                                    RichText::new("No messages in this folder")
                                        .font(FontId::new(13.0, FontFamily::Proportional))
                                        .color(Color32::from_rgb(120, 120, 140)),
                                );
                            });
                        } else {
                            for email in filtered_emails {
                                // Pass a reference to avoid borrowing conflicts
                                render_email_ui(ui, email);
                                ui.add_space(8.0);
                            }
                        }
                    });
                });
            });
    } else {
        create_content_frame()
            .fill(Color32::from_rgb(245, 245, 250))
            .rounding(Rounding::same(8.0))
            .inner_margin(Margin::same(12.0))
            .show(ui, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.label(
                        RichText::new("No circle selected or data not available")
                            .font(FontId::new(15.0, FontFamily::Proportional))
                            .color(Color32::from_rgb(120, 120, 140))
                            .italics(),
                    );
                });
            });
    }
}

// Renamed function that doesn't require mutable app reference
fn render_email_ui(ui: &mut egui::Ui, email: &Email) {
    egui::Frame::none()
        .fill(Color32::from_rgb(255, 255, 255)) // White card
        .stroke(Stroke::new(1.0, Color32::from_rgb(200, 200, 210))) // Light gray border
        .rounding(Rounding::same(6.0))
        .inner_margin(Margin::same(10.0))
        .outer_margin(Margin::same(2.0))
        .show(ui, |ui| {
            let response = ui.interact(ui.min_rect(), ui.id().with(email.id), egui::Sense::click());
            if response.hovered() {
                ui.painter().rect_filled(
                    response.rect.expand(2.0),
                    Rounding::same(6.0),
                    Color32::from_rgb(200, 220, 255).gamma_multiply(0.2),
                );
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }
            if response.clicked() {
                // Placeholder for opening email in new window
                println!("Opening email in new window: {}", email.subject); // TODO: Implement window
            }

            ui.horizontal(|ui| {
                // Unread indicator (dot for unread emails)
                if !email.read {
                    ui.add_space(4.0);
                    ui.painter().circle_filled(
                        ui.cursor().min + egui::vec2(4.0, 8.0),
                        3.0,
                        Color32::from_rgb(150, 200, 255), // Sky blue dot
                    );
                    ui.add_space(8.0);
                } else {
                    ui.add_space(16.0); // Align read emails
                }

                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        // Sender
                        let sender_text = RichText::new(format!("👤 {}", email.sender))
                            .font(FontId::new(13.0, FontFamily::Proportional))
                            .color(Color32::from_rgb(80, 80, 100)) // Muted navy
                            .strong();
                        ui.label(sender_text).on_hover_ui(|ui| {
                            ui.style_mut().visuals.popup_shadow = epaint::Shadow::NONE;
                            ui.style_mut().visuals.override_text_color =
                                Some(Color32::from_rgb(60, 60, 80));
                            ui.style_mut().visuals.window_fill = Color32::from_rgb(255, 255, 255);
                            ui.label(egui::RichText::new(format!("Sender")).color(Color32::WHITE));
                        });

                        // Time (right-aligned)
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let time_text = RichText::new(format_time_ago(&email.timestamp))
                                .font(FontId::new(11.0, FontFamily::Proportional))
                                .color(Color32::from_rgb(120, 120, 140)); // Muted gray
                            ui.label(time_text).on_hover_ui(|ui| {
                                ui.style_mut().visuals.popup_shadow = epaint::Shadow::NONE;
                                ui.style_mut().visuals.override_text_color =
                                    Some(Color32::from_rgb(60, 60, 80));
                                ui.style_mut().visuals.window_fill =
                                    Color32::from_rgb(255, 255, 255);
                                ui.label(
                                    egui::RichText::new(format!("Sent at")).color(Color32::WHITE),
                                );
                            });
                        });
                    });

                    // Subject
                    let subject_color = if !email.read {
                        Color32::from_rgb(60, 60, 80) // Darker navy for unread
                    } else {
                        Color32::from_rgb(80, 80, 100) // Muted navy for read
                    };
                    let subject_text = RichText::new(&email.subject)
                        .font(FontId::new(12.0, FontFamily::Proportional))
                        .color(subject_color);
                    ui.label(subject_text).on_hover_ui(|ui| {
                        ui.style_mut().visuals.popup_shadow = epaint::Shadow::NONE;
                        ui.style_mut().visuals.override_text_color =
                            Some(Color32::from_rgb(60, 60, 80));
                        ui.style_mut().visuals.window_fill = Color32::from_rgb(255, 255, 255);
                        ui.label(egui::RichText::new(format!("Subject")).color(Color32::WHITE));
                    });

                    // Preview
                    let preview = if email.content.len() > 60 {
                        format!("{}...", &email.content[..60])
                    } else {
                        email.content.clone()
                    };
                    let preview_text = RichText::new(preview)
                        .font(FontId::new(11.0, FontFamily::Proportional))
                        .color(Color32::from_rgb(120, 120, 140)) // Muted gray
                        .weak();
                    ui.label(preview_text).on_hover_ui(|ui| {
                        ui.style_mut().visuals.popup_shadow = epaint::Shadow::NONE;
                        ui.style_mut().visuals.override_text_color =
                            Some(Color32::from_rgb(60, 60, 80));
                        ui.style_mut().visuals.window_fill = Color32::from_rgb(255, 255, 255);
                        ui.label(
                            egui::RichText::new(format!("Message Preview")).color(Color32::WHITE),
                        );
                    });
                });
            });
        });
}

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

fn get_folder_icon(folder_name: &str) -> &'static str {
    match folder_name.to_lowercase().as_str() {
        "inbox" => "📥",
        "sent" => "📤",
        "drafts" => "📝",
        "trash" => "🗑️",
        _ => "📁",
    }
}
