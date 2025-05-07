use eframe::egui::{self, Align2, Button, RichText, Vec2};
use rand::prelude::*;
use uuid::Uuid;

use crate::models::features::video_conf::{Meeting, Participant, ParticipantRole, VideoConferenceState};
use crate::models::user::User;
use crate::utils::config::Theme;
use chrono::Utc;

/// Create meeting dialog component
pub struct CreateMeetingDialog;

impl CreateMeetingDialog {
    /// Render the create meeting dialog
    pub fn render(
        ui: &mut egui::Ui,
        state: &mut VideoConferenceState,
        theme: &Theme,
        user_id: Uuid,
        user_name: &str,
    ) {
        let ctx = ui.ctx();

        egui::Window::new("Create Meeting")
            .fixed_size([400.0, 250.0])
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
                    ui.heading(RichText::new("Create Meeting").color(theme.header_text));
                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Meeting Name:").color(theme.text));
                        ui.text_edit_singleline(&mut state.meeting_name_input);
                    });

                    // Set text color before adding the checkbox
                    ui.style_mut().visuals.override_text_color = Some(theme.text);
                    ui.checkbox(&mut state.require_password, "Require Password");

                    if state.require_password {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Password:").color(theme.text));
                            ui.text_edit_singleline(&mut state.create_password_input);
                        });
                    }

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
                            state.create_dialog_open = false;
                        }

                        ui.add_space(10.0);

                        if ui
                            .add(
                                Button::new(RichText::new("Create").color(theme.light_color))
                                    .fill(theme.button_primary)
                                    .min_size(Vec2::new(80.0, 30.0)),
                            )
                            .clicked()
                        {
                            if !state.meeting_name_input.is_empty() {
                                // Generate a meeting ID
                                fn generate_meeting_id() -> String {
                                    let mut rng = rand::thread_rng();
                                    let part1: u16 = rng.gen_range(100..1000);
                                    let part2: u16 = rng.gen_range(100..1000);
                                    let part3: u16 = rng.gen_range(100..1000);
                                    format!("{}-{}-{}", part1, part2, part3)
                                }

                                // Create a meeting using the Meeting::new method
                                let mut meeting = Meeting::new(
                                    state.meeting_name_input.clone(),
                                    &User {
                                        id: user_id,
                                        name: user_name.to_string(),
                                        email: "user@example.com".to_string(),
                                        created_at: Utc::now(),
                                        preferences: crate::models::user::UserPreferences::default(),
                                    },
                                );

                                // Set password if required
                                if state.require_password && !state.create_password_input.is_empty() {
                                    meeting.password = Some(state.create_password_input.clone());
                                }

                                // Add the current user as a participant (host)
                                let participant = Participant {
                                    id: user_id,
                                    name: user_name.to_string(),
                                    camera_on: state.camera_on,
                                    mic_on: state.mic_on,
                                    is_sharing_screen: false,
                                    hand_raised: false,
                                    is_active_speaker: false,
                                    role: ParticipantRole::Host,
                                    joined_at: Utc::now(),
                                };

                                meeting.add_participant(participant);

                                // Set the meeting
                                state.current_meeting = Some(meeting);
                                state.in_meeting = true;
                                state.create_dialog_open = false;
                            }
                        }
                    });
                });
            });
    }
}
