use eframe::egui;

use crate::app::CircleApp;
use crate::ui::app_layout::{create_action_button, create_content_frame, render_header};

pub fn render_chat(app: &CircleApp, ui: &mut egui::Ui) {
    render_header(ui, "💬", "Chat");
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        if ui.add(create_action_button("New Chat", "➕")).clicked() {
            // TODO: Implement new chat
        }
        ui.add_space(8.0);
    });
    ui.add_space(16.0);
    create_content_frame().show(ui, |ui| ui.label("Chat feature not yet implemented"));
}
