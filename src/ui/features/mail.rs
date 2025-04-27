use eframe::egui;
use egui::{Margin, RichText};
use std::time::{Duration, Instant};
use uuid::Uuid;

use crate::app::CircleApp;
use crate::ui::components::button::render_button;
use crate::ui::components::mail::compose::{open_compose_screen, render_compose_screen};
use crate::ui::components::mail::email_card::{get_folder_icon, render_email_card};
use crate::ui::components::mail::email_details::render_email_detail;
use crate::utils::config::Theme;

pub fn render_mail(app: &mut CircleApp, ui: &mut egui::Ui) {
    let theme = app.get_current_theme();

    if app.compose_dialog_open {
        render_compose_screen(ui, app, &theme);
        return;
    }

    render_top_bar(ui, app, &theme);

    if app.is_refreshing {
        render_loading_message(ui, &theme);
        if ui.input(|i| i.stable_dt) > 0.5 {
            app.is_refreshing = false;
        }
    }

    if app.email_dialog_open {
        render_email_view(app, ui, &theme);
    } else {
        render_folder_list(app, ui, &theme);
        ui.add_space(16.0);
        render_email_count(app, ui, &theme);
        ui.add_space(8.0);
        render_email_list(app, ui, &theme);
    }
}

fn render_top_bar(ui: &mut egui::Ui, app: &mut CircleApp, theme: &Theme) {
    ui.horizontal(|ui| {
        ui.add_space(12.0);
        render_compose_button(ui, app, theme);
        ui.add_space(10.0);
        render_refresh_button(ui, app, theme);
        ui.add_space(10.0);
    });
}

fn render_compose_button(ui: &mut egui::Ui, app: &mut CircleApp, theme: &Theme) {
    let compose = render_button(ui, "Compose", true, theme, Some("✏️"));
    if compose.clicked() {
        open_compose_screen(app, ui);
    }
    if compose.hovered() {
        ui.ctx().request_repaint();
        ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
    }
}

pub fn render_refresh_button(ui: &mut egui::Ui, app: &mut CircleApp, theme: &Theme) {
    let icon = if app.is_refreshing { "⌛" } else { "🔄" };
    let refresh = render_button(ui, "Refresh", true, theme, Some(icon));

    if refresh.clicked() {
        app.is_refreshing = true;
        app.refresh_start_time = Instant::now();
        ui.ctx().request_repaint();
    }

    if refresh.hovered() {
        ui.ctx().request_repaint();
        ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
    }

    if app.is_refreshing {
        if app.refresh_start_time.elapsed() >= Duration::from_millis(2500) {
            app.is_refreshing = false;
        }
    }
}

fn render_loading_message(ui: &mut egui::Ui, theme: &Theme) {
    ui.horizontal(|ui| {
        ui.add_space(16.0);
        ui.label(
            RichText::new("Refreshing emails...")
                .size(14.0)
                .color(theme.secondary_text),
        );
    });
}

fn render_email_view(app: &mut CircleApp, ui: &mut egui::Ui, theme: &Theme) {
    if let Some(email_id) = app.selected_email_id {
        let email_opt = app.active_feature_data.as_ref().and_then(|data| {
            data.mail_data
                .emails
                .iter()
                .find(|e| e.id == email_id)
                .cloned()
        });

        if let Some(email) = email_opt {
            if render_button(ui, "Back to emails", false, theme, None).clicked() {
                app.email_dialog_open = false;
                app.selected_email_id = None;
            }
            ui.add_space(16.0);
            render_email_detail(app, ui, &email, theme);
        }
    }
}

fn render_folder_list(app: &mut CircleApp, ui: &mut egui::Ui, theme: &Theme) {
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

    egui::Frame::new()
        .inner_margin(Margin::same(16))
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                for folder in &["Inbox", "Sent", "Drafts", "Spam", "Trash"] {
                    render_folder_button(ui, app, folder, &folder_data[..], theme);
                    ui.add_space(8.0);
                }
            });
        });
}

fn render_folder_button(
    ui: &mut egui::Ui,
    app: &mut CircleApp,
    folder: &str,
    folder_data: &[(Uuid, String)],
    theme: &Theme,
) {
    let is_selected = app
        .active_mail_folder_id
        .map(|id| {
            folder_data
                .iter()
                .any(|(fid, name)| *fid == id && name.eq_ignore_ascii_case(folder))
        })
        .unwrap_or(false);

    let folder_icon = get_folder_icon(folder);
    let fill_color = if is_selected {
        theme.accent
    } else {
        theme.secondary_background
    };
    let text_color = if is_selected {
        theme.light_color
    } else {
        theme.text
    };
    let label = RichText::new(format!("{} {}", folder_icon, folder))
        .size(14.0)
        .color(text_color);

    if ui
        .add(
            egui::Button::new(label)
                .fill(fill_color)
                .corner_radius(4.0)
                .min_size(egui::Vec2::new(100.0, 32.0)),
        )
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .clicked()
    {
        if let Some((folder_id, _)) = folder_data
            .iter()
            .find(|(_, name)| name.eq_ignore_ascii_case(folder))
        {
            app.active_mail_folder_id = Some(*folder_id);
        }
    }
}

fn render_email_count(app: &CircleApp, ui: &mut egui::Ui, theme: &Theme) {
    let count =
        app.active_feature_data
            .as_ref()
            .map_or(0, |data| match app.active_mail_folder_id {
                Some(folder_id) => data
                    .mail_data
                    .emails
                    .iter()
                    .filter(|email| email.folder_id == folder_id)
                    .count(),
                None => data.mail_data.emails.len(),
            });

    ui.horizontal(|ui| {
        ui.add_space(16.0);
        ui.label(
            RichText::new(format!("{} emails", count))
                .size(16.0)
                .color(theme.header_text)
                .strong(),
        );
    });
}

fn render_email_list(app: &mut CircleApp, ui: &mut egui::Ui, theme: &Theme) {
    let emails = app
        .active_feature_data
        .as_ref()
        .map(|data| match app.active_mail_folder_id {
            Some(folder_id) => data
                .mail_data
                .emails
                .iter()
                .filter(|email| email.folder_id == folder_id)
                .cloned()
                .collect::<Vec<_>>(),
            None => data.mail_data.emails.clone(),
        })
        .unwrap_or_default();

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            for (index, email) in emails.iter().enumerate() {
                let cloned_email = email.clone();
                if render_email_card(ui, &cloned_email, index, app, theme) {
                    app.selected_email_id = Some(email.id);
                    app.email_dialog_open = true;
                }
                ui.add_space(8.0);
            }
        });
}
