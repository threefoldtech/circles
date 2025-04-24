use egui::{Button, RichText, Stroke, Ui, Vec2};

use super::super::state::{CalendarState, CalendarViewMode};
use crate::utils::config::Theme;

pub fn render_toolbar(ui: &mut Ui, state: &mut CalendarState, theme: &Theme) {
    // Create a container for the toolbar with proper spacing
    egui::Frame::new().outer_margin(8.0).show(ui, |ui| {
        // Use a grid layout for better alignment
        egui::Grid::new("calendar_toolbar_grid")
            .spacing([8.0, 8.0])
            .show(ui, |ui| {
                // Action buttons group
                ui.horizontal(|ui| {
                    if create_action_button(ui, "New Event", theme, false).clicked() {
                        state.new_event =
                            Some(crate::ui::components::calendar::event::Event::default());
                        state.show_event_dialog = true;
                    }

                    if create_action_button(ui, "Today", theme, false).clicked() {
                        state.selected_date = chrono::Local::now();
                    }
                });

                ui.end_row();

                // View mode buttons group
                ui.horizontal(|ui| {
                    let view_modes = [
                        ("Day", CalendarViewMode::Day),
                        ("Week", CalendarViewMode::Week),
                        ("Month", CalendarViewMode::Month),
                        ("Year", CalendarViewMode::Year),
                    ];

                    for (label, mode) in view_modes {
                        let is_active = state.view_mode == mode;
                        if create_view_button(ui, label, theme, is_active).clicked() {
                            state.view_mode = mode;
                        }
                    }
                });
            });
    });
}

// Button for actions like "New Event" and "Today"
fn create_action_button(ui: &mut Ui, text: &str, theme: &Theme, is_active: bool) -> egui::Response {
    let (text_color, bg_color) = if is_active {
        (theme.light_color, theme.accent)
    } else {
        (theme.text, theme.secondary_background)
    };

    let mut response = ui.add(
        Button::new(RichText::new(text).color(text_color).size(14.0))
            .fill(bg_color)
            .stroke(Stroke::new(1.0, theme.border))
            .min_size(Vec2::new(90.0, 32.0))
            .corner_radius(6.0),
    );

    // Add hover effect
    if response.hovered() {
        let hover_text = text.to_string();
        response = response.on_hover_text(hover_text);
    }

    response
}

// Button for view mode selection
fn create_view_button(ui: &mut Ui, text: &str, theme: &Theme, is_active: bool) -> egui::Response {
    let (text_color, bg_color) = if is_active {
        (theme.light_color, theme.accent)
    } else {
        (theme.text, theme.panel)
    };

    let mut response = ui.add(
        Button::new(RichText::new(text).color(text_color).size(14.0))
            .fill(bg_color)
            .stroke(Stroke::new(1.0, theme.border))
            .min_size(Vec2::new(70.0, 32.0))
            .corner_radius(6.0),
    );

    // Add hover effect and accessibility
    if response.hovered() && !is_active {
        ui.painter().rect_filled(response.rect, 6.0, theme.hover);

        // Add accessibility tooltip
        let hover_text = format!("Switch to {} view", text);
        response = response.on_hover_text(hover_text);
    }

    // Add keyboard navigation support
    if is_active {
        response = response.highlight();
    }

    response
}
