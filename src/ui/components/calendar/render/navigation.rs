use chrono::Datelike;
use egui::{Button, RichText, Ui, Vec2};

use super::super::state::{CalendarState, CalendarViewMode};
use crate::ui::features::calendar::Calendar;
use crate::utils::config::Theme;

pub fn render_navigation_header(ui: &mut Ui, state: &mut CalendarState, theme: &Theme) {
    let header_text = get_header_text(state);
    let is_today = state.selected_date.date_naive() == chrono::Local::now().date_naive();

    ui.horizontal(|ui| {
        if ui
            .add(
                Button::new(RichText::new("◀").color(theme.accent)).min_size(Vec2::new(32.0, 32.0)),
            )
            .clicked()
        {
            Calendar::navigate_state(state, false);
        }

        ui.with_layout(
            egui::Layout::top_down_justified(egui::Align::Center),
            |ui| {
                ui.add_space(8.0);
                ui.strong(
                    RichText::new(header_text)
                        .color(if is_today {
                            theme.error
                        } else {
                            theme.header_text
                        })
                        .size(16.0),
                );
                ui.add_space(8.0);
            },
        );

        if ui
            .add(
                Button::new(RichText::new("▶").color(theme.accent)).min_size(Vec2::new(32.0, 32.0)),
            )
            .clicked()
        {
            Calendar::navigate_state(state, true);
        }
    });
    ui.separator();
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
