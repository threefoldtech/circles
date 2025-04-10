use eframe::egui;

use crate::app::CircleApp;
use crate::ui::app_layout;

/// Wrapper for the CircleApp that implements eframe::App
pub struct EguiApp {
    /// The main application state
    pub app: CircleApp,
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        app_layout::render(&mut self.app, ctx);
    }
}
