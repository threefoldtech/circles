use chrono::{Datelike, Duration, Local, Timelike};
use egui::{Button, Frame, Grid, RichText, Stroke, Ui, Vec2};

use super::super::state::{CalendarState, CalendarViewMode};
use super::common::style_day_text;
use crate::utils::config::Theme;

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
        ui.set_width(ui.available_width());
        super::navigation::render_navigation_header(ui, state, theme);

        Frame::default()
            .fill(theme.background)
            .outer_margin(8.0)
            .show(ui, |ui| {
                Grid::new("month_calendar_grid")
                    .spacing([4.0, 8.0])
                    .min_col_width(ui.available_width() / 7.0 - 4.0)
                    .show(ui, |ui| {
                        for day in ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"] {
                            ui.add(
                                egui::Label::new(
                                    RichText::new(day)
                                        .strong()
                                        .color(theme.header_text)
                                        .size(14.0),
                                )
                                .wrap(),
                            );
                        }
                        ui.end_row();

                        let mut current_day = first_day - Duration::days(weekday_offset);
                        for _week in 0..6 {
                            for _day in 0..7 {
                                let day_num = current_day.day();
                                let is_current_month = current_day.month() == selected_date.month();
                                let is_today = current_day.date_naive() == current_date;
                                let current_date_naive = current_day.date_naive();
                                let has_events = state.event_map.get(&current_date_naive).is_some();

                                Frame::default()
                                    .fill(if is_today {
                                        theme.hover.linear_multiply(0.7)
                                    } else if is_current_month {
                                        theme.panel
                                    } else {
                                        theme.secondary_background
                                    })
                                    .stroke(if is_today {
                                        Stroke::new(1.5, theme.accent)
                                    } else {
                                        Stroke::new(0.5, theme.border)
                                    })
                                    .corner_radius(4.0)
                                    .inner_margin(4.0)
                                    .show(ui, |ui| {
                                        ui.set_min_height(60.0);

                                        let text = style_day_text(
                                            day_num,
                                            is_current_month,
                                            is_today,
                                            current_date_naive,
                                            state,
                                            theme,
                                        );

                                        let response = ui.add(
                                            Button::new(text)
                                                .frame(false)
                                                .min_size(Vec2::new(ui.available_width(), 20.0)),
                                        );

                                        if has_events {
                                            ui.painter().circle_filled(
                                                egui::pos2(
                                                    response.rect.right() - 10.0,
                                                    response.rect.center().y,
                                                ),
                                                4.0,
                                                theme.accent,
                                            );
                                        }

                                        if response.clicked() {
                                            state.selected_date = current_day;
                                            state.view_mode = CalendarViewMode::Day;
                                        }
                                    });

                                current_day += Duration::days(1);
                            }
                            ui.end_row();
                        }
                    });
            });
    });
}
