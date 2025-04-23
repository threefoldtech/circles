use chrono::{DateTime, Datelike, Duration, Local, NaiveDate, NaiveTime, TimeZone, Timelike, Utc};
use eframe::egui::{self, Color32, RichText, Stroke, Ui, Vec2};
use egui::Margin;
use std::collections::BTreeMap;
use uuid::Uuid;

use crate::utils::config::Theme;

#[derive(Debug, Clone)]
pub struct Event {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub location: Option<String>,
    pub attendees: Vec<String>,
    pub color: Color32,
}

impl Event {
    fn duration_minutes(&self) -> i64 {
        (self.end_time - self.start_time).num_minutes()
    }

    fn overlaps_with(&self, other: &Event) -> bool {
        (self.start_time <= other.end_time) && (self.end_time >= other.start_time)
    }
}

impl Default for Event {
    fn default() -> Self {
        // Round current time to nearest half hour
        let now = Utc::now();
        let minutes = now.minute();

        // Calculate rounded time
        let (rounded_minutes, hour_adjustment) = if minutes < 30 { (30, 0) } else { (0, 1) };

        // Apply rounding
        let start_time = now
            .with_minute(rounded_minutes)
            .unwrap()
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap();

        // Apply hour adjustment if needed
        let start_time = start_time + Duration::hours(hour_adjustment);

        Self {
            id: Uuid::new_v4(),
            title: String::new(),
            description: String::new(),
            start_time,
            end_time: start_time + Duration::hours(1),
            location: None,
            attendees: Vec::new(),
            color: Color32::from_rgb(66, 133, 244), // Default blue
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CalendarViewMode {
    Year,
    Month,
    Week,
    Day,
}

impl Default for CalendarViewMode {
    fn default() -> Self {
        Self::Month
    }
}

#[derive(Debug, Clone)]
pub struct CalendarState {
    // Efficient event storage by date
    event_map: BTreeMap<NaiveDate, Vec<Event>>,
    all_events: Vec<Event>,
    selected_date: DateTime<Local>,
    view_mode: CalendarViewMode,
    selected_event: Option<Uuid>,
    show_event_dialog: bool,
    new_event: Option<Event>,
    // For time slot selection
    selected_time_slot: Option<(NaiveDate, NaiveTime)>,
    // For drag selection
    drag_start: Option<(f32, f32)>,
    drag_end: Option<(f32, f32)>,
}

impl Default for CalendarState {
    fn default() -> Self {
        Self {
            event_map: BTreeMap::new(),
            all_events: Vec::new(),
            selected_date: Local::now(),
            view_mode: CalendarViewMode::Month,
            selected_event: None,
            show_event_dialog: false,
            new_event: None,
            selected_time_slot: None,
            drag_start: None,
            drag_end: None,
        }
    }
}

#[derive(Clone)]
pub struct Calendar {
    state: CalendarState,
}

impl Calendar {
    pub fn new() -> Self {
        Self {
            state: CalendarState::default(),
        }
    }

    pub fn with_events(events: Vec<Event>) -> Self {
        let mut calendar = Self::new();
        for event in events {
            calendar.add_event(event);
        }
        calendar
    }

    pub fn add_event(&mut self, event: Event) {
        let date = event.start_time.date_naive();
        self.state
            .event_map
            .entry(date)
            .or_insert_with(Vec::new)
            .push(event.clone());
        self.state.all_events.push(event);
    }

    pub fn get_events(&self) -> &Vec<Event> {
        &self.state.all_events
    }

    pub fn render(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        // Get theme colors from the current style
        let visuals = ui.style().visuals.clone();
        self.render_toolbar(ui, &theme);
        ui.add_space(16.0);

        egui::Frame::new()
            .fill(theme.background)
            .show(ui, |ui| match self.state.view_mode {
                CalendarViewMode::Year => self.render_year_view(ui, &theme),
                CalendarViewMode::Month => self.render_month_view(ui, &theme),
                CalendarViewMode::Week => self.render_week_view(ui, &theme),
                CalendarViewMode::Day => self.render_day_view(ui, &theme),
            });

        if self.state.show_event_dialog {
            self.render_event_dialog(ui, &theme);
        }
    }

    fn create_toolbar_button(&mut self, ui: &mut Ui, text: &str, theme: &Theme) -> egui::Response {
        ui.add(
            egui::Button::new(RichText::new(text).color(theme.light_color))
                .fill(theme.accent)
                .stroke(Stroke::NONE)
                .min_size(Vec2::new(32.0, 32.0))
                .corner_radius(8),
        )
    }

    fn render_toolbar(&mut self, ui: &mut Ui, theme: &Theme) {
        ui.horizontal(|ui| {
            // Define toolbar buttons with their labels
            let toolbar_buttons = ["New Event", "Today", "Day", "Week", "Month", "Year"];

            // Render all buttons
            for &label in toolbar_buttons.iter() {
                if self.create_toolbar_button(ui, label, theme).clicked() {
                    match label {
                        "New Event" => {
                            self.state.new_event = Some(Event::default());
                            self.state.show_event_dialog = true;
                        }
                        "Today" => {
                            self.state.selected_date = Local::now();
                        }
                        "Day" => {
                            self.state.view_mode = CalendarViewMode::Day;
                        }
                        "Week" => {
                            self.state.view_mode = CalendarViewMode::Week;
                        }
                        "Month" => {
                            self.state.view_mode = CalendarViewMode::Month;
                        }
                        "Year" => {
                            self.state.view_mode = CalendarViewMode::Year;
                        }
                        _ => {}
                    }
                }
                ui.add_space(5.0);
            }
        });
    }

    fn get_header_text(&self) -> String {
        match self.state.view_mode {
            CalendarViewMode::Year => self.state.selected_date.format("%Y").to_string(),
            CalendarViewMode::Month => self.state.selected_date.format("%B %Y").to_string(),
            CalendarViewMode::Week => format!(
                "Week of {} {}",
                self.state.selected_date.format("%B"),
                self.state.selected_date.day()
            ),
            CalendarViewMode::Day => self.state.selected_date.format("%B %d, %Y").to_string(),
        }
    }

    fn render_navigation_header(&mut self, ui: &mut Ui, theme: &Theme) {
        let header_text = self.get_header_text();
        let is_today = self.state.selected_date.date_naive() == Local::now().date_naive();

        // Create a layout with three columns: left button, centered text, right button
        ui.horizontal(|ui| {
            // Left column - fixed width for the button
            let left_response = ui.add(
                egui::Button::new(RichText::new("◀").color(theme.accent))
                    .min_size(Vec2::new(32.0, 32.0)),
            );
            if left_response.clicked() {
                self.navigate(false);
            }

            // Middle column - flexible width with centered text
            // Use a centered layout for the middle section
            ui.with_layout(
                egui::Layout::top_down_justified(egui::Align::Center),
                |ui| {
                    // Add flexible space to push the text to the center
                    ui.add_space(8.0);

                    ui.strong(
                        RichText::new(header_text)
                            .color(if is_today {
                                theme.error
                            } else {
                                theme.header_text
                            })
                            .size(16.0),
                    );

                    ui.add_space(8.0);
                },
            );

            // Right column - fixed width for the button
            let right_response = ui.add(
                egui::Button::new(RichText::new("▶").color(theme.accent))
                    .min_size(Vec2::new(32.0, 32.0)),
            );
            if right_response.clicked() {
                self.navigate(true);
            }
        });
        ui.separator();
    }

    // Helper function to handle time slot click and create new event
    fn handle_time_slot_click(&mut self, date: NaiveDate, hour: u32) {
        let naive_time = NaiveTime::from_hms_opt(hour, 0, 0).unwrap();
        self.state.selected_time_slot = Some((date, naive_time));
        self.state.new_event = Some(Event::default());
        self.state.show_event_dialog = true;
    }

    fn navigate(&mut self, forward: bool) {
        // Store the original date for debugging
        let original_date = self.state.selected_date;

        match self.state.view_mode {
            CalendarViewMode::Year => {
                // Navigate by year
                let current_year = self.state.selected_date.year();
                let new_year = if forward {
                    current_year + 1
                } else {
                    current_year - 1
                };
                self.state.selected_date = self
                    .state
                    .selected_date
                    .with_year(new_year)
                    .unwrap_or(self.state.selected_date);

                // Debug output
                println!(
                    "Year navigation: {} -> {} (forward: {})",
                    original_date.format("%Y-%m-%d"),
                    self.state.selected_date.format("%Y-%m-%d"),
                    forward
                );
            }
            CalendarViewMode::Month => {
                // Navigate by month
                let current_month = self.state.selected_date.month();
                let current_year = self.state.selected_date.year();

                let (new_year, new_month) = if forward {
                    if current_month == 12 {
                        (current_year + 1, 1)
                    } else {
                        (current_year, current_month + 1)
                    }
                } else {
                    if current_month == 1 {
                        (current_year - 1, 12)
                    } else {
                        (current_year, current_month - 1)
                    }
                };

                self.state.selected_date = self
                    .state
                    .selected_date
                    .with_year(new_year)
                    .unwrap()
                    .with_month(new_month)
                    .unwrap();

                // Debug output
                println!(
                    "Month navigation: {} -> {} (forward: {})",
                    original_date.format("%Y-%m-%d"),
                    self.state.selected_date.format("%Y-%m-%d"),
                    forward
                );
            }
            CalendarViewMode::Week => {
                // Navigate by week
                let duration = Duration::days(7);
                self.state.selected_date = if forward {
                    self.state.selected_date + duration
                } else {
                    self.state.selected_date - duration
                };

                // Debug output
                println!(
                    "Week navigation: {} -> {} (forward: {})",
                    original_date.format("%Y-%m-%d"),
                    self.state.selected_date.format("%Y-%m-%d"),
                    forward
                );
            }
            CalendarViewMode::Day => {
                // Navigate by day
                let duration = Duration::days(1);
                self.state.selected_date = if forward {
                    self.state.selected_date + duration
                } else {
                    self.state.selected_date - duration
                };

                // Debug output
                println!(
                    "Day navigation: {} -> {} (forward: {})",
                    original_date.format("%Y-%m-%d"),
                    self.state.selected_date.format("%Y-%m-%d"),
                    forward
                );
            }
        }
    }

    fn days_in_month(&self, year: i32, month: u32) -> u32 {
        // Safely handle month transitions
        let next_month = if month == 12 {
            match Local.with_ymd_and_hms(year + 1, 1, 1, 0, 0, 0) {
                chrono::LocalResult::Single(dt) => dt,
                // Fall back to a reasonable default if date is ambiguous or invalid
                _ => return 31,
            }
        } else {
            match Local.with_ymd_and_hms(year, month + 1, 1, 0, 0, 0) {
                chrono::LocalResult::Single(dt) => dt,
                // Fall back to a reasonable default if date is ambiguous or invalid
                _ => return 30,
            }
        };

        // Safely get current month
        match Local.with_ymd_and_hms(year, month, 1, 0, 0, 0) {
            chrono::LocalResult::Single(current_month) => {
                (next_month - current_month).num_days() as u32
            }
            // Fall back to a reasonable default if date is ambiguous or invalid
            _ => 30,
        }
    }

    // Helper function to style day text based on conditions
    fn style_day_text(
        &self,
        day_num: u32,
        is_current_month: bool,
        is_today: bool,
        current_date_naive: NaiveDate,
        theme: &Theme,
    ) -> RichText {
        // Format the day number
        let mut text = RichText::new(format!("{:2}", day_num)).small();

        // Check if there are events on this day
        let has_events = self.state.event_map.get(&current_date_naive).is_some();

        // Apply appropriate color based on conditions
        if is_today {
            text = text.color(theme.error);
        } else if has_events && is_current_month {
            text = text.color(theme.accent);
        } else if is_current_month {
            text = text.color(theme.text);
        } else {
            text = text.color(theme.secondary_text);
        }

        // Make events bold
        if has_events && is_current_month {
            text = text.strong();
        }

        text
    }

    fn render_year_view(&mut self, ui: &mut Ui, theme: &Theme) {
        let year = self.state.selected_date.year();

        egui::Grid::new("year_grid")
            .spacing([20.0, 20.0])
            .show(ui, |ui| {
                for month_chunk in (1..=12).collect::<Vec<u32>>().chunks(3) {
                    for &month in month_chunk {
                        let month_date = Local.with_ymd_and_hms(year, month, 1, 0, 0, 0).unwrap();
                        self.render_mini_month(ui, month_date, theme);
                    }
                    ui.end_row();
                }
            });
    }

    fn render_mini_month(&mut self, ui: &mut Ui, date: DateTime<Local>, theme: &Theme) {
        let month_name = date.format("%B").to_string();
        let current_date = Local::now().date_naive();

        egui::Frame::new()
            .stroke(Stroke::new(1.0, theme.border))
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.heading(RichText::new(month_name).color(theme.header_text));

                    egui::Grid::new(format!("month_{}", date.month()))
                        .spacing([4.0, 4.0])
                        .show(ui, |ui| {
                            // Weekday headers
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

                                    // Style the day text
                                    let text = self.style_day_text(
                                        day_num,
                                        is_current_month,
                                        is_today,
                                        current_date_naive,
                                        theme,
                                    );

                                    let response = ui.add(egui::Button::new(text).frame(false));

                                    // Highlight today with a circle
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
                                        self.state.selected_date = current_day;
                                        self.state.view_mode = CalendarViewMode::Day;
                                    }

                                    current_day = current_day + Duration::days(1);
                                }
                                ui.end_row();
                            }
                        });
                });
            });
    }

    fn render_event_dialog(&mut self, ui: &mut Ui, theme: &Theme) {
        egui::Window::new("Event Details")
            .collapsible(false)
            .resizable(false)
            .show(ui.ctx(), |ui| {
                if let Some(event) = &self.state.new_event {
                    let mut event = event.clone(); // Clone the event

                    // If we have a selected time slot, use it for the new event
                    if let Some((date, time)) = self.state.selected_time_slot.take() {
                        if event.start_time.date_naive() != date || event.start_time.time() != time
                        {
                            // Convert to UTC for storage
                            if let Some(local_dt) =
                                Local.from_local_datetime(&date.and_time(time)).single()
                            {
                                event.start_time = local_dt.with_timezone(&Utc);
                                event.end_time = event.start_time + Duration::hours(1);
                            }
                        }
                    }

                    ui.vertical(|ui| {
                        // Title
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Title:").color(theme.text));
                            ui.text_edit_singleline(&mut event.title);
                        });
                        ui.add_space(8.0);

                        // Description
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Description:").color(theme.text));
                            ui.text_edit_multiline(&mut event.description);
                        });
                        ui.add_space(8.0);

                        // Start Time
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Start Time:").color(theme.text));
                            let mut start_date = event.start_time.format("%Y-%m-%d").to_string();
                            let mut start_time = event.start_time.format("%H:%M").to_string();
                            ui.text_edit_singleline(&mut start_date);
                            ui.text_edit_singleline(&mut start_time);
                            if let Ok(parsed_date) =
                                chrono::NaiveDate::parse_from_str(&start_date, "%Y-%m-%d")
                            {
                                if let Ok(parsed_time) =
                                    chrono::NaiveTime::parse_from_str(&start_time, "%H:%M")
                                {
                                    if let Some(parsed_datetime) = parsed_date
                                        .and_time(parsed_time)
                                        .and_local_timezone(Utc)
                                        .single()
                                    {
                                        event.start_time = parsed_datetime;
                                    }
                                }
                            }
                        });
                        ui.add_space(8.0);

                        // End Time
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("End Time:").color(theme.text));
                            let mut end_date = event.end_time.format("%Y-%m-%d").to_string();
                            let mut end_time = event.end_time.format("%H:%M").to_string();
                            ui.text_edit_singleline(&mut end_date);
                            ui.text_edit_singleline(&mut end_time);
                            if let Ok(parsed_date) =
                                chrono::NaiveDate::parse_from_str(&end_date, "%Y-%m-%d")
                            {
                                if let Ok(parsed_time) =
                                    chrono::NaiveTime::parse_from_str(&end_time, "%H:%M")
                                {
                                    if let Some(parsed_datetime) = parsed_date
                                        .and_time(parsed_time)
                                        .and_local_timezone(Utc)
                                        .single()
                                    {
                                        event.end_time = parsed_datetime;
                                    }
                                }
                            }
                        });
                        ui.add_space(8.0);

                        // Location
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Location:").color(theme.text));
                            let mut location = event.location.clone().unwrap_or_default();
                            ui.text_edit_singleline(&mut location);
                            event.location = if location.is_empty() {
                                None
                            } else {
                                Some(location)
                            };
                        });
                        ui.add_space(8.0);

                        // Attendees
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new("Attendees (comma-separated):").color(theme.text),
                            );
                            let attendees_str = event.attendees.join(", ");
                            let mut new_attendees = attendees_str.clone();
                            ui.text_edit_singleline(&mut new_attendees);
                            if new_attendees != attendees_str {
                                event.attendees = new_attendees
                                    .split(',')
                                    .map(|s| s.trim().to_string())
                                    .filter(|s| !s.is_empty())
                                    .collect();
                            }
                        });
                        ui.add_space(8.0);

                        // Color picker
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Color:").color(theme.text));
                            let mut color = event.color;
                            egui::color_picker::color_edit_button_srgba(
                                ui,
                                &mut color,
                                egui::color_picker::Alpha::Opaque,
                            );
                            event.color = color;
                        });
                        ui.add_space(8.0);

                        // Buttons
                        ui.horizontal(|ui| {
                            if ui
                                .button(RichText::new("Save").color(theme.accent))
                                .clicked()
                            {
                                // Add the event to our data structures
                                self.add_event(event.clone());
                                self.state.show_event_dialog = false;
                                self.state.new_event = None;
                            }
                            if ui
                                .button(RichText::new("Cancel").color(theme.text))
                                .clicked()
                            {
                                self.state.show_event_dialog = false;
                                self.state.new_event = None;
                            }
                        });
                    });
                    // Update the event in state
                    self.state.new_event = Some(event);
                }
            });
    }

    fn render_events_list(&self, ui: &mut Ui, theme: &Theme) {
        ui.vertical(|ui| {
            ui.strong(
                RichText::new("Upcoming Events")
                    .size(18.0)
                    .color(theme.header_text),
            );
            ui.separator();

            if self.state.all_events.is_empty() {
                ui.label(RichText::new("No upcoming events").color(theme.secondary_text));
            } else {
                let mut events: Vec<&Event> = self.state.all_events.iter().collect();
                events.sort_by(|a, b| a.start_time.cmp(&b.start_time));

                for event in events {
                    self.render_event(ui, event, theme);
                    ui.add_space(4.0);
                }
            }
        });
    }

    fn render_month_view(&mut self, ui: &mut Ui, theme: &Theme) {
        let selected_date = self.state.selected_date;
        let current_date = Local::now().date_naive();

        // Get the first day of the month
        let first_day = selected_date
            .with_day(1)
            .unwrap()
            .with_hour(0)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap();

        // Calculate offset to start from Monday
        let weekday_offset = first_day.weekday().num_days_from_monday() as i64;

        ui.vertical(|ui| {
            // Month navigation header
            self.render_navigation_header(ui, theme);

            // Day of week headers
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

                        // For month view, we want to use the same styling logic but with a different format
                        // Check if there are events on this day
                        let has_events = self.state.event_map.get(&current_date_naive).is_some();

                        // Style the day number
                        let mut text = RichText::new(format!("{}", day_num));

                        // Apply appropriate color based on conditions
                        if is_today {
                            text = text.color(theme.error);
                        } else if has_events && is_current_month {
                            text = text.color(theme.accent);
                        } else if is_current_month {
                            text = text.color(theme.text);
                        } else {
                            text = text.color(theme.secondary_text);
                        }

                        // Make events bold
                        if has_events && is_current_month {
                            text = text.strong();
                        }

                        let response = ui.add(egui::Button::new(text).frame(false));

                        // Highlight today with a circle
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
                            self.state.selected_date = current_day;
                            self.state.view_mode = CalendarViewMode::Day;
                        }

                        current_day = current_day + Duration::days(1);
                    }
                });
            }
        });
    }

    fn render_week_view(&mut self, ui: &mut Ui, theme: &Theme) {
        let selected_date = self.state.selected_date;
        let current_date = Local::now().date_naive();

        // Calculate the start of the week (Monday)
        let start_of_week =
            selected_date - Duration::days(selected_date.weekday().num_days_from_monday() as i64);

        ui.vertical(|ui| {
            // Week navigation header
            self.render_navigation_header(ui, theme);

            // Create a scrollable area to handle overflow
            egui::ScrollArea::horizontal().show(ui, |ui| {
                // Constrain the width to the available space
                let available_width = ui.available_width();
                // Calculate column widths - make time column narrower
                let time_col_width = 60.0; // Fixed width for time column
                let day_width = (available_width - time_col_width) / 7.0; // Divide remaining space for 7 days
                let hour_height = 60.0;

                // Create a grid for the week view
                egui::Grid::new("week_grid")
                    .spacing([1.0, 1.0])
                    .min_col_width(day_width)
                    .min_row_height(hour_height)
                    .show(ui, |ui| {
                        // Header row with day names
                        ui.label(""); // Empty cell for time column
                        for day_offset in 0..7 {
                            let current_day = start_of_week + Duration::days(day_offset);
                            let is_today = current_day.date_naive() == current_date;
                            let day_text =
                                RichText::new(format!("{}", current_day.format("%a %d")))
                                    .color(if is_today { theme.error } else { theme.text })
                                    .strong();
                            ui.label(day_text);
                        }
                        ui.end_row();

                        // Time slots (from 8:00 to 20:00)
                        for hour in 8..21 {
                            // Time label
                            ui.label(
                                RichText::new(format!("{:02}:00", hour))
                                    .color(theme.secondary_text),
                            );

                            // Day columns
                            for day_offset in 0..7 {
                                let current_day = start_of_week + Duration::days(day_offset);
                                let current_date_naive = current_day.date_naive();

                                // Create a frame for this time slot
                                let frame = egui::Frame::none()
                                    .stroke(Stroke::new(0.5, theme.border))
                                    .inner_margin(Margin::same(2))
                                    .fill(theme.panel);

                                frame.show(ui, |ui| {
                                    // Find events for this time slot
                                    if let Some(day_events) =
                                        self.state.event_map.get(&current_date_naive)
                                    {
                                        let hour_events: Vec<_> = day_events
                                            .iter()
                                            .filter(|e| {
                                                let event_hour = e.start_time.hour();
                                                event_hour == hour as u32
                                            })
                                            .collect();

                                        if !hour_events.is_empty() {
                                            for event in hour_events {
                                                let duration_mins = event.duration_minutes();
                                                let height =
                                                    (duration_mins as f32 / 60.0) * hour_height;

                                                // Event button
                                                let event_button = egui::Button::new(
                                                    RichText::new(&event.title)
                                                        .color(theme.panel)
                                                        .strong(),
                                                )
                                                .fill(event.color)
                                                .min_size(egui::Vec2::new(
                                                    day_width - 4.0,
                                                    height.min(hour_height - 4.0),
                                                ));

                                                if ui.add(event_button).clicked() {
                                                    // Select this event
                                                    self.state.selected_event = Some(event.id);
                                                }
                                            }
                                        } else {
                                            // Empty slot - clickable to create new event
                                            let response = ui.allocate_rect(
                                                ui.available_rect_before_wrap(),
                                                egui::Sense::click(),
                                            );

                                            if response.clicked() {
                                                // Create a new event at this time slot
                                                self.handle_time_slot_click(
                                                    current_date_naive,
                                                    hour as u32,
                                                );
                                            }
                                        }
                                    } else {
                                        // Empty slot - clickable to create new event
                                        let response = ui.allocate_rect(
                                            ui.available_rect_before_wrap(),
                                            egui::Sense::click(),
                                        );

                                        if response.clicked() {
                                            // Create a new event at this time slot
                                            self.handle_time_slot_click(
                                                current_date_naive,
                                                hour as u32,
                                            );
                                        }
                                    }
                                });
                            }
                            ui.end_row();
                        }
                    });
            });
        });
    }

    fn render_day_view(&mut self, ui: &mut Ui, theme: &Theme) {
        let selected_date = self.state.selected_date;
        let current_date = Local::now().date_naive();
        let selected_date_naive = selected_date.date_naive();
        let is_today = selected_date_naive == current_date;

        ui.vertical(|ui| {
            // Day navigation header
            self.render_navigation_header(ui, theme);

            // Create a scrollable area for the day view with constrained width
            let available_width = ui.available_width();
            let hour_height = 60.0;

            // Create a scrollable area that handles both horizontal and vertical overflow
            egui::ScrollArea::both().show(ui, |ui| {
                // Constrain the width to fit in the window
                ui.set_max_width(available_width);
                // Header for all-day events
                ui.horizontal(|ui| {
                    ui.label(RichText::new("All Day").color(theme.secondary_text));

                    // Check for all-day events
                    if let Some(day_events) = self.state.event_map.get(&selected_date_naive) {
                        let all_day_events: Vec<_> = day_events
                            .iter()
                            .filter(|e| {
                                let duration_hours = e.duration_minutes() / 60;
                                duration_hours >= 24
                            })
                            .collect();

                        if !all_day_events.is_empty() {
                            for event in all_day_events {
                                let event_button = egui::Button::new(
                                    RichText::new(&event.title).color(theme.panel).strong(),
                                )
                                .fill(event.color)
                                .min_size(egui::Vec2::new(available_width - 100.0, 30.0));

                                if ui.add(event_button).clicked() {
                                    self.state.selected_event = Some(event.id);
                                }
                            }
                        }
                    }
                });
                ui.separator();

                // Time slots (from 8:00 to 20:00)
                for hour in 8..21 {
                    ui.horizontal(|ui| {
                        // Time label
                        ui.label(
                            RichText::new(format!("{:02}:00", hour)).color(theme.secondary_text),
                        );

                        // Create a frame for this time slot
                        let frame = egui::Frame::none()
                            .stroke(Stroke::new(0.5, theme.border))
                            .inner_margin(Margin::same(4))
                            .fill(theme.panel);

                        frame.show(ui, |ui| {
                            // Find events for this time slot
                            if let Some(day_events) = self.state.event_map.get(&selected_date_naive)
                            {
                                let hour_events: Vec<_> = day_events
                                    .iter()
                                    .filter(|e| {
                                        let event_hour = e.start_time.hour();
                                        event_hour == hour as u32
                                    })
                                    .collect();

                                if !hour_events.is_empty() {
                                    for event in hour_events {
                                        let duration_mins = event.duration_minutes();
                                        let height = (duration_mins as f32 / 60.0) * hour_height;

                                        // Event button
                                        let event_button = egui::Button::new(
                                            RichText::new(&event.title).color(theme.panel).strong(),
                                        )
                                        .fill(event.color)
                                        .min_size(egui::Vec2::new(
                                            available_width - 100.0,
                                            height.min(hour_height - 8.0),
                                        ));

                                        if ui.add(event_button).clicked() {
                                            // Select this event
                                            self.state.selected_event = Some(event.id);
                                        }
                                    }
                                } else {
                                    // Empty slot - clickable to create new event
                                    let response = ui.allocate_rect(
                                        ui.available_rect_before_wrap(),
                                        egui::Sense::click(),
                                    );

                                    if response.clicked() {
                                        // Create a new event at this time slot
                                        self.handle_time_slot_click(
                                            selected_date_naive,
                                            hour as u32,
                                        );
                                    }
                                }
                            } else {
                                // Empty slot - clickable to create new event
                                let response = ui.allocate_rect(
                                    ui.available_rect_before_wrap(),
                                    egui::Sense::click(),
                                );

                                if response.clicked() {
                                    // Create a new event at this time slot
                                    self.handle_time_slot_click(selected_date_naive, hour as u32);
                                }
                            }
                        });
                    });
                }
            });
        });
    }

    fn render_event(&self, ui: &mut Ui, event: &Event, theme: &Theme) {
        egui::Frame::default()
            .fill(event.color.linear_multiply(0.7))
            .stroke(Stroke::new(1.0, event.color))
            .corner_radius(6.0)
            .inner_margin(Margin::same(10))
            .show(ui, |ui| {
                // Event title and date
                ui.horizontal(|ui| {
                    ui.strong(RichText::new(&event.title).size(16.0).color(theme.text));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Use chrono's formatting capabilities instead of manual matching
                        let date = event.start_time.format("%b %d").to_string();
                        ui.label(RichText::new(date).color(theme.accent));
                    });
                });

                ui.add_space(4.0);

                // Event time
                let start_time = format!(
                    "{:02}:{:02}",
                    event.start_time.hour(),
                    event.start_time.minute()
                );
                let end_time = format!(
                    "{:02}:{:02}",
                    event.end_time.hour(),
                    event.end_time.minute()
                );
                ui.label(
                    RichText::new(format!("⏱️ {} - {}", start_time, end_time)).color(theme.text),
                );

                ui.add_space(4.0);

                // Event description
                if !event.description.is_empty() {
                    ui.label(RichText::new(&event.description).weak().color(theme.text));
                    ui.add_space(2.0);
                }

                // Event location
                if let Some(location) = &event.location {
                    ui.label(
                        RichText::new(format!("📍 {}", location))
                            .weak()
                            .color(theme.text),
                    );
                }

                // Event attendees
                if !event.attendees.is_empty() {
                    let attendees = if event.attendees.len() <= 3 {
                        event.attendees.join(", ")
                    } else {
                        format!(
                            "{} and {} others",
                            event.attendees[0],
                            event.attendees.len() - 1
                        )
                    };
                    ui.label(
                        RichText::new(format!("👥 {}", attendees))
                            .weak()
                            .color(theme.text),
                    );
                }

                // Edit/Delete buttons
                ui.horizontal(|ui| {
                    if ui.button(RichText::new("Edit").color(theme.text)).clicked() {
                        // This would be implemented in a future update
                        println!("Edit button clicked for event: {}", event.id);
                    }

                    if ui
                        .button(RichText::new("Delete").color(theme.error))
                        .clicked()
                    {
                        // This would be implemented in a future update
                        println!("Delete button clicked for event: {}", event.id);
                    }
                });
            });
    }
}
