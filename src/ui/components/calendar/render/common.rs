use chrono::{NaiveDate, Timelike};
use egui::{Button, Frame, Margin, RichText, Stroke, Ui, Vec2};

use super::super::state::CalendarState;
use crate::utils::config::Theme;

pub fn style_day_text(
    day_num: u32,
    is_current_month: bool,
    is_today: bool,
    current_date_naive: NaiveDate,
    state: &CalendarState,
    theme: &Theme,
) -> RichText {
    let mut text = RichText::new(format!("{:2}", day_num)).size(14.0);
    let has_events = state.event_map.get(&current_date_naive).is_some();

    if is_today {
        text = text.color(theme.accent).strong();
    } else if has_events && is_current_month {
        text = text.color(theme.accent).strong();
    } else if is_current_month {
        text = text.color(theme.text);
    } else {
        text = text.color(theme.secondary_text);
    }

    text
}

pub fn render_time_slot(
    ui: &mut Ui,
    state: &mut CalendarState,
    date: NaiveDate,
    hour: u32,
    width: f32,
    height: f32,
    theme: &Theme,
) {
    Frame::default()
        .stroke(Stroke::new(0.5, theme.border))
        .inner_margin(Margin::same(4))
        .corner_radius(4.0)
        .fill(theme.panel)
        .show(ui, |ui| {
            if let Some(day_events) = state.event_map.get(&date) {
                let hour_events: Vec<_> = day_events
                    .iter()
                    .filter(|e| e.start_time.with_timezone(&chrono::Local).hour() == hour)
                    .collect();

                if !hour_events.is_empty() {
                    for event in hour_events {
                        let duration_mins = event.duration_minutes();
                        let event_height = (duration_mins as f32 / 60.0) * height;
                        let duration_hours = duration_mins as f32 / 60.0;
                        let base_width = (duration_hours * 80.0).max(60.0);
                        let max_width = (width * 0.8).max(60.0);
                        let event_width = base_width.min(max_width);

                        let event_button = Button::new(
                            RichText::new(&event.title)
                                .color(theme.light_color)
                                .strong()
                                .size(14.0),
                        )
                        .fill(event.color)
                        .corner_radius(4.0)
                        .min_size(Vec2::new(event_width, event_height.min(height - 8.0)));

                        let mut response = ui.add(event_button);

                        if response.hovered() {
                            let hover_text =
                                format!("{}\nDuration: {} minutes", event.title, duration_mins);
                            response = response.on_hover_text(hover_text);
                        }

                        if response.clicked() {
                            state.selected_event = Some(event.id);
                        }
                    }
                } else {
                    let mut response =
                        ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::click());

                    if response.hovered() {
                        ui.painter().rect_filled(response.rect, 4.0, theme.hover);
                        let hover_text = format!("Click to add event at {}:00", hour);
                        response = response.on_hover_text(hover_text);
                    }

                    if response.clicked() {
                        crate::ui::features::calendar::Calendar::handle_time_slot_click_state(
                            state, date, hour,
                        );
                    }
                }
            } else {
                let mut response =
                    ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::click());

                if response.hovered() {
                    ui.painter().rect_filled(response.rect, 4.0, theme.hover);
                    let hover_text = format!("Click to add event at {}:00", hour);
                    response = response.on_hover_text(hover_text);
                }

                if response.clicked() {
                    crate::ui::features::calendar::Calendar::handle_time_slot_click_state(
                        state, date, hour,
                    );
                }
            }
        });
}
