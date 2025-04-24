use chrono::Datelike;
use egui::{Button, Key, RichText, Stroke, Ui, Vec2};

use super::super::state::{CalendarState, CalendarViewMode};
use crate::ui::features::calendar::Calendar;
use crate::utils::config::Theme;

pub fn render_navigation_header(ui: &mut Ui, state: &mut CalendarState, theme: &Theme) {
    let header_text = get_header_text(state);
    let is_today = state.selected_date.date_naive() == chrono::Local::now().date_naive();

    // Create a centered container for the navigation header
    ui.vertical_centered(|ui| {
        ui.add_space(8.0);

        // Navigation controls in a horizontal layout
        ui.horizontal(|ui| {
            // Previous button with theme-based styling
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

            // Add accessibility
            prev_button.on_hover_text("Previous");

            ui.add_space(16.0);

            // Month/year display with proper styling
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

            ui.add_space(16.0);

            // Next button with theme-based styling
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

            // Add accessibility
            next_button.on_hover_text("Next");
        });

        ui.add_space(8.0);
    });

    // Add a subtle separator with theme color
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
