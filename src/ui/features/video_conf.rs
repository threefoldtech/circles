use eframe::egui;

use crate::app::CircleApp;
use crate::ui::app_layout::{create_content_frame, render_header};

#[allow(unused_variables)]
pub fn render_video_conference(app: &CircleApp, ui: &mut egui::Ui) {
    render_header(ui, "📹", "Video Conference");
    ui.add_space(16.0);
    create_content_frame().show(ui, |ui| {
        ui.label("Video Conference feature not yet implemented")
    });
}
