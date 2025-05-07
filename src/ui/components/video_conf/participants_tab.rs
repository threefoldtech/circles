use eframe::egui::{self, Align, Button, Layout, RichText, ScrollArea};
use egui_phosphor::regular::{CAMERA, CAMERA_SLASH, MICROPHONE, MICROPHONE_SLASH};
use uuid::Uuid;

use crate::models::features::video_conf::{ParticipantRole, VideoConferenceState};
use crate::utils::config::Theme;

/// Participants tab component for the video conference meeting
pub struct ParticipantsTab;

impl ParticipantsTab {
    /// Render the participants tab
    pub fn render(ui: &mut egui::Ui, state: &mut VideoConferenceState, theme: &Theme, user_id: Uuid) {
        if let Some(meeting) = state.current_meeting.as_ref() {
            let is_host = meeting.host_id == user_id;
            let is_co_host = meeting.co_host_ids.contains(&user_id);
            let can_manage = is_host || is_co_host;

            ui.vertical(|ui| {
                ui.heading(RichText::new("Participants").color(theme.header_text));
                ui.add_space(10.0);

                // Participant count
                ui.label(
                    RichText::new(format!("{} participants", meeting.participants.len()))
                        .color(theme.secondary_text),
                );
                ui.add_space(5.0);

                // Participants list
                ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        for participant in &meeting.participants {
                            let is_self = participant.id == user_id;
                            let role_text = match participant.role {
                                ParticipantRole::Host => " (Host)",
                                ParticipantRole::CoHost => " (Co-Host)",
                                ParticipantRole::Participant => "",
                            };

                            ui.horizontal(|ui| {
                                // Participant name with role
                                let name_text = if is_self {
                                    format!("{}{} (You)", participant.name, role_text)
                                } else {
                                    format!("{}{}", participant.name, role_text)
                                };

                                ui.label(RichText::new(name_text).color(theme.text));

                                // Push controls to the right
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    // Only show controls for others if user is host/co-host
                                    if !is_self && can_manage {
                                        // Remove participant button (host only)
                                        if is_host {
                                            if ui.small_button("Remove").clicked() {
                                                // In a real implementation, this would remove the participant
                                                // For now, we'll just print a message
                                                println!("Removing participant: {}", participant.name);
                                            }
                                        }

                                        // Mute/unmute button
                                        let mic_icon = if participant.mic_on {
                                            MICROPHONE
                                        } else {
                                            MICROPHONE_SLASH
                                        };
                                        if ui.small_button(mic_icon).clicked() {
                                            // In a real implementation, this would toggle the participant's mic
                                            println!(
                                                "{} participant: {}",
                                                if participant.mic_on { "Muting" } else { "Unmuting" },
                                                participant.name
                                            );
                                        }

                                        // Camera on/off button
                                        let camera_icon = if participant.camera_on {
                                            CAMERA
                                        } else {
                                            CAMERA_SLASH
                                        };
                                        if ui.small_button(camera_icon).clicked() {
                                            // In a real implementation, this would toggle the participant's camera
                                            println!(
                                                "{} camera for: {}",
                                                if participant.camera_on {
                                                    "Turning off"
                                                } else {
                                                    "Turning on"
                                                },
                                                participant.name
                                            );
                                        }
                                    }

                                    // Show status icons for all participants
                                    if participant.is_active_speaker {
                                        ui.label(RichText::new("🔊").color(theme.accent));
                                    }
                                    if participant.hand_raised {
                                        ui.label(RichText::new("✋").color(theme.accent));
                                    }
                                    if !participant.mic_on {
                                        ui.label(RichText::new(MICROPHONE_SLASH).color(theme.secondary_text));
                                    }
                                    if !participant.camera_on {
                                        ui.label(RichText::new(CAMERA_SLASH).color(theme.secondary_text));
                                    }
                                });
                            });
                            ui.separator();
                        }
                    });
            });
        }
    }
}
