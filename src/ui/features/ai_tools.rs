use egui::{RichText, Ui};

use crate::{app::CircleApp, ui::app_layout::create_content_frame};

pub fn render_ai_tools(app: &CircleApp, ui: &mut Ui) {
    ui.add_space(16.0);
    let theme = app.get_current_theme();
    create_content_frame(&theme).show(ui, |ui| {
        ui.label(
            RichText::new("AI Tools feature not yet implemented")
                .size(14.0)
                .color(theme.secondary_text),
        );
    });
}
