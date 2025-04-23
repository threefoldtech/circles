use chrono::{Datelike, Duration, Local, NaiveDate, TimeZone, Timelike};
use egui::{Button, Frame, Grid, Margin, RichText, ScrollArea, Stroke, Ui};

use super::super::state::{CalendarState, CalendarViewMode};
use crate::utils::config::Theme;

pub fn render_year_view(ui: &mut Ui, state: &mut CalendarState, theme: &Theme) {
    let year = state.selected_date.year();

    Grid::new("year_grid").spacing([20.0, 20.0]).show(ui, |ui| {
        for month_chunk in (1..=12).collect::<Vec<u32>>().chunks(3) {
            for &month in month_chunk {
                let month_date = Local.with_ymd_and_hms(year, month, 1, 0, 0, 0).unwrap();
                render_mini_month(ui, state, month_date, theme);
            }
            ui.end_row();
        }
    });
}

pub fn render_mini_month(
    ui: &mut Ui,
    state: &mut CalendarState,
    date: chrono::DateTime<Local>,
    theme: &Theme,
) {
    let month_name = date.format("%B").to_string();
    let current_date = Local::now().date_naive();

    Frame::default()
        .stroke(Stroke::new(1.0, theme.border))
        .fill(theme.panel)
        .show(ui, |ui| {
            ui.vertical(|ui| {
                ui.heading(RichText::new(month_name).color(theme.header_text));

                Grid::new(format!("month_grid_{}", date.month()))
                    .spacing([4.0, 4.0])
                    .show(ui, |ui| {
                        for day in ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"] {
                            ui.label(RichText::new(day).color(theme.secondary_text).small());
                        }
                        ui.end_row();

                        let first_day = date.with_day(1).unwrap();
                        let weekday_offset = first_day.weekday().num_days_from_monday() as i64;
                        let mut current_day = first_day - Duration::days(weekday_offset);

                        for _ in 0..6 {
                            for _ in 0..7 {
                                let day_num = current_day.day();
                                let is_current_month = current_day.month() == date.month();
                                let is_today = current_day.date_naive() == current_date;
                                let current_date_naive = current_day.date_naive();

                                let text = style_day_text(
                                    day_num,
                                    is_current_month,
                                    is_today,
                                    current_date_naive,
                                    state,
                                    theme,
                                );

                                let response = ui.add(Button::new(text).frame(false));

                                if is_today {
                                    let rect = response.rect;
                                    ui.painter().circle(
                                        rect.center(),
                                        rect.height() / 2.0,
                                        theme.hover,
                                        Stroke::new(1.0, theme.error),
                                    );
                                }

                                if response.clicked() {
                                    state.selected_date = current_day;
                                    state.view_mode = CalendarViewMode::Day;
                                }

                                current_day += Duration::days(1);
                            }
                            ui.end_row();
                        }
                    });
            });
        });
}

pub fn render_month_view(ui: &mut Ui, state: &mut CalendarState, theme: &Theme) {
    let selected_date = state.selected_date;
    let current_date = Local::now().date_naive();
    let first_day = selected_date
        .with_day(1)
        .unwrap()
        .with_hour(0)
        .unwrap()
        .with_minute(0)
        .unwrap()
        .with_second(0)
        .unwrap();
    let weekday_offset = first_day.weekday().num_days_from_monday() as i64;

    ui.vertical(|ui| {
        super::navigation::render_navigation_header(ui, state, theme);

        ui.horizontal(|ui| {
            for day in ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"] {
                ui.label(RichText::new(day).strong().color(theme.text));
            }
        });

        let mut current_day = first_day - Duration::days(weekday_offset);
        for _week in 0..6 {
            ui.horizontal(|ui| {
                for _day in 0..7 {
                    let day_num = current_day.day();
                    let is_current_month = current_day.month() == selected_date.month();
                    let is_today = current_day.date_naive() == current_date;
                    let current_date_naive = current_day.date_naive();

                    let text = style_day_text(
                        day_num,
                        is_current_month,
                        is_today,
                        current_date_naive,
                        state,
                        theme,
                    );

                    let response = ui.add(Button::new(text).frame(false));

                    if is_today {
                        let rect = response.rect;
                        ui.painter().circle(
                            rect.center(),
                            rect.height() / 2.0,
                            theme.hover,
                            Stroke::new(1.0, theme.error),
                        );
                    }

                    if response.clicked() {
                        state.selected_date = current_day;
                        state.view_mode = CalendarViewMode::Day;
                    }

                    current_day += Duration::days(1);
                }
            });
        }
    });
}

pub fn render_week_view(ui: &mut Ui, state: &mut CalendarState, theme: &Theme) {
    let selected_date = state.selected_date;
    let current_date = Local::now().date_naive();
    let start_of_week =
        selected_date - Duration::days(selected_date.weekday().num_days_from_monday() as i64);

    ui.vertical(|ui| {
        super::navigation::render_navigation_header(ui, state, theme);

        ScrollArea::horizontal().show(ui, |ui| {
            let available_width = ui.available_width();
            let time_col_width = 60.0;
            let day_width = (available_width - time_col_width) / 7.0;
            let hour_height = 60.0;

            Grid::new("week_grid")
                .spacing([1.0, 1.0])
                .min_col_width(day_width)
                .min_row_height(hour_height)
                .show(ui, |ui| {
                    ui.label("");
                    for day_offset in 0..7 {
                        let current_day = start_of_week + Duration::days(day_offset);
                        let is_today = current_day.date_naive() == current_date;
                        let day_text = RichText::new(format!("{}", current_day.format("%a %d")))
                            .color(if is_today { theme.error } else { theme.text })
                            .strong();
                        ui.label(day_text);
                    }
                    ui.end_row();

                    for hour in 8..21 {
                        ui.label(
                            RichText::new(format!("{:02}:00", hour)).color(theme.secondary_text),
                        );

                        for day_offset in 0..7 {
                            let current_day = start_of_week + Duration::days(day_offset);
                            let current_date_naive = current_day.date_naive();

                            render_time_slot(
                                ui,
                                state,
                                current_date_naive,
                                hour as u32,
                                day_width,
                                hour_height,
                                theme,
                            );
                        }
                        ui.end_row();
                    }
                });
        });
    });
}

pub fn render_day_view(ui: &mut Ui, state: &mut CalendarState, theme: &Theme) {
    let selected_date = state.selected_date;
    let selected_date_naive = selected_date.date_naive();
    let _current_date = Local::now().date_naive();

    ui.vertical(|ui| {
        super::navigation::render_navigation_header(ui, state, theme);

        ScrollArea::both().show(ui, |ui| {
            ui.vertical(|ui| {
                ui.set_width(ui.available_width());
                ui.horizontal(|ui| {
                    ui.label(RichText::new("All Day").color(theme.secondary_text));

                    if let Some(day_events) = state.event_map.get(&selected_date_naive) {
                        let all_day_events: Vec<_> = day_events
                            .iter()
                            .filter(|e| e.duration_minutes() / 60 >= 24)
                            .collect();

                        for event in all_day_events {
                            let event_button = Button::new(
                                RichText::new(&event.title).color(theme.panel).strong(),
                            )
                            .fill(event.color)
                            .min_size(egui::Vec2::new(ui.available_width() - 100.0, 30.0));

                            if ui.add(event_button).clicked() {
                                state.selected_event = Some(event.id);
                            }
                        }
                    }
                });
                ui.separator();

                let hour_height = 60.0;
                for hour in 8..21 {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(format!("{:02}:00", hour)).color(theme.secondary_text),
                        );

                        render_time_slot(
                            ui,
                            state,
                            selected_date_naive,
                            hour as u32,
                            ui.available_width() - 100.0,
                            hour_height,
                            theme,
                        );
                    });
                }
            });
        });
    });
}

fn style_day_text(
    day_num: u32,
    is_current_month: bool,
    is_today: bool,
    current_date_naive: NaiveDate,
    state: &CalendarState,
    theme: &Theme,
) -> RichText {
    let mut text = RichText::new(format!("{:2}", day_num)).small();
    let has_events = state.event_map.get(&current_date_naive).is_some();

    if is_today {
        text = text.color(theme.error);
    } else if has_events && is_current_month {
        text = text.color(theme.accent).strong();
    } else if is_current_month {
        text = text.color(theme.text);
    } else {
        text = text.color(theme.secondary_text);
    }

    text
}

fn render_time_slot(
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
        .inner_margin(Margin::same(2))
        .fill(theme.panel)
        .show(ui, |ui| {
            if let Some(day_events) = state.event_map.get(&date) {
                let hour_events: Vec<_> = day_events
                    .iter()
                    .filter(|e| e.start_time.hour() == hour)
                    .collect();

                if !hour_events.is_empty() {
                    for event in hour_events {
                        let duration_mins = event.duration_minutes();
                        let event_height = (duration_mins as f32 / 60.0) * height;

                        let event_button =
                            Button::new(RichText::new(&event.title).color(theme.panel).strong())
                                .fill(event.color)
                                .min_size(egui::Vec2::new(
                                    width - 4.0,
                                    event_height.min(height - 4.0),
                                ));

                        if ui.add(event_button).clicked() {
                            state.selected_event = Some(event.id);
                        }
                    }
                } else {
                    let response =
                        ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::click());
                    if response.clicked() {
                        crate::ui::features::calendar::Calendar::handle_time_slot_click_state(
                            state, date, hour,
                        );
                    }
                }
            } else {
                let response =
                    ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::click());
                if response.clicked() {
                    crate::ui::features::calendar::Calendar::handle_time_slot_click_state(
                        state, date, hour,
                    );
                }
            }
        });
}
