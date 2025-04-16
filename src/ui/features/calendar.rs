use chrono::{DateTime, Datelike, Duration, Local, Timelike, Utc};
use eframe::egui;
use egui::{Color32, RichText, Stroke, Ui};
// use icalendar::{Calendar, Component, Event as IcalEvent};
// use reqwest::Client;
// use std::str::FromStr;
use uuid::Uuid;

use crate::app::CircleApp;
use crate::models::dummy_data::Event;
use crate::ui::app_layout;

// Constants for styling
const PRIMARY_COLOR: Color32 = Color32::from_rgb(66, 133, 244);
const SECONDARY_COLOR: Color32 = Color32::from_rgb(232, 240, 254);
const TEXT_COLOR: Color32 = Color32::from_rgb(40, 50, 60);
const MUTED_COLOR: Color32 = Color32::from_rgb(150, 150, 150);
const BORDER_COLOR: Color32 = Color32::from_rgb(218, 220, 224);

/// Calendar state containing events and view settings
#[derive(Clone)]
struct CalendarState {
    events: Vec<Event>,
    selected_date: DateTime<Local>,
    view_mode: CalendarViewMode,
    // caldav_config: Option<CalDavConfig>,
    // last_sync: Option<DateTime<Utc>>,
}

/// Available calendar view modes
#[derive(Clone, PartialEq)]
enum CalendarViewMode {
    Month,
    Week,
    Day,
}

/// Creates a default calendar state with current date and month view
fn get_default_calendar_state() -> CalendarState {
    CalendarState {
        events: Vec::new(),
        selected_date: Local::now(),
        view_mode: CalendarViewMode::Month,
        // caldav_config: None,
        // last_sync: None,
    }
}

// /// Client for interacting with CalDAV servers
// struct CalDavClient {
//     client: Client,
//     config: CalDavConfig,
// }

// impl CalDavClient {
//     async fn new(config: CalDavConfig) -> Self {
//         CalDavClient {
//             client: Client::new(),
//             config,
//         }
//     }

//     async fn fetch_events(&self, calendar_url: &str) -> Result<Vec<Event>, String> {
//         let response = self
//             .client
//             .get(calendar_url)
//             .basic_auth(&self.config.username, Some(&self.config.password))
//             .send()
//             .await
//             .map_err(|e| e.to_string())?;

//         let ical_text = response.text().await.map_err(|e| e.to_string())?;
//         let calendar = Calendar::from_str(&ical_text).map_err(|e| e.to_string())?;

//         let mut events = Vec::new();
//         for component in calendar.components {
//             if let Some(ical_event) = component.as_event() {
//                 let event = Event {
//                     id: uuid::Uuid::new_v4(),
//                     title: ical_event.get_summary().unwrap_or_default().to_string(),
//                     start_time: chrono::Utc::now(), // Default to current time
//                     end_time: chrono::Utc::now() + chrono::Duration::hours(1), // Default to 1 hour later
//                     description: ical_event.get_description().unwrap_or_default().to_string(),
//                     location: None, // Simplified
//                     attendees: Vec::new(),
//                 };
//                 events.push(event);
//             }
//         }
//         Ok(events)
//     }

//     async fn push_event(&self, calendar_url: &str, event: &Event) -> Result<(), String> {
//         let mut ical_event = IcalEvent::new();
//         ical_event.summary(&event.title);
//         // Use the EventLike trait methods properly
//         ical_event.add_property("DTSTART", event.start_time.to_rfc3339());
//         ical_event.add_property("DTEND", event.end_time.to_rfc3339());
//         ical_event.description(&event.description);
//         if let Some(location) = &event.location {
//             ical_event.add_property("LOCATION", location);
//         }
//         for attendee in &event.attendees {
//             ical_event.add_property("ATTENDEE", attendee);
//         }
//         ical_event.uid(&event.id.to_string());

//         let calendar = Calendar::new().push(ical_event).done();
//         let ical_text = calendar.to_string();

//         self.client
//             .put(format!("{}/{}", calendar_url, event.id))
//             .basic_auth(&self.config.username, Some(&self.config.password))
//             .body(ical_text)
//             .send()
//             .await
//             .map_err(|e| e.to_string())?;

//         Ok(())
//     }
// }

/// Main calendar rendering function
///
/// Renders the calendar interface with toolbar, view selector, and content area
pub fn render_calendar(app: &mut CircleApp, ui: &mut egui::Ui) {
    // Create a local calendar state for this render
    let mut calendar_state = get_default_calendar_state();

    // Get events from the active feature data if available
    if let Some(feature_data) = &app.active_feature_data {
        calendar_state.events = feature_data.calendar_data.events.clone();
    }
    // Get the active circle name (for potential future use)
    let _circle_name = match app.active_circle() {
        Some(circle) => circle.name.clone(),
        None => "No Circle".to_string(),
    };

    // Render toolbar with actions
    render_toolbar(ui, &mut calendar_state);

    ui.add_space(16.0);

    // Main content area with calendar view and events list
    app_layout::create_content_frame().show(ui, |ui| {
        ui.horizontal(|ui| {
            // Calendar view in left panel
            render_calendar_view(ui, &mut calendar_state);

            ui.separator();

            // Events list in right panel
            render_events_list(ui, &calendar_state);
        });
    });
}

/// Renders the toolbar with action buttons and view selector
fn render_toolbar(ui: &mut Ui, calendar_state: &mut CalendarState) {
    ui.horizontal(|ui| {
        ui.add_space(8.0);

        // New Event button
        if ui
            .add(app_layout::create_action_button("New Event", "➕"))
            .clicked()
        {
            calendar_state.events.push(Event {
                id: Uuid::new_v4(),
                title: String::new(),
                start_time: Utc::now(),
                end_time: Utc::now(),
                description: String::new(),
                location: None,
                attendees: Vec::new(),
            });
        }

        // Today button
        if ui
            .add(app_layout::create_action_button("Today", "📌"))
            .clicked()
        {
            calendar_state.selected_date = Local::now();
        }

        // Sync button
        if ui
            .add(app_layout::create_action_button("Sync", "🔄"))
            .clicked()
        {
            // if let Some(_) = &calendar_state.caldav_config {
            //     println!("CalDAV sync would happen here");
            //     calendar_state.last_sync = Some(Utc::now());
            // }
        }

        ui.add_space(8.0);

        // View selector using the same style as other buttons
        if ui
            .add(app_layout::create_action_button("View", "👁️"))
            .clicked()
        {
            // Toggle between view modes when clicked
            calendar_state.view_mode = match calendar_state.view_mode {
                CalendarViewMode::Month => CalendarViewMode::Week,
                CalendarViewMode::Week => CalendarViewMode::Day,
                CalendarViewMode::Day => CalendarViewMode::Month,
            };
        }

        // Show current view mode
        ui.label(
            RichText::new(format!(
                "Current: {}",
                match calendar_state.view_mode {
                    CalendarViewMode::Month => "Month",
                    CalendarViewMode::Week => "Week",
                    CalendarViewMode::Day => "Day",
                }
            ))
            .size(14.0)
            .color(PRIMARY_COLOR),
        );
    });
}

/// Renders the main calendar view based on the selected view mode
fn render_calendar_view(ui: &mut Ui, calendar_state: &mut CalendarState) {
    egui::Frame::new()
        .fill(Color32::WHITE)
        .stroke(Stroke::new(1.0, BORDER_COLOR))
        .corner_radius(4)
        .show(ui, |ui| match calendar_state.view_mode {
            CalendarViewMode::Month => render_month_view(ui, calendar_state),
            CalendarViewMode::Week => render_week_view(ui, calendar_state),
            CalendarViewMode::Day => render_day_view(ui, calendar_state),
        });
}

/// Renders the events list panel
fn render_events_list(ui: &mut Ui, calendar_state: &CalendarState) {
    ui.vertical(|ui| {
        ui.strong(RichText::new("Upcoming Events").size(18.0));
        ui.separator();

        if calendar_state.events.is_empty() {
            ui.label("No upcoming events");
        } else {
            let mut events = calendar_state.events.clone();
            events.sort_by(|a, b| a.start_time.cmp(&b.start_time));

            for event in events {
                render_event(ui, &event);
                ui.add_space(4.0);
            }
        }
    });
}

/// Renders the month view of the calendar
fn render_month_view(ui: &mut Ui, calendar_state: &mut CalendarState) {
    let selected_date = calendar_state.selected_date;

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
                calendar_state.selected_date = selected_date - Duration::days(30);
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
                calendar_state.selected_date = selected_date + Duration::days(30);
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

                    let has_events = calendar_state
                        .events
                        .iter()
                        .any(|event| event.start_time.date_naive() == current_day.date_naive());

                    // Style the day number based on whether it's in the current month and has events
                    let text = RichText::new(format!("{}", day_num)).color(if is_current_month {
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
                        calendar_state.selected_date = current_day;
                        calendar_state.view_mode = CalendarViewMode::Day;
                    }

                    current_day = current_day + Duration::days(1);
                }
            });
        }
    });
}

/// Renders the week view of the calendar
fn render_week_view(ui: &mut Ui, calendar_state: &mut CalendarState) {
    let selected_date = calendar_state.selected_date;

    // Calculate the start of the week (Monday)
    let start_of_week =
        selected_date - Duration::days(selected_date.weekday().num_days_from_monday() as i64);

    ui.vertical(|ui| {
        // Week navigation header
        ui.horizontal(|ui| {
            if ui.button(RichText::new("◀").color(PRIMARY_COLOR)).clicked() {
                calendar_state.selected_date = selected_date - Duration::days(7);
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
                calendar_state.selected_date = selected_date + Duration::days(7);
            }
        });
        ui.separator();

        for day_offset in 0..7 {
            let current_day = start_of_week + Duration::days(day_offset);
            let has_events = calendar_state
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

/// Renders the day view of the calendar
fn render_day_view(ui: &mut Ui, calendar_state: &mut CalendarState) {
    let selected_date = calendar_state.selected_date;

    ui.vertical(|ui| {
        // Day navigation header
        ui.horizontal(|ui| {
            if ui.button(RichText::new("◀").color(PRIMARY_COLOR)).clicked() {
                calendar_state.selected_date = selected_date - Duration::days(1);
            }
            ui.strong(RichText::new(format!("{}", selected_date.format("%B %d, %Y"))).size(16.0));
            if ui.button(RichText::new("▶").color(PRIMARY_COLOR)).clicked() {
                calendar_state.selected_date = selected_date + Duration::days(1);
            }
        });
        ui.separator();

        let day_events: Vec<_> = calendar_state
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
                render_event(ui, event);
                ui.add_space(8.0);
            }
        }
    });
}

/// Renders a single event card
fn render_event(ui: &mut Ui, event: &Event) {
    egui::Frame::new()
        .fill(SECONDARY_COLOR)
        .stroke(Stroke::new(1.0, PRIMARY_COLOR.linear_multiply(0.5)))
        .corner_radius(6)
        .inner_margin(egui::Margin::same(10))
        .shadow(egui::epaint::Shadow {
            color: Color32::from_black_alpha(25),
            offset: [0, 4],
            blur: 8,
            spread: 0,
        })
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
            ui.label(RichText::new(format!("⏱️ {} - {}", start_time, end_time)).color(TEXT_COLOR));

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
