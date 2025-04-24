use chrono::Timelike;
use egui::{Button, Frame, Margin, RichText, Stroke, Ui};

use super::super::event::Event;
use crate::utils::config::Theme;

#[allow(dead_code)]
pub fn render_event(ui: &mut Ui, event: &Event, theme: &Theme) {
    Frame::default()
        .fill(event.color.linear_multiply(0.7))
        .stroke(Stroke::new(1.0, event.color))
        .corner_radius(6.0)
        .inner_margin(Margin::same(10))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.strong(RichText::new(&event.title).size(16.0).color(theme.text));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let date = event.start_time.format("%b %d").to_string();
                    ui.label(RichText::new(date).color(theme.accent));
                });
            });

            ui.add_space(4.0);

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
            ui.label(RichText::new(format!("⏱️ {} - {}", start_time, end_time)).color(theme.text));

            ui.add_space(4.0);

            if !event.description.is_empty() {
                ui.label(RichText::new(&event.description).weak().color(theme.text));
                ui.add_space(2.0);
            }

            if let Some(location) = &event.location {
                ui.label(
                    RichText::new(format!("📍 {}", location))
                        .weak()
                        .color(theme.text),
                );
            }

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

            ui.horizontal(|ui| {
                if ui
                    .add(Button::new(RichText::new("Edit").color(theme.text)))
                    .clicked()
                {
                    println!("Edit button clicked for event: {}", event.id);
                }

                if ui
                    .add(Button::new(RichText::new("Delete").color(theme.error)))
                    .clicked()
                {
                    println!("Delete button clicked for event: {}", event.id);
                }
            });
        });
}
