use crate::{
    models::user::Theme as ThemeMode,
    ui::{app_layout::create_content_frame, components::button::render_button},
    utils::config::Theme,
};
use eframe::egui::Stroke;
use egui::{RichText, Ui};

use crate::app::CircleApp;

// Store the selected theme temporarily
#[derive(Clone, Copy)]
struct AppSettingsState {
    selected_theme: ThemeMode,
}

impl Default for AppSettingsState {
    fn default() -> Self {
        Self {
            selected_theme: ThemeMode::System,
        }
    }
}

/// Renders the app settings screen with a full-width settings card.
pub fn render_app_settings(app: &mut CircleApp, ui: &mut Ui, ctx: &egui::Context, theme: &Theme) {
    ui.add_space(16.0);

    // Get or create app settings state
    let app_settings_id = "app_settings_state";
    let mut state = ui.ctx().memory_mut(|mem| {
        mem.data
            .get_persisted::<AppSettingsState>(egui::Id::new(app_settings_id))
            .unwrap_or_else(|| {
                // Initialize with current user theme
                let current_theme = app
                    .user
                    .as_ref()
                    .map(|u| u.preferences.theme)
                    .unwrap_or(ThemeMode::System);

                AppSettingsState {
                    selected_theme: current_theme,
                }
            })
    });

    let frame = create_content_frame(theme);
    frame.show(ui, |ui| {
        ui.set_min_width(ui.available_width());

        ui.vertical(|ui| {
            // App Settings Header
            ui.label(
                RichText::new("App Settings")
                    .size(20.0)
                    .strong()
                    .color(theme.text),
            );
            ui.add_space(16.0);

            // Theme Settings
            render_theme_settings(ui, &mut state, theme);
            ui.add_space(16.0);

            // Save Button
            render_save_button(ui, app, ctx, state);
        });
    });

    // Store the state for the next frame
    ui.ctx().memory_mut(|mem| {
        mem.data
            .insert_persisted(egui::Id::new(app_settings_id), state);
    });
}

fn render_theme_settings(ui: &mut Ui, state: &mut AppSettingsState, theme: &Theme) {
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

    ui.horizontal(|ui| {
        ui.radio_value(&mut state.selected_theme, ThemeMode::Light, "Light");
        ui.radio_value(&mut state.selected_theme, ThemeMode::Dark, "Dark");
        ui.radio_value(&mut state.selected_theme, ThemeMode::System, "System");
    });
}

fn render_save_button(
    ui: &mut Ui,
    app: &mut CircleApp,
    ctx: &egui::Context,
    state: AppSettingsState,
) {
    ui.horizontal(|ui| {
        ui.add_space(ui.available_width() - 110.0 - 16.0);
        let theme = app.get_current_theme_with_context(ctx);
        if render_button(ui, "Save Settings", true, &theme, None).clicked() {
            if let Some(user) = &mut app.user {
                // Apply the selected theme
                if user.preferences.theme != state.selected_theme {
                    // Only update if the theme has changed
                    user.preferences.theme = state.selected_theme;
                    // Apply the theme
                    app.apply_theme(ctx);
                }

                // Save the current preferences
                app.save_user_preferences(ctx);
            }
        }
    });
}
