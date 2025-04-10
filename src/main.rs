use eframe::egui;
use env_logger;

mod app;
mod egui_app;
mod models;
mod services;
mod ui;
mod utils;

use egui_app::EguiApp;

fn main() -> eframe::Result<()> {
    // Initialize logger
    env_logger::init();

    // Set up native options
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_min_inner_size([800.0, 600.0]),
        // No centered option in newer egui versions
        ..Default::default()
    };

    // Run the application
    eframe::run_native(
        "Circle Collaboration System",
        native_options,
        Box::new(|cc| {
            // Create the app
            let app = app::CircleApp::new(cc);

            // Create the eframe app with the improved UI
            Box::new(EguiApp { app })
        }),
    )
}
