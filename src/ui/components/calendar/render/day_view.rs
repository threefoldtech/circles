use chrono::{Duration, Local};
use egui::{Button, Frame, RichText, ScrollArea, Stroke, Ui, Vec2};

use super::super::state::CalendarState;
use crate::utils::config::Theme;

pub fn render_day_view(ui: &mut Ui, state: &mut CalendarState, theme: &Theme) {
    let selected_date = state.selected_date;
    let selected_date_naive = selected_date.date_naive();
    let current_date = Local::now().date_naive();
    let is_today = selected_date_naive == current_date;

    ui.vertical(|ui| {
        // Set full width for the calendar
        ui.set_width(ui.available_width());

        // Navigation header
        super::navigation::render_navigation_header(ui, state, theme);

        // Main frame for the day view
        Frame::default()
            .fill(theme.background)
            .outer_margin(8.0)
            .show(ui, |ui| {
                // Date header
                ui.vertical_centered(|ui| {
                    if is_today {
                        let today_label = RichText::new("Today")
                            .color(egui::Color32::WHITE)
                            .size(16.0)
                            .strong();
                        let today_button = Button::new(today_label)
                            .fill(egui::Color32::from_rgb(234, 67, 53))
                            .corner_radius(12)
                            .min_size(Vec2::new(80.0, 28.0));
                        ui.add(today_button);
                        ui.add_space(12.0);
                    }

                    let date_text = selected_date.format("%A, %B %d").to_string();
                    ui.add(egui::Label::new(
                        RichText::new(date_text)
                            .color(theme.header_text)
                            .size(24.0)
                            .strong(),
                    ));
                    ui.add_space(16.0);
                });

                // Full-width upcoming events section
                Frame::default()
                    .fill(theme.panel)
                    .stroke(Stroke::new(1.0, theme.border))
                    .corner_radius(8.0)
                    .outer_margin(4.0)
                    .inner_margin(12.0)
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        ui.heading(
                            RichText::new("Upcoming Events")
                                .color(theme.header_text)
                                .size(18.0)
                                .strong(),
                        );
                        ui.add_space(12.0);

                        // Collect and sort upcoming events
                        let mut upcoming_events = Vec::new();
                        for i in 0..7 {
                            let date = current_date + Duration::days(i);
                            if let Some(events) = state.event_map.get(&date) {
                                for event in events {
                                    upcoming_events.push((date, event));
                                }
                            }
                        }
                        upcoming_events.sort_by(|a, b| {
                            a.0.cmp(&b.0)
                                .then_with(|| a.1.start_time.cmp(&b.1.start_time))
                        });

                        // Scrollable area for events
                        ScrollArea::vertical()
                            .auto_shrink([false, false])
                            .max_height(300.0)
                            .show(ui, |ui| {
                                for (date, event) in upcoming_events.iter().take(5) {
                                    let is_event_today = *date == current_date;

                                    Frame::default()
                                        .fill(theme.secondary_background)
                                        .corner_radius(6.0)
                                        .outer_margin(4.0)
                                        .inner_margin(8.0)
                                        .show(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                // Event color indicator
                                                let (rect, _) = ui.allocate_exact_size(
                                                    Vec2::new(8.0, 8.0),
                                                    egui::Sense::hover(),
                                                );
                                                ui.painter().circle_filled(
                                                    rect.center(),
                                                    4.0,
                                                    event.color,
                                                );
                                                ui.add_space(8.0);

                                                ui.vertical(|ui| {
                                                    // Event title
                                                    ui.label(
                                                        RichText::new(&event.title)
                                                            .strong()
                                                            .color(theme.text)
                                                            .size(14.0),
                                                    );

                                                    // Date and time
                                                    let date_text = if is_event_today {
                                                        format!(
                                                            "Today, {}",
                                                            event
                                                                .start_time
                                                                .with_timezone(&Local)
                                                                .format("%H:%M")
                                                        )
                                                    } else {
                                                        format!(
                                                            "{}, {}",
                                                            date.format("%a, %b %d"),
                                                            event
                                                                .start_time
                                                                .with_timezone(&Local)
                                                                .format("%H:%M")
                                                        )
                                                    };
                                                    ui.label(
                                                        RichText::new(date_text)
                                                            .color(theme.secondary_text)
                                                            .size(12.0),
                                                    );
                                                });
                                            });
                                        });
                                    ui.add_space(8.0);
                                }

                                if upcoming_events.is_empty() {
                                    ui.label(
                                        RichText::new("No upcoming events")
                                            .color(theme.secondary_text)
                                            .italics()
                                            .size(14.0),
                                    );
                                }
                            });
                    });
            });
    });
}
