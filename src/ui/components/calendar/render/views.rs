use chrono::{Datelike, Duration, Local, NaiveDate, TimeZone, Timelike};
use egui::{Button, Frame, Grid, Margin, RichText, ScrollArea, Stroke, Ui, Vec2};

use super::super::state::{CalendarState, CalendarViewMode};
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

            // Scrollable area for the months grid
            ScrollArea::both()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    Grid::new("year_grid")
                        .spacing([24.0, 24.0])
                        .min_col_width(ui.available_width() / 3.0 - 24.0)
                        .show(ui, |ui| {
                            for month_chunk in (1..=12).collect::<Vec<u32>>().chunks(3) {
                                for &month in month_chunk {
                                    let month_date =
                                        Local.with_ymd_and_hms(year, month, 1, 0, 0, 0).unwrap();
                                    render_mini_month(ui, state, month_date, theme);
                                }
                                ui.end_row();
                            }
                        });
                });
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

    // Styled frame for each month
    Frame::default()
        .stroke(Stroke::new(1.0, theme.border))
        .fill(theme.panel)
        .inner_margin(16.0)
        .corner_radius(6.0)
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                // Month name
                ui.heading(
                    RichText::new(month_name)
                        .color(theme.header_text)
                        .size(18.0)
                        .strong(),
                );
                ui.add_space(8.0);

                // Grid for days
                Grid::new(format!("month_grid_{}", date.month()))
                    .spacing([8.0, 8.0])
                    .show(ui, |ui| {
                        // Day headers
                        for day in ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"] {
                            ui.centered_and_justified(|ui| {
                                ui.label(RichText::new(day).color(theme.secondary_text).size(14.0));
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
                                        egui::Color32::TRANSPARENT
                                    })
                                    .corner_radius(4.0)
                                    .inner_margin(4.0);

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

                                        let response = ui.add(
                                            Button::new(text)
                                                .frame(false)
                                                .min_size(Vec2::new(24.0, 24.0)),
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

fn style_day_text(
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
                        .min_size(egui::Vec2::new(event_width, event_height.min(height - 8.0)));

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
