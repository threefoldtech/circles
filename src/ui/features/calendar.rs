use chrono::{Datelike, Timelike};
use eframe::egui;
use egui::RichText;

use crate::app::CircleApp;
use crate::models::dummy_data::Event;
use crate::ui::app_layout;

pub fn render_calendar(app: &CircleApp, ui: &mut egui::Ui) {
    app_layout::render_header(ui, "📅", "Calendar");

    // Get the active circle name
    let circle_name = app
        .active_circle()
        .map_or("No Circle".to_string(), |c| c.name.clone());

    ui.horizontal(|ui| {
        ui.add_space(8.0);
        if ui
            .add(app_layout::create_action_button("New Event", "➕"))
            .clicked()
        {
            // TODO: Implement new event
        }
        ui.add_space(8.0);
        if ui
            .add(app_layout::create_action_button("Today", "📌"))
            .clicked()
        {
            // TODO: Implement today
        }
        ui.add_space(8.0);

        // Show the active circle name
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                RichText::new(format!("Circle: {}", circle_name))
                    .size(14.0)
                    .strong(),
            );
        });
    });

    ui.add_space(16.0);

    // Check if we have active feature data
    if let Some(feature_data) = &app.active_feature_data {
        app_layout::create_content_frame().show(ui, |ui| {
            ui.horizontal(|ui| {
                // Calendar view
                egui::Frame::none()
                    .fill(egui::Color32::from_rgb(255, 255, 255))
                    .stroke(egui::Stroke::new(
                        1.0,
                        egui::Color32::from_rgb(218, 220, 224),
                    ))
                    .rounding(egui::Rounding::same(4.0))
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                if ui.button("◀").clicked() {}
                                ui.strong("April 2025");
                                if ui.button("▶").clicked() {}
                            });
                            ui.separator();
                            ui.horizontal(|ui| {
                                for day in ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"] {
                                    ui.label(day);
                                }
                            });
                            for week in 0..5 {
                                ui.horizontal(|ui| {
                                    for day in 1..=7 {
                                        let day_num = week * 7 + day;
                                        if day_num <= 30 {
                                            // Check if there are events on this day
                                            let has_events =
                                                feature_data.calendar_data.events.iter().any(
                                                    |event| {
                                                        let event_day =
                                                            event.start_time.date_naive().day();
                                                        event_day == day_num as u32
                                                            && event.start_time.date_naive().month()
                                                                == 4
                                                    },
                                                );

                                            let text = RichText::new(format!("{}", day_num)).color(
                                                if has_events {
                                                    egui::Color32::from_rgb(66, 133, 244)
                                                } else {
                                                    egui::Color32::from_rgb(40, 50, 60)
                                                },
                                            );

                                            let text =
                                                if has_events { text.strong() } else { text };
                                            let button = egui::Button::new(text).frame(false);

                                            ui.add(button);
                                        } else {
                                            ui.label("");
                                        }
                                    }
                                });
                            }
                        });
                    });
                ui.separator();

                // Events list
                ui.vertical(|ui| {
                    ui.strong("Upcoming Events");
                    ui.separator();

                    if feature_data.calendar_data.events.is_empty() {
                        ui.label("No upcoming events");
                    } else {
                        // Sort events by start time
                        let mut events = feature_data.calendar_data.events.clone();
                        events.sort_by(|a, b| a.start_time.cmp(&b.start_time));

                        for event in events {
                            render_event(ui, &event);
                            ui.add_space(4.0);
                        }
                    }
                });
            });
        });
    } else {
        // No active feature data
        app_layout::create_content_frame().show(ui, |ui| {
            ui.centered_and_justified(|ui| {
                ui.label(
                    RichText::new("No circle selected or data not available")
                        .size(16.0)
                        .color(egui::Color32::from_rgb(100, 100, 100)),
                );
            });
        });
    }
}

fn render_event(ui: &mut egui::Ui, event: &Event) {
    egui::Frame::none()
        .fill(egui::Color32::from_rgb(232, 240, 254))
        .rounding(egui::Rounding::same(4.0))
        .inner_margin(egui::Margin::same(8.0))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.strong(&event.title);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let date = format!(
                        "{} {}",
                        match event.start_time.date_naive().month() {
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
                        },
                        event.start_time.date_naive().day()
                    );
                    ui.label(date);
                });
            });

            // Format time
            let start_time = format!(
                "{:02}:{:02}",
                event.start_time.time().hour(),
                event.start_time.time().minute()
            );
            let end_time = format!(
                "{:02}:{:02}",
                event.end_time.time().hour(),
                event.end_time.time().minute()
            );
            ui.label(format!("{} - {}", start_time, end_time));

            // Show description
            ui.label(RichText::new(&event.description).weak());

            // Show location if available
            if let Some(location) = &event.location {
                ui.label(RichText::new(format!("📍 {}", location)).weak());
            }

            // Show attendees
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
                ui.label(RichText::new(format!("👥 {}", attendees)).weak());
            }
        });
}
