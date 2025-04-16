use egui::{Color32, RichText, Ui};

use crate::{app::CircleApp, ui::app_layout::create_content_frame};

pub fn render_ai_tools(_: &CircleApp, ui: &mut Ui) {
    ui.add_space(16.0);
    create_content_frame().show(ui, |ui| {
        ui.label(
            RichText::new("AI Tools feature not yet implemented")
                .size(14.0)
                .color(Color32::from_rgb(100, 110, 120)),
        );
    });
}
