use chrono::{Datelike, Duration, Local, TimeZone};
use egui::{Button, Frame, Grid, RichText, ScrollArea, Stroke, Ui, Vec2};

use super::super::state::{CalendarState, CalendarViewMode};
use super::common::style_day_text;
use crate::utils::config::Theme;

pub fn render_year_view(ui: &mut Ui, state: &mut CalendarState, theme: &Theme) {
    let year = state.selected_date.year();

    // Set full width for the calendar
    ui.set_width(ui.available_width());

    // Render the navigation header
    super::navigation::render_navigation_header(ui, state, theme);

    // Create a container for the year view
    Frame::default()
        .fill(theme.background)
        .outer_margin(8.0)
        .show(ui, |ui| {
            // Year header
            ui.vertical_centered(|ui| {
                ui.heading(
                    RichText::new(format!("Calendar {}", year))
                        .color(theme.header_text)
                        .size(24.0)
                        .strong(),
                );
                ui.add_space(16.0);
            });
            
            // Scrollable area for the months
            ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    // Get the total available width
                    let available_width = ui.available_width();
                    
                    // Divide the available width into three equal parts
                    let column_width = (available_width / 3.0) - 16.0;
                    
                    // Render each row of months
                    for row in 0..4 {
                        ui.horizontal(|ui| {
                            ui.set_width(available_width);
                            
                            // First column
                            let month1 = row * 3 + 1;
                            if month1 <= 12 {
                                let month_date = Local.with_ymd_and_hms(year, month1 as u32, 1, 0, 0, 0).unwrap();
                                ui.scope(|ui| {
                                    ui.set_width(column_width);
                                    render_mini_month(ui, state, month_date, theme, column_width);
                                });
                            }
                            
                            ui.add_space(16.0);
                            
                            // Second column
                            let month2 = row * 3 + 2;
                            if month2 <= 12 {
                                let month_date = Local.with_ymd_and_hms(year, month2 as u32, 1, 0, 0, 0).unwrap();
                                ui.scope(|ui| {
                                    ui.set_width(column_width);
                                    render_mini_month(ui, state, month_date, theme, column_width);
                                });
                            }
                            
                            ui.add_space(16.0);
                            
                            // Third column
                            let month3 = row * 3 + 3;
                            if month3 <= 12 {
                                let month_date = Local.with_ymd_and_hms(year, month3 as u32, 1, 0, 0, 0).unwrap();
                                ui.scope(|ui| {
                                    ui.set_width(column_width);
                                    render_mini_month(ui, state, month_date, theme, column_width);
                                });
                            }
                        });
                        
                        // Add spacing between rows
                        ui.add_space(24.0);
                    }
                });
        });
}

pub fn render_mini_month(
    ui: &mut Ui,
    state: &mut CalendarState,
    date: chrono::DateTime<Local>,
    theme: &Theme,
    width: f32,
) {
    let month_name = date.format("%B").to_string();
    let current_date = Local::now().date_naive();

    // Styled frame for each month
    Frame::default()
        .stroke(Stroke::new(1.0, theme.border))
        .fill(theme.panel)
        .inner_margin(8.0)
        .corner_radius(6.0)
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                // Month name
                ui.heading(
                    RichText::new(month_name)
                        .color(theme.header_text)
                        .size(16.0)
                        .strong(),
                );
                ui.add_space(4.0);

                // Calculate responsive sizes based on available width
                let available_width = ui.available_width();
                let cell_size = (available_width / 7.0).min(40.0).max(16.0);
                let grid_spacing = 2.0;

                // Grid for days
                Grid::new(format!("month_grid_{}", date.month()))
                    .spacing([grid_spacing, grid_spacing])
                    .min_col_width(cell_size)
                    .max_col_width(cell_size)
                    .show(ui, |ui| {
                        // Day headers
                        for day in ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"] {
                            ui.centered_and_justified(|ui| {
                                ui.label(RichText::new(day).color(theme.secondary_text).size(12.0));
                            });
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
                                let has_events = state.event_map.get(&current_date_naive).is_some();

                                // Day cell frame
                                let day_frame = Frame::default()
                                    .fill(if is_today {
                                        theme.hover.linear_multiply(0.7)
                                    } else {
                                        theme.transparent
                                    })
                                    .corner_radius(4.0)
                                    .inner_margin(1.0);

                                day_frame.show(ui, |ui| {
                                    ui.centered_and_justified(|ui| {
                                        let text = style_day_text(
                                            day_num,
                                            is_current_month,
                                            is_today,
                                            current_date_naive,
                                            state,
                                            theme,
                                        );

                                        // Use responsive size for day buttons with fixed constraints
                                        let button_size = Vec2::new(cell_size - 2.0, cell_size - 2.0);
                                        let response = ui.add(
                                            Button::new(text)
                                                .frame(false)
                                                .min_size(button_size)
                                                // .max_size(button_size),
                                        );

                                        // Hover effect
                                        if response.hovered() && !is_today {
                                            ui.painter().rect_filled(
                                                response.rect,
                                                4.0,
                                                theme.hover.linear_multiply(0.3),
                                            );
                                        }

                                        // Event indicator
                                        if has_events && is_current_month {
                                            let rect = response.rect;
                                            ui.painter().circle_filled(
                                                egui::pos2(rect.center().x, rect.bottom() - 2.0),
                                                2.0,
                                                theme.accent,
                                            );
                                        }

                                        // Today indicator
                                        if is_today {
                                            let rect = response.rect;
                                            ui.painter().circle_stroke(
                                                rect.center(),
                                                rect.height() / 2.0,
                                                Stroke::new(1.5, theme.accent),
                                            );
                                        }

                                        if response.clicked() {
                                            state.selected_date = current_day;
                                            state.view_mode = CalendarViewMode::Day;
                                        }
                                    });
                                });

                                current_day += Duration::days(1);
                            }
                            ui.end_row();
                        }
                    });
            });
        });
}
