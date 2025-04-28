use crate::app::CircleApp;
use crate::utils::config::Theme;
use egui::{Align, CursorIcon, Layout, RichText, Ui};

use super::helpers::create_add_button;

// Circle header with title and add button
pub fn render_circle_header(ui: &mut Ui, app: &mut CircleApp, theme: &Theme) {
    ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
        ui.vertical(|ui| {
            ui.heading(
                RichText::new("Circles")
                    .size(18.0)
                    .strong()
                    .color(theme.text),
            );
        });
        ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
            if ui
                .add(create_add_button(theme))
                .on_hover_text(
                    RichText::new("Create a new circle")
                        .size(12.0)
                        .color(theme.white),
                )
                .on_hover_cursor(CursorIcon::PointingHand)
                .clicked()
            {
                app.open_circle_dialog();
            }
        });
    });
}
