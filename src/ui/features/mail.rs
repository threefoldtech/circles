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
            .fill(theme.accent)
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
    });

    ui.add_space(12.0);

    // Main content
    if let Some(feature_data) = &mut app.active_feature_data.clone() {
        create_content_frame()
            .fill(theme.secondary_background) // Soft cream
            .rounding(Rounding::same(8.0))
            .inner_margin(Margin::same(12.0))
            .show(ui, |ui| {
                // If an email is selected, show the email detail view
                if app.email_dialog_open {
                    if let Some(email_id) = app.selected_email_id {
                        if let Some(email) = feature_data
                            .mail_data
                            .emails
                            .iter()
                            .find(|e| e.id == email_id)
                        {
                            // Back button
                            let back_button = egui::Button::new(
                                RichText::new("← Back to emails")
                                    .font(FontId::new(13.0, FontFamily::Proportional))
                                    .color(theme.accent),
                            )
                            .fill(theme.background)
                            .rounding(Rounding::same(4.0));
                            if ui.add(back_button).clicked() {
                                app.email_dialog_open = false;
                                app.selected_email_id = None;
                                println!("Back to emails");
                            }
                            ui.add_space(12.0);

                            // Email details
                            render_email_detail(ui, email, &theme);
                        } else {
                            // Handle case where email_id doesn't match
                            ui.label(
                                RichText::new("Error: Email not found")
                                    .font(FontId::new(13.0, FontFamily::Proportional))
                                    .color(Color32::from_rgb(200, 100, 100)),
                            );
                            app.email_dialog_open = false;
                            app.selected_email_id = None;
                        }
                    } else {
                        // Handle invalid state
                        app.email_dialog_open = false;
                    }
                } else {
                    // Show the folder and email list view
                    ui.horizontal(|ui| {
                        // Folders (left sidebar)
                        ui.vertical(|ui| {
                            ui.set_width(220.0);
                            ui.label(
                                RichText::new("Folders")
                                    .font(FontId::new(14.0, FontFamily::Proportional))
                                    .color(theme.header_text)
                                    .strong(),
                            );
                            ui.add_space(4.0);
                            ui.separator();
                            ui.add_space(4.0);

                            for folder in &feature_data.mail_data.folders {
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
                                    theme.accent // Sky blue for active
                                } else {
                                    theme.background // White for inactive
                                };

                                let text_color = if is_active {
                                    theme.background // White for active
                                } else {
                                    theme.text // Muted navy
                                };

                                let folder_label = RichText::new(folder_text.clone())
                                    .font(FontId::new(12.0, FontFamily::Proportional))
                                    .color(text_color);

                                let button = ui
                                    .add(
                                        egui::Button::new(folder_label)
                                            .fill(button_fill)
                                            .rounding(Rounding::same(4.0))
                                            .min_size(egui::vec2(ui.available_width(), 24.0))
                                            .sense(Sense::click()),
                                    )
                                    .on_hover_ui(|ui| {
                                        ui.style_mut().visuals.popup_shadow = epaint::Shadow::NONE;
                                        ui.style_mut().visuals.override_text_color =
                                            Some(Color32::from_rgb(255, 255, 255));
                                        ui.style_mut().visuals.window_fill =
                                            Color32::from_rgb(255, 255, 255);
                                        ui.label(
                                            egui::RichText::new(format!("View {}", folder.name))
                                                .color(Color32::from_rgb(255, 255, 255)),
                                        );
                                    })
                                    .on_hover_cursor(egui::CursorIcon::PointingHand);

                                if button.hovered() {
                                    ui.painter().rect_filled(
                                        button.rect,
                                        Rounding::same(4.0),
                                        theme.accent,
                                    );
                                    ui.painter().text(
                                        button.rect.center(),
                                        egui::Align2::CENTER_CENTER,
                                        folder_text,
                                        FontId::new(12.0, FontFamily::Proportional),
                                        Color32::from_rgb(255, 255, 255),
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
                                .and_then(|id| {
                                    feature_data.mail_data.folders.iter().find(|f| f.id == id)
                                })
                                .map_or("Messages".to_string(), |f| format!("{} Messages", f.name));

                            ui.label(
                                RichText::new(active_folder_name)
                                    .font(FontId::new(14.0, FontFamily::Proportional))
                                    .color(theme.header_text)
                                    .strong(),
                            );
                            ui.add_space(4.0);
                            ui.separator();

                            // Filter emails by the active folder
                            let filtered_emails: Vec<(usize, &Email)> = feature_data
                                .mail_data
                                .emails
                                .iter()
                                .enumerate()
                                .filter(|(_, email)| {
                                    app.active_mail_folder_id
                                        .map_or(true, |id| email.folder_id == id)
                                })
                                .collect();

                            if filtered_emails.is_empty() {
                                ui.centered_and_justified(|ui| {
                                    ui.label(
                                        RichText::new("No messages in this folder")
                                            .font(FontId::new(13.0, FontFamily::Proportional))
                                            .color(theme.secondary_text),
                                    );
                                });
                            } else {
                                for (index, email) in filtered_emails {
                                    let clicked = render_email_item(ui, email, index, app);
                                    if clicked {
                                        println!(
                                            "Email clicked: {} (ID: {})",
                                            email.subject, email.id
                                        );
                                    }
                                    ui.add_space(8.0);
                                }
                            }
                        });
                    });
                }
            });
    } else {
        create_content_frame()
            .fill(theme.secondary_background)
            .rounding(Rounding::same(8.0))
            .inner_margin(Margin::same(12.0))
            .show(ui, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.label(
                        RichText::new("No circle selected or data not available")
                            .font(FontId::new(15.0, FontFamily::Proportional))
                            .color(theme.secondary_text)
                            .italics(),
                    );
                });
            });
    }
}

fn render_email_item(ui: &mut egui::Ui, email: &Email, index: usize, app: &mut CircleApp) -> bool {
    let mut clicked = false;

    let frame = egui::Frame::none()
        .fill(Color32::from_rgb(255, 255, 255))
        .stroke(Stroke::new(1.0, Color32::from_rgb(200, 200, 210)))
        .rounding(Rounding::same(6.0))
        .inner_margin(Margin::same(10.0))
        .outer_margin(Margin::same(2.0));

    frame.show(ui, |ui| {
        // Render the card content first
        ui.horizontal(|ui| {
            // Unread indicator
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
                // Sender and time
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

                // Subject
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

                // Preview
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

        // Now interact with the entire content rectangle
        let response = ui.interact(
            ui.min_rect().expand2(egui::vec2(
                frame.inner_margin.left,
                frame.inner_margin.right,
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
                Rounding::same(6.0),
                Color32::from_rgb(200, 220, 255).gamma_multiply(0.2),
            );
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }
    });

    clicked
}

fn render_email_detail(ui: &mut egui::Ui, email: &Email, theme: &Theme) {
    ui.vertical(|ui| {
        // Email header section
        egui::Frame::none()
            .fill(theme.secondary_background)
            .rounding(Rounding::same(8.0))
            .inner_margin(Margin::same(12.0))
            .show(ui, |ui| {
                // From
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

                // To
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

                // Date
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
                    ui.label(
                        RichText::new(&email.subject)
                            .font(FontId::new(16.0, FontFamily::Proportional))
                            .color(theme.text)
                            .strong(),
                    );
                });
            });

        ui.add_space(16.0);

        // Email content section
        egui::Frame::none()
            .fill(Color32::from_rgb(255, 255, 255))
            .rounding(Rounding::same(8.0))
            .inner_margin(Margin::same(16.0))
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

        // Action buttons
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Reply button
                let reply_button = egui::Button::new(
                    RichText::new("Reply")
                        .font(FontId::new(14.0, FontFamily::Proportional))
                        .color(theme.background),
                )
                .fill(theme.accent)
                .rounding(Rounding::same(4.0))
                .min_size(Vec2::new(80.0, 32.0));

                if ui.add(reply_button).clicked() {
                    println!("Reply to email: {}", email.subject);
                }

                // Forward button
                let forward_button = egui::Button::new(
                    RichText::new("Forward")
                        .font(FontId::new(14.0, FontFamily::Proportional))
                        .color(theme.background),
                )
                .fill(theme.accent)
                .rounding(Rounding::same(4.0))
                .min_size(Vec2::new(80.0, 32.0));

                if ui.add(forward_button).clicked() {
                    println!("Forward email: {}", email.subject);
                }
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
