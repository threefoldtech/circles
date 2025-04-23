use eframe::egui;

use crate::app::CircleApp;
use crate::ui::app_layout::create_content_frame;

#[allow(unused_variables)]
pub fn render_video_conference(app: &CircleApp, ui: &mut egui::Ui) {
    ui.add_space(16.0);
    let theme = app.get_current_theme();
    create_content_frame(&theme).show(ui, |ui| {
        ui.label("Video Conference feature not yet implemented")
    });
}
