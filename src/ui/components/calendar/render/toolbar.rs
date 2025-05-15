use super::super::state::{CalendarState, CalendarViewMode};
use crate::{ui::components::button::render_button, utils::config::Theme};
use egui::{Button, RichText, Ui, Vec2};

pub fn render_toolbar(ui: &mut Ui, state: &mut CalendarState, theme: &Theme) {
    ui.horizontal(|ui| {
        ui.add_space(12.0);
        render_event_button(ui, state, theme);
        ui.add_space(10.0);
        render_today_button(ui, state, theme);
    });

    ui.horizontal(|ui| {
        ui.add_space(12.0);
        render_view_mode_buttons(ui, state, theme);
    });
}

fn render_event_button(ui: &mut Ui, state: &mut CalendarState, theme: &Theme) {
    let mut button = render_button(ui, "New Event", true, theme, Some("➕"));
    button = button.on_hover_cursor(egui::CursorIcon::PointingHand);
    button = button.on_hover_text(
        RichText::new("Create a new event")
            .size(12.0)
            .color(theme.light_color),
    );

    if button.clicked() {
        state.new_event = Some(crate::ui::components::calendar::event::Event::default());
        state.show_event_dialog = true;
    }
}

fn render_today_button(ui: &mut Ui, state: &mut CalendarState, theme: &Theme) {
    let mut button = render_button(ui, "Today", true, theme, Some("📅"));
    button = button.on_hover_cursor(egui::CursorIcon::PointingHand);
    button = button.on_hover_text(
        RichText::new("Go to today")
            .size(12.0)
            .color(theme.light_color),
    );

    if button.clicked() {
        state.selected_date = chrono::Local::now();
    }
}

fn render_view_mode_buttons(ui: &mut Ui, state: &mut CalendarState, theme: &Theme) {
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
}

fn create_view_button(ui: &mut Ui, text: &str, theme: &Theme, is_active: bool) -> egui::Response {
    let fill_color = if is_active {
        theme.accent
    } else {
        theme.secondary_background
    };
    let text_color = if is_active {
        theme.light_color
    } else {
        theme.text
    };
    let label = RichText::new(text).size(14.0).color(text_color);

    let button = Button::new(label)
        .fill(fill_color)
        .corner_radius(4.0)
        .min_size(Vec2::new(100.0, 32.0));

    let mut response = ui
        .add(button)
        .on_hover_cursor(egui::CursorIcon::PointingHand);

    if response.hovered() && !is_active {
        let hover_text = format!("Switch to {} view", text);
        response = response.on_hover_text(
            RichText::new(hover_text)
                .size(12.0)
                .color(theme.light_color),
        );
    }

    response
}
