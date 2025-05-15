use eframe::egui;

use crate::app::CircleApp;
use crate::models::user::Theme as ThemeMode;
use crate::ui::app_layout;

/// Wrapper for the CircleApp that implements eframe::App
pub struct EguiApp {
    /// The main application state
    pub app: CircleApp,
    /// Track the last applied theme to detect changes
    pub last_theme_mode: Option<ThemeMode>,
}

impl EguiApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Create the app
        let app = CircleApp::new(cc);

        // Get the initial theme mode
        let initial_theme_mode = app.user.as_ref().map(|u| u.preferences.theme);

        // Apply the theme to the context
        let theme = app.get_current_theme_with_context(&cc.egui_ctx);
        cc.egui_ctx.set_visuals(theme.to_visuals());

        Self {
            app,
            last_theme_mode: initial_theme_mode,
        }
    }
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check if theme has changed
        let current_theme_mode = self.app.user.as_ref().map(|u| u.preferences.theme);

        // If theme mode changed, reapply the theme
        if self.last_theme_mode != current_theme_mode {
            let theme = self.app.get_current_theme_with_context(ctx);
            ctx.set_visuals(theme.to_visuals());
            self.last_theme_mode = current_theme_mode;
        }

        // Render the app
        app_layout::render(&mut self.app, ctx);
    }
}
