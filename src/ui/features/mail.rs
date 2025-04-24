use eframe::egui;
use egui::{Color32, Margin, RichText};

use crate::app::CircleApp;
use crate::ui::components::button::render_button;
use crate::ui::components::mail::compose::{open_compose_screen, render_compose_screen};
use crate::ui::components::mail::email_card::{get_folder_icon, render_email_card};
use crate::ui::components::mail::email_details::render_email_detail;

/// Render the mail feature
pub fn render_mail(app: &mut CircleApp, ui: &mut egui::Ui) {
    let theme = app.get_current_theme();

    // Only show top bar buttons if we're not in compose mode
    if !app.compose_dialog_open {
        // Top bar
        ui.horizontal(|ui| {
            ui.add_space(12.0);
            let compose_response = render_button(ui, "Compose", true, &theme, Some("✏️"));
            if compose_response.clicked() {
                open_compose_screen(app, ui);
            }
            if compose_response.hovered() {
                ui.ctx().request_repaint();
                ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
            }

            ui.add_space(10.0);
            let icon = if app.is_refreshing { "⌛" } else { "🔄" };
            let refresh_response = render_button(ui, "Refresh", true, &theme, Some(icon));
            if refresh_response.clicked() {
                app.is_refreshing = true;
                ui.ctx().request_repaint(); // Force immediate repaint to show loading state
            }
            if refresh_response.hovered() {
                ui.ctx().request_repaint();
                ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
            }
            ui.add_space(10.0);
        });

        // Show loading message if refreshing
        if app.is_refreshing {
            ui.horizontal(|ui| {
                ui.add_space(16.0);
                ui.label(
                    RichText::new("Refreshing emails...")
                        .size(14.0)
                        .color(theme.secondary_text),
                );
            });
            // Simulate loading for demo purposes
            if ui.input(|i| i.stable_dt) > 0.5 {
                app.is_refreshing = false;
            }
        }

        // Main content
        ui.add_space(16.0);

        // If an email is selected, show its details
        if app.email_dialog_open {
            if let Some(email_id) = app.selected_email_id {
                // Extract the email data before we need to borrow app mutably
                let email_opt = app.active_feature_data.as_ref().and_then(|data| {
                    data.mail_data
                        .emails
                        .iter()
                        .find(|e| e.id == email_id)
                        .cloned() // Clone the email to avoid borrow issues
                });

                if let Some(email) = email_opt {
                    // Back button
                    if render_button(ui, "Back to emails", false, &theme, None).clicked() {
                        app.email_dialog_open = false;
                        app.selected_email_id = None;
                    }

                    ui.add_space(16.0);

                    // Render the email details with the cloned email
                    render_email_detail(app, ui, &email, &theme);
                }
            }
        } else {
            // Folder selection
            ui.horizontal(|ui| {
                ui.add_space(16.0);
                ui.label(
                    RichText::new("Folders")
                        .size(16.0)
                        .color(theme.header_text)
                        .strong(),
                );
            });

            ui.add_space(8.0);

            // Extract folder data before we need to borrow app mutably
            let folder_data = app
                .active_feature_data
                .as_ref()
                .map(|data| {
                    data.mail_data
                        .folders
                        .iter()
                        .map(|f| (f.id, f.name.clone()))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();

            // Folder list
            egui::Frame::new()
                .inner_margin(Margin::same(16))
                .show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        for folder in &["Inbox", "Sent", "Drafts", "Spam", "Trash"] {
                            let is_selected = app.active_mail_folder_id.is_some()
                                && folder_data.iter().any(|(id, name)| {
                                    *id == app.active_mail_folder_id.unwrap() && name == *folder
                                });

                            let folder_icon = get_folder_icon(folder);
                            let button_fill = if is_selected {
                                theme.accent
                            } else {
                                theme.secondary_background
                            };
                            let text_color = if is_selected {
                                Color32::WHITE
                            } else {
                                theme.text
                            };

                            let folder_label = RichText::new(format!("{} {}", folder_icon, folder))
                                .size(14.0)
                                .color(text_color);

                            if ui
                                .add(
                                    egui::Button::new(folder_label)
                                        .fill(button_fill)
                                        .corner_radius(4)
                                        .min_size(egui::Vec2::new(100.0, 32.0)),
                                )
                                .clicked()
                            {
                                // Find the folder ID by name
                                if let Some((folder_id, _)) =
                                    folder_data.iter().find(|(_, name)| name == folder)
                                {
                                    app.active_mail_folder_id = Some(*folder_id);
                                }
                            }
                            ui.add_space(8.0);
                        }
                    });
                });

            ui.add_space(16.0);

            // Email count
            let email_count = app
                .active_feature_data
                .as_ref()
                .map_or(0, |data| data.mail_data.emails.len());

            ui.horizontal(|ui| {
                ui.add_space(16.0);
                ui.label(
                    RichText::new(format!("{} emails", email_count))
                        .size(16.0)
                        .color(theme.header_text)
                        .strong(),
                );
            });

            ui.add_space(8.0);

            // Email cards - we need to collect emails first to avoid borrow issues
            let emails = app
                .active_feature_data
                .as_ref()
                .map(|data| data.mail_data.emails.clone())
                .unwrap_or_default();

            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    for (index, email) in emails.iter().enumerate() {
                        // We need to clone the email to avoid borrow issues
                        let email_clone = email.clone();
                        if render_email_card(ui, &email_clone, index, app, &theme) {
                            app.selected_email_id = Some(email.id);
                            app.email_dialog_open = true;
                        }
                        ui.add_space(8.0);
                    }
                });
        }
    } else {
        // Render the compose screen
        render_compose_screen(ui, app, &theme);
    }
}
