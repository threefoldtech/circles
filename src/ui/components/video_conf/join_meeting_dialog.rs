use eframe::egui::{self, Align2, Button, RichText, Vec2};
use uuid::Uuid;

use crate::models::features::video_conf::{Meeting, Participant, ParticipantRole, VideoConferenceState};
use crate::utils::config::Theme;
use chrono::Utc;

/// Join meeting dialog component
pub struct JoinMeetingDialog;

impl JoinMeetingDialog {
    /// Render the join meeting dialog
    pub fn render(
        ui: &mut egui::Ui,
        state: &mut VideoConferenceState,
        theme: &Theme,
        user_id: Uuid,
        user_name: &str,
    ) {
        let ctx = ui.ctx();

        egui::Window::new("Join Meeting")
            .fixed_size([400.0, 200.0])
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .collapsible(false)
            .frame(
                egui::Frame::window(&ctx.style())
                    .fill(theme.panel)
                    .shadow(egui::epaint::Shadow {
                        color: theme.shadow,
                        offset: [0, 4],
                        blur: 8,
                        spread: 0,
                    }),
            )
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading(RichText::new("Join Meeting").color(theme.header_text));
                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Meeting ID:").color(theme.text));
                        ui.text_edit_singleline(&mut state.meeting_id_input);
                    });

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Password (if required):").color(theme.text));
                        ui.text_edit_singleline(&mut state.password_input);
                    });

                    ui.add_space(20.0);

                    ui.horizontal(|ui| {
                        if ui
                            .add(
                                Button::new(RichText::new("Cancel").color(theme.text))
                                    .fill(theme.button_secondary)
                                    .min_size(Vec2::new(80.0, 30.0)),
                            )
                            .clicked()
                        {
                            state.join_dialog_open = false;
                        }

                        ui.add_space(10.0);

                        if ui
                            .add(
                                Button::new(RichText::new("Join").color(theme.light_color))
                                    .fill(theme.button_primary)
                                    .min_size(Vec2::new(80.0, 30.0)),
                            )
                            .clicked()
                        {
                            if !state.meeting_id_input.is_empty() {
                                // Create a new meeting
                                let meeting_id = Uuid::new_v4();
                                let mut meeting = Meeting {
                                    id: meeting_id,
                                    name: "Joined Meeting".to_string(),
                                    meeting_id: state.meeting_id_input.clone(),
                                    password: None,
                                    host_id: user_id, // Set the user as the host for now
                                    co_host_ids: Vec::new(),
                                    participants: Vec::new(),
                                    chat_messages: Vec::new(),
                                    settings:
                                        crate::models::features::video_conf::MeetingSettings::default(),
                                    start_time: Utc::now(),
                                    duration: 0, // Unlimited
                                    is_locked: false,
                                    waiting_room_enabled: false,
                                    waiting_room: Vec::new(),
                                    has_ended: false,
                                };

                                // Add the current user as a participant
                                let participant = Participant {
                                    id: user_id,
                                    name: user_name.to_string(),
                                    camera_on: state.camera_on,
                                    mic_on: state.mic_on,
                                    is_sharing_screen: false,
                                    hand_raised: false,
                                    is_active_speaker: false,
                                    role: ParticipantRole::Participant, // Not the host in this case
                                    joined_at: Utc::now(),
                                };

                                meeting.add_participant(participant);

                                // Set the meeting
                                state.current_meeting = Some(meeting);
                                state.in_meeting = true;
                                state.join_dialog_open = false;
                            }
                        }
                    });
                });
            });
    }
}
