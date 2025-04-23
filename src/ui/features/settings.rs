use crate::{
    models::{
        notification::{AppNotification, NotificationPriority},
        user::Theme as ThemeMode,
    },
    ui::app_layout::create_content_frame,
    utils::config::Theme,
};
use egui::{Button, Color32, RichText, Stroke, Ui, Vec2};

use crate::app::CircleApp;

/// Renders the settings screen with a full-width settings card.
pub fn render_settings(app: &mut CircleApp, ui: &mut Ui, ctx: &egui::Context) {
    ui.add_space(16.0);

    let theme = app.get_current_theme();
    let frame = create_content_frame(&theme);
    frame.show(ui, |ui| {
        ui.set_min_width(ui.available_width());

        ui.vertical(|ui| {
            render_theme_settings(ui, app, ctx);
            ui.add_space(16.0);

            render_notification_settings(ui, app);
            ui.add_space(16.0);

            render_circle_settings(ui, app);
            ui.add_space(16.0);

            render_save_button(ui, app, ctx);
        });
    });
}

fn render_theme_settings(ui: &mut Ui, app: &mut CircleApp, ctx: &egui::Context) {
    let theme = app.get_current_theme();

    ui.label(
        RichText::new("Theme Settings")
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

    let mut current_theme = app
        .user
        .as_ref()
        .map(|u| u.preferences.theme)
        .unwrap_or(ThemeMode::System);

    ui.horizontal(|ui| {
        if ui
            .radio_value(&mut current_theme, ThemeMode::Light, "Light")
            .clicked()
        {
            app.set_theme(ctx, ThemeMode::Light);
        }
        if ui
            .radio_value(&mut current_theme, ThemeMode::Dark, "Dark")
            .clicked()
        {
            app.set_theme(ctx, ThemeMode::Dark);
        }
        if ui
            .radio_value(&mut current_theme, ThemeMode::System, "System")
            .clicked()
        {
            app.set_theme(ctx, ThemeMode::System);
        }
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
    let theme = app.get_current_theme();

    ui.horizontal(|ui| {
        ui.add_space(ui.available_width() - 120.0 - 16.0);
        if ui
            .add(
                Button::new(
                    RichText::new("Save Settings")
                        .size(14.0)
                        .color(Color32::WHITE),
                )
                .fill(Color32::from_rgb(66, 133, 244))
                .corner_radius(6)
                .min_size(Vec2::new(120.0, 36.0)),
            )
            .clicked()
        {
            if let Some(user) = &mut app.user {
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
