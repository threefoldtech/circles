use eframe::egui;

use crate::app::CircleApp;
use crate::ui::app_layout::{create_action_button, create_content_frame, render_header};

#[allow(unused_variables)]
pub fn render_documents(app: &CircleApp, ui: &mut egui::Ui) {
    render_header(ui, "📄", "Documents");
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        if ui.add(create_action_button("New Document", "➕")).clicked() {
            // TODO: Implement new document
        }
        ui.add_space(8.0);
        if ui.add(create_action_button("Upload", "📤")).clicked() {
            // TODO: Implement upload
        }
        ui.add_space(8.0);
    });
    ui.add_space(16.0);
    create_content_frame().show(ui, |ui| ui.label("Documents feature not yet implemented"));
}
