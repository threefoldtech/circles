use eframe::egui;

use crate::app::CircleApp;
use crate::ui::app_layout::create_content_frame;
use crate::ui::components::button;

#[allow(unused_variables)]
pub fn render_documents(app: &CircleApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        if ui
            .add(button::create_button("New Document", "➕"))
            .clicked()
        {
            // TODO: Implement new document
        }
        ui.add_space(8.0);
        if ui.add(button::create_button("Upload", "📤")).clicked() {
            // TODO: Implement upload
        }
        ui.add_space(8.0);
    });
    ui.add_space(16.0);
    create_content_frame().show(ui, |ui| ui.label("Documents feature not yet implemented"));
}
