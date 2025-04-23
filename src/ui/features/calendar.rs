use chrono::{DateTime, Datelike, Duration, Local, TimeZone, Timelike, Utc};
use eframe::egui::{self, Color32, RichText, Stroke, Ui};
use uuid::Uuid;

// Constants for styling
const PRIMARY_COLOR: Color32 = Color32::from_rgb(66, 133, 244);
const SECONDARY_COLOR: Color32 = Color32::from_rgb(232, 240, 254);
const TEXT_COLOR: Color32 = Color32::from_rgb(40, 50, 60);
const MUTED_COLOR: Color32 = Color32::from_rgb(150, 150, 150);
const BORDER_COLOR: Color32 = Color32::from_rgb(218, 220, 224);
const TODAY_COLOR: Color32 = Color32::from_rgb(234, 67, 53);

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

impl Default for Event {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            title: String::new(),
            description: String::new(),
            start_time: Utc::now(),
            end_time: Utc::now() + Duration::hours(1),
            location: None,
            attendees: Vec::new(),
            color: PRIMARY_COLOR,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CalendarState {
    events: Vec<Event>,
    selected_date: DateTime<Local>,
    view_mode: CalendarViewMode,
    selected_event: Option<Uuid>,
    show_event_dialog: bool,
    new_event: Option<Event>,
}

impl Default for CalendarState {
    fn default() -> Self {
        Self {
            events: Vec::new(),
            selected_date: Local::now(),
            view_mode: CalendarViewMode::Month,
            selected_event: None,
            show_event_dialog: false,
            new_event: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum CalendarViewMode {
    Year,
    Month,
    Week,
    Day,
}

pub struct Calendar {
    state: CalendarState,
}

impl Calendar {
    pub fn new() -> Self {
        Self {
            state: CalendarState::default(),
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui) {
        self.render_toolbar(ui);
        ui.add_space(16.0);

        egui::Frame::new()
            .fill(ui.style().visuals.window_fill())
            .show(ui, |ui| match self.state.view_mode {
                CalendarViewMode::Year => self.render_year_view(ui),
                CalendarViewMode::Month => self.render_month_view(ui),
                CalendarViewMode::Week => self.render_week_view(ui),
                CalendarViewMode::Day => self.render_day_view(ui),
            });

        if self.state.show_event_dialog {
            self.render_event_dialog(ui);
        }
    }

    fn render_toolbar(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if ui.button("New Event").clicked() {
                self.state.new_event = Some(Event::default());
                self.state.show_event_dialog = true;
            }

            if ui.button("Today").clicked() {
                self.state.selected_date = Local::now();
            }

            if ui.button("◀").clicked() {
                self.navigate(false);
            }

            ui.label(self.get_header_text());

            if ui.button("▶").clicked() {
                self.navigate(true);
            }

            egui::ComboBox::from_label("View")
                .selected_text(format!("{:?}", self.state.view_mode))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.state.view_mode, CalendarViewMode::Year, "Year");
                    ui.selectable_value(
                        &mut self.state.view_mode,
                        CalendarViewMode::Month,
                        "Month",
                    );
                    ui.selectable_value(&mut self.state.view_mode, CalendarViewMode::Week, "Week");
                    ui.selectable_value(&mut self.state.view_mode, CalendarViewMode::Day, "Day");
                });
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

    fn navigate(&mut self, forward: bool) {
        let duration = match self.state.view_mode {
            CalendarViewMode::Year => Duration::days(365),
            CalendarViewMode::Month => {
                let days_in_month = self.days_in_month(
                    self.state.selected_date.year(),
                    self.state.selected_date.month(),
                );
                Duration::days(days_in_month as i64)
            }
            CalendarViewMode::Week => Duration::days(7),
            CalendarViewMode::Day => Duration::days(1),
        };

        self.state.selected_date = if forward {
            self.state.selected_date + duration
        } else {
            self.state.selected_date - duration
        };
    }

    fn days_in_month(&self, year: i32, month: u32) -> u32 {
        let next_month = if month == 12 {
            Local.with_ymd_and_hms(year + 1, 1, 1, 0, 0, 0)
        } else {
            Local.with_ymd_and_hms(year, month + 1, 1, 0, 0, 0)
        };

        let current_month = Local.with_ymd_and_hms(year, month, 1, 0, 0, 0).unwrap();
        (next_month.unwrap() - current_month).num_days() as u32
    }

    fn render_year_view(&mut self, ui: &mut Ui) {
        let year = self.state.selected_date.year();

        egui::Grid::new("year_grid")
            .spacing([20.0, 20.0])
            .show(ui, |ui| {
                for month_chunk in (1..=12).collect::<Vec<u32>>().chunks(3) {
                    for &month in month_chunk {
                        let month_date = Local.with_ymd_and_hms(year, month, 1, 0, 0, 0).unwrap();
                        self.render_mini_month(ui, month_date);
                    }
                    ui.end_row();
                }
            });
    }

    fn render_mini_month(&mut self, ui: &mut Ui, date: DateTime<Local>) {
        let month_name = date.format("%B").to_string();

        egui::Frame::new()
            .stroke(Stroke::new(1.0, BORDER_COLOR))
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.heading(month_name);

                    egui::Grid::new(format!("month_{}", date.month()))
                        .spacing([4.0, 4.0])
                        .show(ui, |ui| {
                            // Weekday headers
                            for day in ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"] {
                                ui.label(RichText::new(day).small());
                            }
                            ui.end_row();

                            let first_day = date.with_day(1).unwrap();
                            let weekday_offset = first_day.weekday().num_days_from_monday() as i64;
                            let mut current_day = first_day - Duration::days(weekday_offset);

                            for _ in 0..6 {
                                for _ in 0..7 {
                                    let day_num = current_day.day();
                                    let is_current_month = current_day.month() == date.month();
                                    let is_today =
                                        current_day.date_naive() == Local::now().date_naive();

                                    let text = RichText::new(format!("{:2}", day_num))
                                        .color(if is_today {
                                            TODAY_COLOR
                                        } else if is_current_month {
                                            TEXT_COLOR
                                        } else {
                                            MUTED_COLOR
                                        })
                                        .small();

                                    if ui.add(egui::Button::new(text).frame(false)).clicked() {
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

    fn render_event_dialog(&mut self, ui: &mut Ui) {
        egui::Window::new("Event Details")
            .collapsible(false)
            .resizable(false)
            .show(ui.ctx(), |ui| {
                if let Some(event) = &self.state.new_event {
                    let mut event = event.clone(); // Clone the event
                    ui.vertical(|ui| {
                        // Title
                        ui.horizontal(|ui| {
                            ui.label("Title:");
                            ui.text_edit_singleline(&mut event.title);
                        });
                        ui.add_space(8.0);

                        // Description
                        ui.horizontal(|ui| {
                            ui.label("Description:");
                            ui.text_edit_multiline(&mut event.description);
                        });
                        ui.add_space(8.0);

                        // Start Time
                        ui.horizontal(|ui| {
                            ui.label("Start Time:");
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
                            ui.label("End Time:");
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
                            ui.label("Location:");
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
                            ui.label("Attendees (comma-separated):");
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

                        // Buttons
                        ui.horizontal(|ui| {
                            if ui.button("Save").clicked() {
                                self.state.events.push(event.clone());
                                self.state.show_event_dialog = false;
                                self.state.new_event = None;
                            }
                            if ui.button("Cancel").clicked() {
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

    fn render_events_list(&self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.strong(RichText::new("Upcoming Events").size(18.0));
            ui.separator();

            if self.state.events.is_empty() {
                ui.label("No upcoming events");
            } else {
                let mut events: Vec<&Event> = self.state.events.iter().collect();
                events.sort_by(|a, b| a.start_time.cmp(&b.start_time));

                for event in events {
                    self.render_event(ui, event);
                    ui.add_space(4.0);
                }
            }
        });
    }

    fn render_month_view(&mut self, ui: &mut Ui) {
        let selected_date = self.state.selected_date;

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
            ui.horizontal(|ui| {
                if ui.button(RichText::new("◀").color(PRIMARY_COLOR)).clicked() {
                    self.navigate(false);
                }
                ui.strong(
                    RichText::new(format!(
                        "{} {}",
                        selected_date.format("%B"),
                        selected_date.year()
                    ))
                    .size(16.0),
                );
                if ui.button(RichText::new("▶").color(PRIMARY_COLOR)).clicked() {
                    self.navigate(true);
                }
            });
            ui.separator();

            // Day of week headers
            ui.horizontal(|ui| {
                for day in ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"] {
                    ui.label(RichText::new(day).strong().color(TEXT_COLOR));
                }
            });

            let mut current_day = first_day - Duration::days(weekday_offset);
            for _week in 0..6 {
                ui.horizontal(|ui| {
                    for _day in 0..7 {
                        let day_num = current_day.day();
                        let is_current_month = current_day.month() == selected_date.month();

                        let has_events =
                            self.state.events.iter().any(|event| {
                                event.start_time.date_naive() == current_day.date_naive()
                            });

                        // Style the day number
                        let text =
                            RichText::new(format!("{}", day_num)).color(if is_current_month {
                                if has_events {
                                    PRIMARY_COLOR
                                } else {
                                    TEXT_COLOR
                                }
                            } else {
                                MUTED_COLOR
                            });

                        let text = if has_events && is_current_month {
                            text.strong()
                        } else {
                            text
                        };

                        let button = egui::Button::new(text).frame(false);

                        if ui.add(button).clicked() {
                            self.state.selected_date = current_day;
                            self.state.view_mode = CalendarViewMode::Day;
                        }

                        current_day = current_day + Duration::days(1);
                    }
                });
            }
        });
    }

    fn render_week_view(&mut self, ui: &mut Ui) {
        let selected_date = self.state.selected_date;

        // Calculate the start of the week (Monday)
        let start_of_week =
            selected_date - Duration::days(selected_date.weekday().num_days_from_monday() as i64);

        ui.vertical(|ui| {
            // Week navigation header
            ui.horizontal(|ui| {
                if ui.button(RichText::new("◀").color(PRIMARY_COLOR)).clicked() {
                    self.navigate(false);
                }
                ui.strong(
                    RichText::new(format!(
                        "Week of {} {}",
                        selected_date.format("%B"),
                        selected_date.day()
                    ))
                    .size(16.0),
                );
                if ui.button(RichText::new("▶").color(PRIMARY_COLOR)).clicked() {
                    self.navigate(true);
                }
            });
            ui.separator();

            for day_offset in 0..7 {
                let current_day = start_of_week + Duration::days(day_offset);
                let has_events = self
                    .state
                    .events
                    .iter()
                    .any(|event| event.start_time.date_naive() == current_day.date_naive());

                ui.horizontal(|ui| {
                    // Show day with indicator for events
                    ui.label(
                        RichText::new(format!("{}", current_day.format("%a %d"))).color(TEXT_COLOR),
                    );
                    if has_events {
                        ui.label(RichText::new("•").size(16.0).color(PRIMARY_COLOR));
                    }
                });
            }
        });
    }

    fn render_day_view(&mut self, ui: &mut Ui) {
        let selected_date = self.state.selected_date;

        ui.vertical(|ui| {
            // Day navigation header
            ui.horizontal(|ui| {
                if ui.button(RichText::new("◀").color(PRIMARY_COLOR)).clicked() {
                    self.navigate(false);
                }
                ui.strong(
                    RichText::new(format!("{}", selected_date.format("%B %d, %Y"))).size(16.0),
                );
                if ui.button(RichText::new("▶").color(PRIMARY_COLOR)).clicked() {
                    self.navigate(true);
                }
            });
            ui.separator();

            let day_events: Vec<_> = self
                .state
                .events
                .iter()
                .filter(|e| e.start_time.date_naive() == selected_date.date_naive())
                .collect();

            // Show events for the selected day
            if day_events.is_empty() {
                ui.add_space(10.0);
                ui.label(RichText::new("No events today").color(MUTED_COLOR));
            } else {
                ui.add_space(10.0);
                for event in day_events {
                    self.render_event(ui, event);
                    ui.add_space(8.0);
                }
            }
        });
    }

    fn render_event(&self, ui: &mut Ui, event: &Event) {
        egui::Frame::default()
            .fill(SECONDARY_COLOR)
            .stroke(Stroke::new(1.0, PRIMARY_COLOR.linear_multiply(0.5)))
            .corner_radius(6)
            .inner_margin(egui::Margin::same(10))
            .show(ui, |ui| {
                // Event title and date
                ui.horizontal(|ui| {
                    ui.strong(RichText::new(&event.title).size(16.0).color(TEXT_COLOR));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let month_name = match event.start_time.month() {
                            1 => "Jan",
                            2 => "Feb",
                            3 => "Mar",
                            4 => "Apr",
                            5 => "May",
                            6 => "Jun",
                            7 => "Jul",
                            8 => "Aug",
                            9 => "Sep",
                            10 => "Oct",
                            11 => "Nov",
                            12 => "Dec",
                            _ => "???",
                        };

                        let date = format!("{} {}", month_name, event.start_time.day());
                        ui.label(RichText::new(date).color(PRIMARY_COLOR));
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
                    RichText::new(format!("⏱️ {} - {}", start_time, end_time)).color(TEXT_COLOR),
                );

                ui.add_space(4.0);

                // Event description
                if !event.description.is_empty() {
                    ui.label(RichText::new(&event.description).weak().color(TEXT_COLOR));
                    ui.add_space(2.0);
                }

                // Event location
                if let Some(location) = &event.location {
                    ui.label(
                        RichText::new(format!("📍 {}", location))
                            .weak()
                            .color(TEXT_COLOR),
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
                            .color(TEXT_COLOR),
                    );
                }
            });
    }
}
