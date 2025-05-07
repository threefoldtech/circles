use eframe::egui::{self, RichText};
use uuid::Uuid;

use crate::models::features::video_conf::VideoConferenceState;
use crate::utils::config::Theme;

/// Top bar component for the video conference meeting
pub struct MeetingAppBar;

impl MeetingAppBar {
    /// Render the meeting app bar
    pub fn render(
        ui: &mut egui::Ui,
        state: &mut VideoConferenceState,
        theme: &Theme,
        user_id: Uuid,
    ) {
        if let Some(meeting) = state.current_meeting.as_ref() {
            let is_host = meeting.host_id == user_id;

            // Top bar with meeting info
            ui.horizontal(|ui| {
                // Meeting name
                ui.heading(&meeting.name);
                ui.add_space(10.0);

                // Meeting ID
                ui.label(format!("Meeting ID: {}", meeting.meeting_id));
                ui.add_space(10.0);

                // Meeting duration
                ui.label(format!("Duration: {}", meeting.get_duration_string()));

                // Push buttons to the right
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // End meeting button (only for host)
                    if is_host {
                        if ui
                            .button(RichText::new("End Meeting").color(theme.error))
                            .clicked()
                        {
                            state.end_confirmation_open = true;
                        }
                    }

                    // Leave meeting button
                    if ui
                        .button(RichText::new("Leave").color(theme.error))
                        .clicked()
                    {
                        state.leave_confirmation_open = true;
                    }

                    // Toggle side panel button
                    let panel_text = if state.side_panel_open {
                        "Hide Panel"
                    } else {
                        "Show Panel"
                    };
                    if ui.button(panel_text).clicked() {
                        state.side_panel_open = !state.side_panel_open;
                    }
                });
            });
        }
    }
}
