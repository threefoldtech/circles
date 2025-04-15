use eframe::{egui, epaint};
use egui::{Color32, FontFamily, FontId, Margin, RichText, Rounding, Sense};

use crate::app::CircleApp;
use crate::models::dummy_data::Email;
use crate::ui::app_layout::{create_action_button, create_content_frame};
use crate::ui::components::mail::compose_dialog::{open_compose_dialog, render_compose_dialog};
use crate::ui::components::mail::email_card::{get_folder_icon, render_email_card};
use crate::ui::components::mail::email_detials::render_email_detail;
use crate::utils::config::Theme;

pub fn render_mail(app: &mut CircleApp, ui: &mut egui::Ui) {
    let theme = Theme::new();

    // Top bar
    ui.horizontal(|ui| {
        ui.add_space(12.0);
        let compose_button = create_action_button("Compose", "✏️")
            .fill(theme.accent)
            .rounding(Rounding::same(6.0));
        if ui.add(compose_button).clicked() {
            // Open compose dialog
            open_compose_dialog(app, ui);
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
            .fill(theme.secondary_background)
            .rounding(Rounding::same(8.0))
            .inner_margin(Margin::same(12.0))
            .show(ui, |ui| {
                // Compose dialog
                if app.compose_dialog_open {
                    render_compose_dialog(ui, app, &theme);
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
                            .rounding(Rounding::same(4.0));
                            if ui.add(back_button).clicked() {
                                app.email_dialog_open = false;
                                app.selected_email_id = None;
                            }
                            ui.add_space(12.0);

                            // Email details
                            render_email_detail(ui, email, &theme);
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
                                    let clicked = render_email_card(ui, email, index, app);
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
