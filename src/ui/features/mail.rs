use eframe::{egui, epaint};
use egui::{Color32, FontFamily, FontId, Margin, RichText, Sense};

use crate::app::CircleApp;
use crate::models::dummy_data::Email;
use crate::ui::app_layout::create_content_frame;
use crate::ui::components::button;
use crate::ui::components::mail::compose::{open_compose_screen, render_compose_screen};
use crate::ui::components::mail::email_card::{get_folder_icon, render_email_card};
use crate::ui::components::mail::email_details::render_email_detail;

use crate::utils::config::Theme;

pub fn render_mail(app: &mut CircleApp, ui: &mut egui::Ui) {
    let theme = app.get_current_theme();

    // Only show top bar buttons if we're not in compose mode
    if !app.compose_dialog_open {
        // Top bar
        ui.horizontal(|ui| {
            ui.add_space(12.0);
            let compose_button = button::create_button("Compose", "✏️")
                .fill(theme.accent)
                .corner_radius(6);
            let compose_response = ui.add(compose_button);
            if compose_response.clicked() {
                open_compose_screen(app, ui);
            }
            if compose_response.hovered() {
                ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
            }

            ui.add_space(10.0);
            let refresh_button =
                button::create_button("Refresh", if app.is_refreshing { "⌛" } else { "🔄" })
                    .fill(theme.accent)
                    .corner_radius(6);
            let refresh_response = ui.add(refresh_button);
            if refresh_response.clicked() {
                app.is_refreshing = true;
                ui.ctx().request_repaint(); // Force immediate repaint to show loading state
            }
            if refresh_response.hovered() {
                ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
            }
            ui.add_space(10.0);
        });

        // Show loading message if refreshing
        if app.is_refreshing {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.add_space(12.0);
                let loading_text = RichText::new("⌛ Refreshing mail data...")
                    .color(theme.accent)
                    .size(14.0);
                ui.label(loading_text);
            });
            ui.add_space(8.0);

            if let Some(feature_data) = &mut app.active_feature_data {
                // Reset unread counts for all folders
                for folder in &mut feature_data.mail_data.folders {
                    folder.unread_count = feature_data
                        .mail_data
                        .emails
                        .iter()
                        .filter(|email| email.folder_id == folder.id && !email.read)
                        .count();
                }

                // If we're in email detail view, refresh the current email
                if app.email_dialog_open {
                    if let Some(email_id) = app.selected_email_id {
                        if feature_data
                            .mail_data
                            .emails
                            .iter()
                            .find(|e| e.id == email_id)
                            .is_none()
                        {
                            // If email no longer exists, close the detail view
                            app.email_dialog_open = false;
                            app.selected_email_id = None;
                        }
                    }
                }
            }

            // Simulate a brief loading period and reset the state
            ui.ctx()
                .request_repaint_after(std::time::Duration::from_millis(500));
            app.is_refreshing = false;
        }

        ui.add_space(12.0);
    }

    // Main content
    if let Some(feature_data) = &mut app.active_feature_data.clone() {
        create_content_frame(&theme)
            .fill(theme.secondary_background)
            .corner_radius(8.0)
            .inner_margin(Margin::same(12))
            .show(ui, |ui| {
                // Compose screen
                if app.compose_dialog_open {
                    render_compose_screen(ui, app, &theme);
                }
                // Email detail view
                else if app.email_dialog_open {
                    if let Some(email_id) = app.selected_email_id {
                        if let Some(email) = feature_data
                            .mail_data
                            .emails
                            .iter()
                            .find(|e| e.id == email_id)
                        {
                            // Back button
                            let back_button = egui::Button::new(
                                RichText::new("Back to emails")
                                    .font(FontId::new(13.0, FontFamily::Proportional))
                                    .color(theme.accent),
                            )
                            .fill(theme.background)
                            .corner_radius(4);
                            if ui.add(back_button).clicked() {
                                app.email_dialog_open = false;
                                app.selected_email_id = None;
                            }
                            ui.add_space(12.0);

                            // Email details
                            render_email_detail(app, ui, email, &theme);
                        } else {
                            ui.label(
                                RichText::new("Error: Email not found")
                                    .font(FontId::new(13.0, FontFamily::Proportional))
                                    .color(Color32::from_rgb(200, 100, 100)),
                            );
                            app.email_dialog_open = false;
                            app.selected_email_id = None;
                        }
                    } else {
                        app.email_dialog_open = false;
                    }
                }
                // Folder and email list view
                else {
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

                                let button_fill = if is_active {
                                    theme.accent
                                } else {
                                    theme.background
                                };

                                let text_color = if is_active {
                                    theme.background
                                } else {
                                    theme.text
                                };

                                let folder_label = RichText::new(folder_text.clone())
                                    .font(FontId::new(12.0, FontFamily::Proportional))
                                    .color(text_color);

                                let button = ui
                                    .add(
                                        egui::Button::new(folder_label)
                                            .fill(button_fill)
                                            .corner_radius(4)
                                            .min_size(egui::vec2(ui.available_width(), 24.0))
                                            .sense(Sense::click()),
                                    )
                                    .on_hover_ui(|ui| {
                                        ui.style_mut().visuals.popup_shadow = epaint::Shadow::NONE;
                                        ui.style_mut().visuals.override_text_color =
                                            Some(theme.text);
                                        ui.style_mut().visuals.window_fill = theme.panel;
                                        ui.label(
                                            egui::RichText::new(format!("View {}", folder.name))
                                                .color(theme.text),
                                        );
                                    })
                                    .on_hover_cursor(egui::CursorIcon::PointingHand);

                                if button.hovered() {
                                    ui.painter().rect_filled(button.rect, 4, theme.accent);
                                    ui.painter().text(
                                        button.rect.center(),
                                        egui::Align2::CENTER_CENTER,
                                        folder_text,
                                        FontId::new(12.0, FontFamily::Proportional),
                                        Color32::from_rgb(255, 255, 255),
                                    );
                                }

                                if button.clicked() {
                                    app.active_mail_folder_id = Some(folder.id);
                                }
                            }
                        });

                        ui.add_space(12.0);

                        // Emails (main area)
                        ui.vertical(|ui| {
                            ui.set_min_width(400.0);

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
                                    let clicked = render_email_card(ui, email, index, app, &theme);
                                    if clicked {
                                        app.selected_email_id = Some(email.id);
                                        app.email_dialog_open = true;
                                    }
                                    ui.add_space(8.0);
                                }
                            }
                        });
                    });
                }
            });
    } else {
        create_content_frame(&theme)
            .fill(theme.secondary_background)
            .corner_radius(8.0)
            .inner_margin(Margin::same(12))
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
