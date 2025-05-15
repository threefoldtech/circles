use chrono::{Datelike, Duration, Local};
use egui::{Frame, Grid, RichText, ScrollArea, Ui};

use super::super::state::CalendarState;
use super::common::render_time_slot;
use crate::utils::config::Theme;

pub fn render_week_view(ui: &mut Ui, state: &mut CalendarState, theme: &Theme) {
    let selected_date = state.selected_date;
    let current_date = Local::now().date_naive();
    let start_of_week =
        selected_date - Duration::days(selected_date.weekday().num_days_from_monday() as i64);

    ui.vertical(|ui| {
        ui.set_width(ui.available_width());
        super::navigation::render_navigation_header(ui, state, theme);

        Frame::default()
            .fill(theme.background)
            .outer_margin(8.0)
            .show(ui, |ui| {
                ScrollArea::both()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        let available_width = ui.available_width();
                        let time_col_width = 60.0;
                        let day_width = (available_width - time_col_width) / 7.0;
                        let hour_height = 70.0;

                        Grid::new("week_grid")
                            .spacing([2.0, 2.0])
                            .min_col_width(day_width)
                            .min_row_height(hour_height)
                            .show(ui, |ui| {
                                ui.vertical_centered(|ui| {
                                    ui.add_space(8.0);
                                    ui.label(
                                        RichText::new("Time").color(theme.header_text).strong(),
                                    );
                                });

                                for day_offset in 0..7 {
                                    let current_day = start_of_week + Duration::days(day_offset);
                                    let is_today = current_day.date_naive() == current_date;

                                    Frame::default()
                                        .fill(if is_today { theme.hover } else { theme.panel })
                                        .corner_radius(4.0)
                                        .show(ui, |ui| {
                                            ui.vertical_centered(|ui| {
                                                ui.add_space(4.0);
                                                ui.add(egui::Label::new(
                                                    RichText::new(format!(
                                                        "{}",
                                                        current_day.format("%a %d")
                                                    ))
                                                    .color(if is_today {
                                                        theme.accent
                                                    } else {
                                                        theme.text
                                                    })
                                                    .size(14.0)
                                                    .strong(),
                                                ));
                                                ui.add_space(4.0);
                                            });
                                        });
                                }
                                ui.end_row();

                                for hour in 8..21 {
                                    ui.vertical_centered(|ui| {
                                        ui.add_space(8.0);
                                        ui.add(egui::Label::new(
                                            RichText::new(format!("{:02}:00", hour))
                                                .color(theme.secondary_text)
                                                .size(14.0),
                                        ));
                                    });

                                    for day_offset in 0..7 {
                                        let current_day =
                                            start_of_week + Duration::days(day_offset);
                                        let current_date_naive = current_day.date_naive();
                                        let is_today = current_date_naive == current_date;

                                        if is_today {
                                            let rect = ui.available_rect_before_wrap();
                                            ui.painter().rect_filled(
                                                rect,
                                                0.0,
                                                theme.hover.linear_multiply(0.3),
                                            );
                                        }

                                        render_time_slot(
                                            ui,
                                            state,
                                            current_date_naive,
                                            hour as u32,
                                            day_width - 4.0,
                                            hour_height - 4.0,
                                            theme,
                                        );
                                    }
                                    ui.end_row();
                                }
                            });
                    });
            });
    });
}
