use chrono::{Local, NaiveDate, NaiveTime, TimeZone, Utc};
use egui::{Button, RichText, Ui, Window, color_picker};

use super::super::state::CalendarState;
use crate::utils::config::Theme;

pub fn render_event_dialog(ui: &mut Ui, state: &mut CalendarState, theme: &Theme) {
    // Early return if no event or dialog not shown
    if !state.show_event_dialog || state.new_event.is_none() {
        return;
    }

    // Track dialog actions
    let mut save_event = false;
    let mut cancel_dialog = false;

    // Show the dialog window
    Window::new("Event Details")
        .collapsible(false)
        .resizable(false)
        .show(ui.ctx(), |ui| {
            // Get a mutable reference to the event
            if let Some(event) = &mut state.new_event {
                // Handle time slot if present
                if let Some((date, time)) = state.selected_time_slot.take() {
                    if event.start_time.date_naive() != date || event.start_time.time() != time {
                        if let Some(local_dt) =
                            Local.from_local_datetime(&date.and_time(time)).single()
                        {
                            event.start_time = local_dt.with_timezone(&Utc);
                            event.end_time = event.start_time + chrono::Duration::hours(1);
                        }
                    }
                }

                // Render form fields
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
                        let mut date = event.start_time.format("%Y-%m-%d").to_string();
                        let mut time = event.start_time.format("%H:%M").to_string();
                        ui.text_edit_singleline(&mut date);
                        ui.text_edit_singleline(&mut time);
                        if let Ok(parsed_date) = NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
                            if let Ok(parsed_time) = NaiveTime::parse_from_str(&time, "%H:%M") {
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
                        let mut date = event.end_time.format("%Y-%m-%d").to_string();
                        let mut time = event.end_time.format("%H:%M").to_string();
                        ui.text_edit_singleline(&mut date);
                        ui.text_edit_singleline(&mut time);
                        if let Ok(parsed_date) = NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
                            if let Ok(parsed_time) = NaiveTime::parse_from_str(&time, "%H:%M") {
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
                        let mut location_text = event.location.clone().unwrap_or_default();
                        ui.text_edit_singleline(&mut location_text);
                        event.location = if location_text.is_empty() {
                            None
                        } else {
                            Some(location_text)
                        };
                    });
                    ui.add_space(8.0);

                    // Attendees
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Attendees (comma-separated):").color(theme.text));
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

                    // Color
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Color:").color(theme.text));
                        color_picker::color_edit_button_srgba(
                            ui,
                            &mut event.color,
                            color_picker::Alpha::Opaque,
                        );
                    });
                    ui.add_space(8.0);

                    // Action buttons
                    ui.horizontal(|ui| {
                        if ui
                            .add(Button::new(RichText::new("Save").color(theme.accent)))
                            .clicked()
                        {
                            save_event = true;
                            cancel_dialog = true;
                        }
                        if ui
                            .add(Button::new(RichText::new("Cancel").color(theme.text)))
                            .clicked()
                        {
                            cancel_dialog = true;
                        }
                    });
                });
            }
        });

    // After the dialog is closed, update the state
    if save_event && state.new_event.is_some() {
        // Clone the event before taking it from state
        let event_to_add = state.new_event.clone().unwrap();
        // Add the event to the calendar
        crate::ui::features::calendar::Calendar::add_event_to_state(state, event_to_add);
    }

    if cancel_dialog {
        state.show_event_dialog = false;
        if !save_event {
            state.new_event = None;
        }
    }
}
