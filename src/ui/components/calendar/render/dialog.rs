use chrono::{Local, NaiveDate, NaiveTime, TimeZone, Utc};
use egui::{RichText, Ui, Window, color_picker};

use super::super::state::CalendarState;
use crate::ui::components::button::render_button;
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
    // Create a standardized window with consistent styling to match circle dialog
    Window::new("Event Details")
        .collapsible(false)
        .resizable(false)
        .fixed_size([600.0, 620.0]) // Fixed size to match circle dialog
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0]) // Centered horizontally
        .frame(
            egui::Frame::window(&ui.ctx().style())
                .fill(theme.background)
                .corner_radius(16)
                .shadow(egui::epaint::Shadow {
                    color: egui::Color32::from_black_alpha(25),
                    offset: [0, 4],
                    blur: 8,
                    spread: 0,
                })
                .inner_margin(egui::Margin::same(24)),
        )
        .show(ui.ctx(), |ui| {
            // Main layout
            ui.vertical(|ui| {
                // Heading
                ui.vertical_centered(|ui| {
                    ui.heading(
                        RichText::new("Event Details")
                            .size(24.0)
                            .strong()
                            .color(theme.text),
                    );
                    ui.add_space(20.0);
                });

                // Get a mutable reference to the event
                if let Some(event) = &mut state.new_event {
                    // Handle time slot if present
                    if let Some((date, time)) = state.selected_time_slot.take() {
                        if event.start_time.date_naive() != date || event.start_time.time() != time
                        {
                            if let Some(local_dt) =
                                Local.from_local_datetime(&date.and_time(time)).single()
                            {
                                event.start_time = local_dt.with_timezone(&Utc);
                                event.end_time = event.start_time + chrono::Duration::hours(1);
                            }
                        }
                    }

                    // Set theme-appropriate background for all widgets
                    ui.style_mut().visuals.extreme_bg_color = theme.secondary_background;
                    ui.style_mut().visuals.widgets.inactive.bg_fill = theme.secondary_background;
                    ui.style_mut().visuals.widgets.active.bg_fill = theme.hover;
                    ui.style_mut().visuals.widgets.hovered.bg_fill = theme.hover;

                    // Title
                    ui.label(
                        RichText::new("Title *")
                            .strong()
                            .size(16.0)
                            .color(theme.text),
                    );
                    ui.add(
                        egui::TextEdit::singleline(&mut event.title)
                            .margin(egui::Vec2::new(10.0, 8.0))
                            .desired_width(f32::INFINITY)
                            .font(egui::FontId::proportional(16.0)),
                    );
                    ui.add_space(16.0);

                    // Description
                    ui.label(
                        RichText::new("Description")
                            .strong()
                            .size(16.0)
                            .color(theme.text),
                    );
                    ui.add(
                        egui::TextEdit::multiline(&mut event.description)
                            .margin(egui::Vec2::new(10.0, 8.0))
                            .desired_width(f32::INFINITY)
                            .desired_rows(4)
                            .font(egui::FontId::proportional(16.0)),
                    );
                    ui.add_space(16.0);

                    // Start Time
                    ui.label(
                        RichText::new("Start Time *")
                            .strong()
                            .size(16.0)
                            .color(theme.text),
                    );
                    ui.horizontal(|ui| {
                        let mut date = event.start_time.format("%Y-%m-%d").to_string();
                        let mut time = event.start_time.format("%H:%M").to_string();

                        ui.add(
                            egui::TextEdit::singleline(&mut date)
                                .margin(egui::Vec2::new(10.0, 8.0))
                                .desired_width(200.0)
                                .hint_text("YYYY-MM-DD")
                                .font(egui::FontId::proportional(16.0)),
                        );

                        ui.add_space(8.0);

                        ui.add(
                            egui::TextEdit::singleline(&mut time)
                                .margin(egui::Vec2::new(10.0, 8.0))
                                .desired_width(100.0)
                                .hint_text("HH:MM")
                                .font(egui::FontId::proportional(16.0)),
                        );

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
                    ui.add_space(16.0);

                    // End Time
                    ui.label(
                        RichText::new("End Time *")
                            .strong()
                            .size(16.0)
                            .color(theme.text),
                    );
                    ui.horizontal(|ui| {
                        let mut date = event.end_time.format("%Y-%m-%d").to_string();
                        let mut time = event.end_time.format("%H:%M").to_string();

                        ui.add(
                            egui::TextEdit::singleline(&mut date)
                                .margin(egui::Vec2::new(10.0, 8.0))
                                .desired_width(200.0)
                                .hint_text("YYYY-MM-DD")
                                .font(egui::FontId::proportional(16.0)),
                        );

                        ui.add_space(8.0);

                        ui.add(
                            egui::TextEdit::singleline(&mut time)
                                .margin(egui::Vec2::new(10.0, 8.0))
                                .desired_width(100.0)
                                .hint_text("HH:MM")
                                .font(egui::FontId::proportional(16.0)),
                        );

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
                    ui.add_space(16.0);

                    // Location
                    ui.label(
                        RichText::new("Location")
                            .strong()
                            .size(16.0)
                            .color(theme.text),
                    );
                    let mut location_text = event.location.clone().unwrap_or_default();
                    ui.add(
                        egui::TextEdit::singleline(&mut location_text)
                            .margin(egui::Vec2::new(10.0, 8.0))
                            .desired_width(f32::INFINITY)
                            .font(egui::FontId::proportional(16.0)),
                    );
                    event.location = if location_text.is_empty() {
                        None
                    } else {
                        Some(location_text)
                    };
                    ui.add_space(16.0);

                    // Attendees
                    ui.label(
                        RichText::new("Attendees")
                            .strong()
                            .size(16.0)
                            .color(theme.text),
                    );
                    let attendees_str = event.attendees.join(", ");
                    let mut new_attendees = attendees_str.clone();
                    ui.add(
                        egui::TextEdit::singleline(&mut new_attendees)
                            .margin(egui::Vec2::new(10.0, 8.0))
                            .desired_width(f32::INFINITY)
                            .hint_text("Enter attendees, comma separated")
                            .font(egui::FontId::proportional(16.0)),
                    );
                    ui.label(
                        RichText::new("(comma-separated)")
                            .color(theme.secondary_text)
                            .small(),
                    );
                    if new_attendees != attendees_str {
                        event.attendees = new_attendees
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                    }
                    ui.add_space(16.0);

                    // Color
                    ui.label(RichText::new("Color").strong().size(16.0).color(theme.text));
                    color_picker::color_edit_button_srgba(
                        ui,
                        &mut event.color,
                        color_picker::Alpha::Opaque,
                    );
                    ui.add_space(28.0);

                    // Separator before buttons
                    ui.separator();
                    ui.add_space(12.0);

                    // Action buttons
                    ui.horizontal(|ui| {
                        // Cancel button
                        if render_button(ui, "Cancel", false, theme, None)
                            .on_hover_cursor(egui::CursorIcon::PointingHand)
                            .clicked()
                        {
                            cancel_dialog = true;
                        }

                        ui.allocate_space(ui.available_size_before_wrap());

                        // Save button
                        if render_button(ui, "Save Event", true, theme, None)
                            .on_hover_cursor(egui::CursorIcon::PointingHand)
                            .clicked()
                        {
                            save_event = true;
                            cancel_dialog = true;
                        }
                    });
                    ui.add_space(16.0);
                }
            });
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
