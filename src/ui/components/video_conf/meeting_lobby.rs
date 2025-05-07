use eframe::egui::{self, Align, Button, Layout, RichText, Vec2};

use crate::models::features::video_conf::VideoConferenceState;
use crate::utils::config::Theme;

/// Meeting lobby component
pub struct MeetingLobby;

impl MeetingLobby {
    /// Render the meeting lobby where users can start or join a meeting
    pub fn render(ui: &mut egui::Ui, state: &mut VideoConferenceState, theme: &Theme, user_name: &str) {
        ui.vertical_centered(|ui| {
            ui.add_space(40.0);

            // Title with styling
            ui.heading(
                RichText::new("Video Conference")
                    .size(28.0)
                    .color(theme.accent),
            );
            ui.add_space(10.0);
            ui.label(
                RichText::new("Start or join a video meeting")
                    .size(16.0)
                    .color(theme.secondary_text),
            );

            ui.add_space(40.0);

            // Main buttons
            ui.horizontal_centered(|ui| {
                let button_size = Vec2::new(200.0, 120.0);

                // Start Meeting Button
                let start_button = ui.add_sized(
                    button_size,
                    Button::new(RichText::new("Start Meeting").size(18.0).color(theme.text))
                        .fill(theme.accent),
                );
                if start_button.clicked() {
                    state.create_dialog_open = true;
                    state.meeting_name_input = format!("{}'s Meeting", user_name);
                }

                ui.add_space(20.0);

                // Join Meeting Button
                let join_button = ui.add_sized(
                    button_size,
                    Button::new(RichText::new("Join Meeting").size(18.0).color(theme.text))
                        .fill(theme.accent),
                );
                if join_button.clicked() {
                    state.join_dialog_open = true;
                    state.meeting_id_input.clear();
                    state.password_input.clear();
                }
            });

            ui.add_space(40.0);

            // Recent meetings section
            egui::Frame::none()
                .fill(theme.panel)
                .rounding(8.0)
                .inner_margin(16.0)
                .show(ui, |ui| {
                    ui.heading(RichText::new("Recent Meetings").color(theme.header_text));
                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(10.0);

                    // Display some mock recent meetings
                    for i in 0..3 {
                        ui.horizontal(|ui| {
                            let meeting_name = match i {
                                0 => "Weekly Team Sync",
                                1 => "Project Planning",
                                2 => "Client Meeting",
                                _ => "Meeting",
                            };

                            let date = match i {
                                0 => "Today, 10:00 AM",
                                1 => "Yesterday, 2:30 PM",
                                2 => "May 3, 11:00 AM",
                                _ => "",
                            };

                            ui.label(RichText::new(meeting_name).size(16.0));
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                if ui
                                    .button(RichText::new("Join").color(theme.accent))
                                    .clicked()
                                {
                                    state.join_dialog_open = true;
                                    state.meeting_id_input = format!("123-456-{}", 789 + i);
                                }
                                ui.label(RichText::new(date).color(theme.secondary_text));
                            });
                        });

                        if i < 2 {
                            ui.separator();
                        }
                    }
                });
        });
    }
}
