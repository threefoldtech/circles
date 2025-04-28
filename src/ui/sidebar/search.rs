use crate::app::CircleApp;
use crate::utils::config::Theme;
use egui::{Frame, Margin, RichText, Stroke, Ui, Vec2};

use super::SEARCH_SPACE;

// Search box
pub fn render_search_box(ui: &mut Ui, app: &mut CircleApp, theme: &Theme) {
    let search_frame = Frame::new()
        .fill(theme.hover)
        .corner_radius(20.0)
        .inner_margin(Margin::same(10))
        .stroke(Stroke::NONE);

    ui.horizontal(|ui| {
        ui.add_space(SEARCH_SPACE);
        search_frame.show(ui, |ui| {
            ui.set_max_width(228.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new("🔍").size(16.0).color(theme.text));
                ui.add_space(super::SECTION_SPACE);
                let original_style = ui.style().clone();
                ui.style_mut().visuals.widgets.inactive.bg_fill = theme.transparent;
                ui.style_mut().visuals.widgets.active.bg_fill = theme.transparent;
                ui.style_mut().visuals.widgets.hovered.bg_fill = theme.transparent;

                ui.add(
                    egui::TextEdit::singleline(&mut app.search_query)
                        .hint_text(RichText::new("Search circles...").color(theme.text))
                        .text_color(theme.text)
                        .frame(false)
                        .margin(Vec2::ZERO)
                        .desired_width(180.0),
                );
                ui.set_style(original_style);
            });
        });
        ui.add_space(SEARCH_SPACE);
    });
}
