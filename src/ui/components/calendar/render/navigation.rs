use chrono::Datelike;
use egui::{Button, Key, RichText, Stroke, Ui, Vec2};

use super::super::state::{CalendarState, CalendarViewMode};
use crate::ui::features::calendar::Calendar;
use crate::utils::config::Theme;

pub fn render_navigation_header(ui: &mut Ui, state: &mut CalendarState, theme: &Theme) {
    let header_text = get_header_text(state);
    let is_today = state.selected_date.date_naive() == chrono::Local::now().date_naive();

    let separator = ui.separator();
    ui.painter().line_segment(
        [separator.rect.left_top(), separator.rect.right_top()],
        (1.0, theme.border),
    );

    ui.columns(3, |columns| {
        columns[0].vertical_centered(|ui| {
            let prev_button = ui.add(
                Button::new(RichText::new("◀").color(theme.text).size(16.0))
                    .min_size(Vec2::new(40.0, 40.0))
                    .stroke(Stroke::new(1.0, theme.border))
                    .fill(theme.panel)
                    .corner_radius(20.0),
            );

            if prev_button.clicked() || ui.input(|i| i.key_pressed(Key::ArrowLeft)) {
                Calendar::navigate_state(state, false);
            }

            prev_button
                .on_hover_text(
                    RichText::new("Previous")
                        .color(theme.light_color)
                        .size(12.0),
                )
                .on_hover_cursor(egui::CursorIcon::PointingHand);
        });

        columns[1].vertical_centered(|ui| {
            ui.add(egui::Label::new(
                RichText::new(header_text)
                    .color(if is_today {
                        theme.accent
                    } else {
                        theme.header_text
                    })
                    .size(20.0)
                    .strong(),
            ));
        });

        columns[2].vertical_centered(|ui| {
            let next_button = ui.add(
                Button::new(RichText::new("▶").color(theme.text).size(16.0))
                    .min_size(Vec2::new(40.0, 40.0))
                    .stroke(Stroke::new(1.0, theme.border))
                    .fill(theme.panel)
                    .corner_radius(20.0),
            );

            if next_button.clicked() || ui.input(|i| i.key_pressed(Key::ArrowRight)) {
                Calendar::navigate_state(state, true);
            }

            next_button
                .on_hover_text(RichText::new("Next").color(theme.light_color).size(12.0))
                .on_hover_cursor(egui::CursorIcon::PointingHand);
        });
    });

    let separator = ui.separator();
    ui.painter().line_segment(
        [separator.rect.left_top(), separator.rect.right_top()],
        (1.0, theme.border),
    );
}

fn get_header_text(state: &CalendarState) -> String {
    match state.view_mode {
        CalendarViewMode::Year => state.selected_date.format("%Y").to_string(),
        CalendarViewMode::Month => state.selected_date.format("%B %Y").to_string(),
        CalendarViewMode::Week => format!(
            "Week of {} {}",
            state.selected_date.format("%B"),
            state.selected_date.day()
        ),
        CalendarViewMode::Day => state.selected_date.format("%B %d, %Y").to_string(),
    }
}
