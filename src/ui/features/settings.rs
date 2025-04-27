use crate::{ui::app_layout::create_content_frame, ui::components::button::render_button};
use eframe::egui::Stroke;
use egui::{RichText, Ui};

use crate::app::CircleApp;

/// Renders the settings screen with a full-width settings card.
pub fn render_settings(app: &mut CircleApp, ui: &mut Ui, ctx: &egui::Context) {
    ui.add_space(16.0);

    let theme = app.get_current_theme();
    let frame = create_content_frame(&theme);
    frame.show(ui, |ui| {
        ui.set_min_width(ui.available_width());

        ui.vertical(|ui| {
            // Circle Settings Header
            ui.label(
                RichText::new("Circle Settings")
                    .size(20.0)
                    .strong()
                    .color(theme.text),
            );
            ui.add_space(16.0);

            render_notification_settings(ui, app);
            ui.add_space(16.0);

            render_circle_settings(ui, app);
            ui.add_space(16.0);

            render_save_button(ui, app, ctx);
        });
    });
}

fn render_notification_settings(ui: &mut Ui, app: &CircleApp) {
    let theme = app.get_current_theme();

    ui.label(
        RichText::new("Notification Settings")
            .size(16.0)
            .strong()
            .color(theme.text),
    );
    ui.add_space(8.0);
    ui.painter().hline(
        ui.available_rect_before_wrap().x_range(),
        ui.cursor().top(),
        Stroke::new(1.0, theme.border),
    );
    ui.add_space(12.0);

    if let Some(user) = &app.user {
        let prefs = &user.preferences.notification_preferences;
        ui.checkbox(
            &mut prefs.email_notifications.clone(),
            RichText::new("Email notifications")
                .size(14.0)
                .color(theme.text),
        );
        ui.checkbox(
            &mut prefs.push_notifications.clone(),
            RichText::new("Push notifications")
                .size(14.0)
                .color(theme.text),
        );
        ui.checkbox(
            &mut prefs.in_app_notifications.clone(),
            RichText::new("In-app notifications")
                .size(14.0)
                .color(theme.text),
        );
    }
}

fn render_circle_settings(ui: &mut Ui, app: &CircleApp) {
    let theme = app.get_current_theme();

    ui.label(
        RichText::new("Circle Settings")
            .size(16.0)
            .strong()
            .color(theme.text),
    );
    ui.add_space(8.0);
    ui.painter().hline(
        ui.available_rect_before_wrap().x_range(),
        ui.cursor().top(),
        Stroke::new(1.0, theme.border),
    );
    ui.add_space(12.0);

    ui.checkbox(
        &mut true,
        RichText::new("Show all circles")
            .size(14.0)
            .color(theme.text),
    );
    ui.checkbox(
        &mut false,
        RichText::new("Auto-join new circles")
            .size(14.0)
            .color(theme.text),
    );
}

fn render_save_button(ui: &mut Ui, app: &mut CircleApp, ctx: &egui::Context) {
    ui.horizontal(|ui| {
        ui.add_space(ui.available_width() - 110.0 - 16.0);
        let theme = app.get_current_theme();
        if render_button(ui, "Save Settings", true, &theme, None).clicked() {
            if let Some(_) = &mut app.user {
                // Save the current preferences
                app.save_user_preferences(ctx);

                // Show a success notification
                // app.notification_manager.push(AppNotification::new(
                //     "Settings saved successfully".to_string(),
                //     NotificationPriority::Normal,
                // ));
            }
        }
    });
}
