use chrono::{Datelike, Duration, Local, NaiveDate, TimeZone, Timelike};
use egui::{Button, Frame, Grid, Margin, RichText, ScrollArea, Stroke, Ui, Vec2};

use super::super::state::{CalendarState, CalendarViewMode};
use crate::utils::config::Theme;

pub fn render_year_view(ui: &mut Ui, state: &mut CalendarState, theme: &Theme) {
    let year = state.selected_date.year();

    // Set full width for the calendar
    ui.set_width(ui.available_width());

    // Render the navigation header (now centered)
    super::navigation::render_navigation_header(ui, state, theme);

    // Create a container for the year view
    egui::Frame::new()
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

            // Create a scrollable area for the months grid
            ScrollArea::both()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    // Use a grid layout with responsive spacing
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

    // Create a styled frame for each month with adequate padding
    Frame::default()
        .stroke(Stroke::new(1.0, theme.border))
        .fill(theme.panel)
        .inner_margin(16.0) // Add padding inside each month box
        .corner_radius(6.0)
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                // Center the month name
                ui.heading(
                    RichText::new(month_name)
                        .color(theme.header_text)
                        .size(18.0)
                        .strong(),
                );

                ui.add_space(8.0);

                // Grid for days of the week and calendar days
                Grid::new(format!("month_grid_{}", date.month()))
                    .spacing([8.0, 8.0]) // Increase spacing for better touch targets
                    .show(ui, |ui| {
                        // Day headers centered
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

                                // Create a frame for each day cell for hover effects
                                let day_frame = egui::Frame::new()
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

                                        // Add hover effect
                                        if response.hovered() && !is_today {
                                            ui.painter().rect_filled(
                                                response.rect,
                                                4.0,
                                                theme.hover.linear_multiply(0.3),
                                            );
                                        }

                                        // Show event indicator
                                        if has_events && is_current_month {
                                            let rect = response.rect;
                                            let indicator_radius = 2.0;
                                            ui.painter().circle_filled(
                                                egui::pos2(rect.center().x, rect.bottom() - 2.0),
                                                indicator_radius,
                                                theme.accent,
                                            );
                                        }

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
        // Set full width for the calendar
        ui.set_width(ui.available_width());

        // Render the navigation header (now centered)
        super::navigation::render_navigation_header(ui, state, theme);

        // Create a frame for the calendar grid
        egui::Frame::new()
            .fill(theme.background)
            .outer_margin(8.0)
            .show(ui, |ui| {
                // Use Grid for better alignment of calendar days
                egui::Grid::new("month_calendar_grid")
                    .spacing([4.0, 8.0])
                    .min_col_width(ui.available_width() / 7.0 - 4.0) // Responsive column width
                    .show(ui, |ui| {
                        // Day headers
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

                        // Calendar days
                        let mut current_day = first_day - Duration::days(weekday_offset);
                        for _week in 0..6 {
                            for _day in 0..7 {
                                let day_num = current_day.day();
                                let is_current_month = current_day.month() == selected_date.month();
                                let is_today = current_day.date_naive() == current_date;
                                let current_date_naive = current_day.date_naive();
                                let has_events = state.event_map.get(&current_date_naive).is_some();

                                // Create a frame for each day cell
                                egui::Frame::new()
                                    .fill(if is_today {
                                        theme.hover.linear_multiply(0.7) // Subtle highlight for today
                                    } else if is_current_month {
                                        theme.panel
                                    } else {
                                        theme.secondary_background
                                    })
                                    .stroke(if is_today {
                                        Stroke::new(1.5, theme.accent) // Theme-based highlight
                                    } else {
                                        Stroke::new(0.5, theme.border)
                                    })
                                    .corner_radius(4.0)
                                    .inner_margin(4.0)
                                    .show(ui, |ui| {
                                        ui.set_min_height(60.0); // Consistent height for day cells

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

                                        // Show actual events instead of just indicators
                                        if has_events {
                                            if let Some(day_events) =
                                                state.event_map.get(&current_date_naive)
                                            {
                                                // Limit to first 2 events to avoid overcrowding
                                                let events_to_show = day_events.iter().take(2);

                                                for event in events_to_show {
                                                    ui.add_space(2.0);

                                                    // Calculate a reasonable width based on available space
                                                    let available_width = ui.available_width();
                                                    // Calculate width based on event duration
                                                    let duration_mins = event.duration_minutes();

                                                    // Base width on duration but with reasonable limits
                                                    let base_width =
                                                        (duration_mins as f32 / 60.0) * 100.0;
                                                    let max_width =
                                                        (available_width * 0.8).min(200.0);
                                                    // Ensure min_width is never greater than max_width
                                                    let min_width = (80.0_f32).min(max_width);

                                                    let event_width =
                                                        base_width.clamp(min_width, max_width);

                                                    let event_button = Button::new(
                                                        RichText::new(&event.title)
                                                            .color(theme.light_color)
                                                            .size(10.0),
                                                    )
                                                    .fill(event.color)
                                                    .corner_radius(2.0)
                                                    .min_size(egui::Vec2::new(event_width, 16.0));

                                                    if ui.add(event_button).clicked() {
                                                        state.selected_event = Some(event.id);
                                                    }
                                                }

                                                // If there are more events than we're showing
                                                if day_events.len() > 2 {
                                                    ui.add_space(2.0);
                                                    ui.label(
                                                        RichText::new(format!(
                                                            "+{} more",
                                                            day_events.len() - 2
                                                        ))
                                                        .size(9.0)
                                                        .color(theme.secondary_text),
                                                    );
                                                }
                                            }
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
        // Set full width for the calendar
        ui.set_width(ui.available_width());

        // Render the navigation header (now centered)
        super::navigation::render_navigation_header(ui, state, theme);

        // Create a frame for the week view
        egui::Frame::new()
            .fill(theme.background)
            .outer_margin(8.0)
            .show(ui, |ui| {
                // Scrollable area for the week view
                ScrollArea::both()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        let available_width = ui.available_width();
                        let time_col_width = 60.0;
                        let day_width = (available_width - time_col_width) / 7.0;
                        let hour_height = 70.0;

                        // Use Grid for better alignment of days and hours
                        Grid::new("week_grid")
                            .spacing([2.0, 2.0])
                            .min_col_width(day_width)
                            .min_row_height(hour_height)
                            .show(ui, |ui| {
                                // Header row with time column and day names
                                ui.vertical_centered(|ui| {
                                    ui.add_space(8.0);
                                    ui.label(
                                        RichText::new("Time").color(theme.header_text).strong(),
                                    );
                                });

                                // Day headers
                                for day_offset in 0..7 {
                                    let current_day = start_of_week + Duration::days(day_offset);
                                    let is_today = current_day.date_naive() == current_date;

                                    // Create a styled header for each day
                                    egui::Frame::new()
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

                                // Time slots for each hour
                                for hour in 8..21 {
                                    // Time column
                                    ui.vertical_centered(|ui| {
                                        ui.add_space(8.0);
                                        ui.add(egui::Label::new(
                                            RichText::new(format!("{:02}:00", hour))
                                                .color(theme.secondary_text)
                                                .size(14.0),
                                        ));
                                    });

                                    // Day columns with time slots
                                    for day_offset in 0..7 {
                                        let current_day =
                                            start_of_week + Duration::days(day_offset);
                                        let current_date_naive = current_day.date_naive();
                                        let is_today = current_date_naive == current_date;

                                        // Add subtle highlight for today's column
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
    let current_time = Local::now();
    let is_today = selected_date_naive == current_date;

    ui.vertical(|ui| {
        // Set full width for the calendar
        ui.set_width(ui.available_width());

        // Render the navigation header (now centered)
        super::navigation::render_navigation_header(ui, state, theme);

        // Create a frame for the day view
        egui::Frame::new()
            .fill(theme.background)
            .outer_margin(8.0)
            .show(ui, |ui| {
                // Header with date information - more prominent for today (Google Calendar style)
                ui.horizontal_centered(|ui| {
                    if is_today {
                        // Add a colored background for "Today" label (Google Calendar style)
                        let today_label = RichText::new("Today")
                            .color(egui::Color32::WHITE)
                            .size(16.0)
                            .strong();

                        let today_button = egui::Button::new(today_label)
                            .fill(egui::Color32::from_rgb(234, 67, 53)) // Google red
                            .rounding(egui::Rounding::same(12))
                            .min_size(Vec2::new(80.0, 28.0));

                        ui.add(today_button);

                        ui.add_space(12.0);
                    }

                    // Add the date (always shown, more prominent for today)
                    let date_text = selected_date.format("%A, %B %d").to_string();
                    ui.add(egui::Label::new(
                        RichText::new(date_text)
                            .color(theme.header_text)
                            .size(24.0)
                            .strong(),
                    ));
                });

                // Split view with mini-month on left and day view on right (Google Calendar style)
                ui.horizontal(|ui| {
                    // Left sidebar with mini-month (25% width)
                    ui.allocate_ui_with_layout(
                        Vec2::new(ui.available_width() * 0.25, ui.available_height()),
                        egui::Layout::top_down(egui::Align::Center),
                        |ui| {
                            // Mini month view
                            egui::Frame::new()
                                .fill(theme.panel)
                                .stroke(Stroke::new(1.0, theme.border))
                                .corner_radius(8.0)
                                .outer_margin(4.0)
                                .inner_margin(8.0)
                                .show(ui, |ui| {
                                    render_mini_month(ui, state, selected_date, theme);
                                });

                            ui.add_space(16.0);

                            // Upcoming events section
                            egui::Frame::new()
                                .fill(theme.panel)
                                .stroke(Stroke::new(1.0, theme.border))
                                .corner_radius(8.0)
                                .outer_margin(4.0)
                                .inner_margin(8.0)
                                .show(ui, |ui| {
                                    ui.heading(
                                        RichText::new("Upcoming")
                                            .color(theme.header_text)
                                            .size(16.0),
                                    );
                                    ui.add_space(8.0);

                                    // Get events for the next 7 days
                                    let mut upcoming_events = Vec::new();
                                    for i in 0..7 {
                                        let date = current_date + Duration::days(i);
                                        if let Some(events) = state.event_map.get(&date) {
                                            for event in events {
                                                upcoming_events.push((date, event));
                                            }
                                        }
                                    }

                                    // Sort by date and time
                                    upcoming_events.sort_by(|a, b| {
                                        a.0.cmp(&b.0)
                                            .then_with(|| a.1.start_time.cmp(&b.1.start_time))
                                    });

                                    // Display upcoming events (limit to 5)
                                    for (date, event) in upcoming_events.iter().take(5) {
                                        let is_event_today = *date == current_date;

                                        egui::Frame::new()
                                            .fill(theme.secondary_background)
                                            .corner_radius(4.0)
                                            .outer_margin(2.0)
                                            .inner_margin(6.0)
                                            .show(ui, |ui| {
                                                ui.horizontal(|ui| {
                                                    // Color dot for event
                                                    let (rect, _) = ui.allocate_exact_size(
                                                        Vec2::new(8.0, 8.0),
                                                        egui::Sense::hover(),
                                                    );
                                                    ui.painter().circle_filled(
                                                        rect.center(),
                                                        4.0,
                                                        event.color,
                                                    );

                                                    ui.add_space(4.0);

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
                                                                date.format("%a"),
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

                                        ui.add_space(4.0);
                                    }

                                    if upcoming_events.is_empty() {
                                        ui.label(
                                            RichText::new("No upcoming events")
                                                .color(theme.secondary_text)
                                                .italics(),
                                        );
                                    }
                                });
                        },
                    );

                    ui.add_space(8.0);

                    // Right side with day view (75% width)
                    ui.vertical(|ui| {
                        ui.set_width(ui.available_width());

                        // Scrollable area for the day's events
                        ScrollArea::both()
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                ui.set_width(ui.available_width());

                                // All-day events section
                                egui::Frame::new()
                                    .fill(theme.panel)
                                    .stroke(Stroke::new(1.0, theme.border))
                                    .corner_radius(4.0)
                                    .outer_margin(4.0)
                                    .inner_margin(8.0)
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            ui.add(egui::Label::new(
                                                RichText::new("All Day")
                                                    .color(theme.header_text)
                                                    .size(14.0)
                                                    .strong(),
                                            ));

                                            ui.add_space(16.0);

                                            if let Some(day_events) =
                                                state.event_map.get(&selected_date_naive)
                                            {
                                                let all_day_events: Vec<_> = day_events
                                                    .iter()
                                                    .filter(|e| e.duration_minutes() / 60 >= 24)
                                                    .collect();

                                                for event in all_day_events {
                                                    // Calculate width based on event duration
                                                    let duration_mins = event.duration_minutes();
                                                    let available_width = ui.available_width();

                                                    // For all-day events, use a reasonable width
                                                    let event_width = if duration_mins >= 24 * 60 {
                                                        // All-day event - use a reasonable fixed width
                                                        (available_width * 0.6).min(250.0)
                                                    } else {
                                                        // Regular event - base width on duration
                                                        let base_width = (duration_mins as f32
                                                            / (24.0 * 60.0))
                                                            * available_width;
                                                        let max_width = available_width * 0.8;
                                                        // Ensure min_width is never greater than max_width
                                                        let min_width = (150.0_f32).min(max_width);
                                                        base_width.clamp(min_width, max_width)
                                                    };

                                                    let event_button = Button::new(
                                                        RichText::new(&event.title)
                                                            .color(theme.light_color)
                                                            .strong(),
                                                    )
                                                    .fill(event.color)
                                                    .corner_radius(4.0)
                                                    .min_size(egui::Vec2::new(event_width, 30.0));
                                                    if ui.add(event_button).clicked() {
                                                        state.selected_event = Some(event.id);
                                                    }
                                                }
                                            }
                                        });
                                    });

                                ui.add_space(8.0);

                                // Hourly events section with current time indicator
                                egui::Grid::new("day_view_grid")
                                    .spacing([8.0, 0.0])
                                    .min_col_width(60.0)
                                    .show(ui, |ui| {
                                        let hour_height = 70.0;

                                        // Extend hours to match Google Calendar (5am to 11pm)
                                        for hour in 5..24 {
                                            // Time column
                                            ui.vertical(|ui| {
                                                ui.add_space(8.0);
                                                ui.add(egui::Label::new(
                                                    RichText::new(format!("{:02}:00", hour))
                                                        .color(theme.secondary_text)
                                                        .size(14.0),
                                                ));
                                            });

                                            // Event column
                                            ui.vertical(|ui| {
                                                // Get the rect for current time indicator
                                                let row_rect = ui.available_rect_before_wrap();

                                                render_time_slot(
                                                    ui,
                                                    state,
                                                    selected_date_naive,
                                                    hour as u32,
                                                    ui.available_width(),
                                                    hour_height,
                                                    theme,
                                                );

                                                // Draw current time indicator if this is today and we're in the current hour
                                                if is_today && hour == current_time.hour() {
                                                    let current_minute_fraction =
                                                        current_time.minute() as f32 / 60.0;
                                                    let y_offset = row_rect.top()
                                                        + (hour_height * current_minute_fraction);

                                                    // Draw the red line for current time (Google Calendar style)
                                                    let line_start = egui::pos2(
                                                        row_rect.left() - 10.0,
                                                        y_offset,
                                                    );
                                                    let line_end =
                                                        egui::pos2(row_rect.right(), y_offset);

                                                    // Draw circle at start of line (Google Calendar style)
                                                    ui.painter().circle_filled(
                                                        line_start,
                                                        6.0,
                                                        egui::Color32::from_rgb(234, 67, 53), // Google red
                                                    );

                                                    // Draw the line with animation effect (Google Calendar style)
                                                    ui.painter().line_segment(
                                                        [line_start, line_end],
                                                        (2.5, egui::Color32::from_rgb(234, 67, 53)), // Google red
                                                    );

                                                    // Add current time text
                                                    let time_text = format!(
                                                        "{:02}:{:02}",
                                                        current_time.hour(),
                                                        current_time.minute()
                                                    );
                                                    ui.painter().text(
                                                        egui::pos2(
                                                            line_start.x + 10.0,
                                                            line_start.y - 10.0,
                                                        ),
                                                        egui::Align2::LEFT_BOTTOM,
                                                        time_text,
                                                        egui::FontId::proportional(11.0),
                                                        egui::Color32::from_rgb(234, 67, 53),
                                                    );
                                                }
                                            });

                                            ui.end_row();
                                        }
                                    });
                            });
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
    // Start with base formatting
    let mut text = RichText::new(format!("{:2}", day_num)).size(14.0);
    let has_events = state.event_map.get(&current_date_naive).is_some();

    // Apply theme-based styling based on day status
    if is_today {
        // Use primary theme color for today
        text = text.color(theme.accent).strong();
    } else if has_events && is_current_month {
        // Use accent color for days with events
        text = text.color(theme.accent).strong();
    } else if is_current_month {
        // Use regular text color for current month
        text = text.color(theme.text);
    } else {
        // Use secondary text color for days outside current month
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
    // Create a frame for the time slot with theme-based styling
    Frame::default()
        .stroke(Stroke::new(0.5, theme.border))
        .inner_margin(Margin::same(4))
        .corner_radius(4.0)
        .fill(theme.panel)
        .show(ui, |ui| {
            if let Some(day_events) = state.event_map.get(&date) {
                let hour_events: Vec<_> = day_events
                    .iter()
                    .filter(|e| {
                        // Convert UTC time to local time before comparing hours
                        let local_time = e.start_time.with_timezone(&chrono::Local);
                        local_time.hour() == hour
                    })
                    .collect();

                if !hour_events.is_empty() {
                    for event in hour_events {
                        let duration_mins = event.duration_minutes();
                        let event_height = (duration_mins as f32 / 60.0) * height;

                        // Create a styled event button with theme colors
                        // Calculate width based on duration
                        let duration_hours = duration_mins as f32 / 60.0;

                        // For hourly events, use a width proportional to duration
                        // but with reasonable limits
                        let base_width = (duration_hours * 80.0).max(60.0);
                        let max_width = (width * 0.8).max(60.0); // Ensure max_width is at least 60.0
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

                        // Add hover effect
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
                    // Empty time slot with click handling
                    let mut response =
                        ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::click());

                    // Add hover effect for empty slots
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
                // Empty time slot with click handling
                let mut response =
                    ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::click());

                // Add hover effect for empty slots
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
