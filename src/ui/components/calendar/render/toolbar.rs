use egui::{Button, RichText, Stroke, Ui, Vec2};

use super::super::state::{CalendarState, CalendarViewMode};
use crate::utils::config::Theme;

pub fn render_toolbar(ui: &mut Ui, state: &mut CalendarState, theme: &Theme) {
    ui.horizontal(|ui| {
        let buttons = [
            ("New Event", None),
            ("Today", None),
            ("Day", Some(CalendarViewMode::Day)),
            ("Week", Some(CalendarViewMode::Week)),
            ("Month", Some(CalendarViewMode::Month)),
            ("Year", Some(CalendarViewMode::Year)),
        ];

        for (label, view_mode) in buttons {
            if create_button(ui, label, theme).clicked() {
                match label {
                    "New Event" => {
                        state.new_event =
                            Some(crate::ui::components::calendar::event::Event::default());
                        state.show_event_dialog = true;
                    }
                    "Today" => {
                        state.selected_date = chrono::Local::now();
                    }
                    _ => {
                        if let Some(mode) = view_mode {
                            state.view_mode = mode;
                        }
                    }
                }
            }
            ui.add_space(5.0);
        }
    });
}

fn create_button(ui: &mut Ui, text: &str, theme: &Theme) -> egui::Response {
    ui.add(
        Button::new(RichText::new(text).color(theme.light_color))
            .fill(theme.accent)
            .stroke(Stroke::NONE)
            .min_size(Vec2::new(32.0, 32.0))
            .corner_radius(8),
    )
}
