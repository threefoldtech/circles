use eframe::egui;

use crate::ui::app_layout;

pub fn render_calendar(ui: &mut egui::Ui) {
    app_layout::render_header(ui, "📅", "Calendar");
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
    });
    ui.add_space(16.0);
    app_layout::create_content_frame().show(ui, |ui| {
        ui.horizontal(|ui| {
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
                                        ui.add(
                                            egui::Button::new(format!("{}", day_num)).frame(false),
                                        );
                                    } else {
                                        ui.label("");
                                    }
                                }
                            });
                        }
                    });
                });
            ui.separator();
            ui.vertical(|ui| {
                ui.strong("Upcoming Events");
                ui.separator();
                for i in 1..=3 {
                    egui::Frame::none()
                        .fill(egui::Color32::from_rgb(232, 240, 254))
                        .rounding(egui::Rounding::same(4.0))
                        .inner_margin(egui::Margin::same(8.0))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.strong(format!("Event {}", i));
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        ui.label(format!("Apr {}", 10 + i));
                                    },
                                );
                            });
                            ui.label("10:00 - 11:00");
                            ui.label(egui::RichText::new("Meeting with team").weak());
                        });
                    ui.add_space(4.0);
                }
            });
        });
    });
}
