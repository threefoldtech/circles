use eframe::egui::{self, Align, Button, Color32, Layout, RichText, Vec2};
use egui_phosphor::regular::{
    CAMERA, CAMERA_SLASH, CHAT_CENTERED_TEXT, DESKTOP, GEAR, HAND_PALM, MICROPHONE,
    MICROPHONE_SLASH, PHONE_DISCONNECT, USERS,
};
use uuid::Uuid;

use crate::models::features::video_conf::VideoConferenceState;
use crate::utils::config::Theme;

/// Bottom controls component for the video conference meeting
pub struct MeetingFooter;

impl MeetingFooter {
    /// Render the meeting footer with controls
    pub fn render(
        ui: &mut egui::Ui,
        state: &mut VideoConferenceState,
        theme: &Theme,
        user_id: Uuid,
    ) {
        if let Some(meeting) = state.current_meeting.as_mut() {
            let is_host = meeting.host_id == user_id;

            // Get current state
            let is_mic_on = state.mic_on;
            let is_camera_on = state.camera_on;
            let is_hand_raised =
                if let Some(index) = meeting.participants.iter().position(|p| p.id == user_id) {
                    meeting.participants[index].hand_raised
                } else {
                    false
                };

            // Create a Zoom-like footer with black background
            // Center the controls in the available space
            ui.vertical_centered(|ui| {
                // Create a fixed-width container for the controls
                let control_width = 500.0; // Fixed width for the control bar

                // Create a centered container for the controls
                let container = egui::Frame::none()
                    .fill(Color32::from_rgb(30, 30, 30))
                    .rounding(8.0)
                    .inner_margin(10.0);

                ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
                    container.show(ui, |ui| {
                        // Set a fixed width for the control container
                        ui.set_width(control_width);

                        // Main row of controls
                        ui.horizontal_centered(|ui| {
                            // Microphone toggle
                            ui.vertical(|ui| {
                                let mic_button = egui::Button::new(
                                    if is_mic_on {
                                        RichText::new("🎙️").size(20.0)
                                    } else {
                                        RichText::new("🔇").size(20.0)
                                    }
                                )
                                .fill(Color32::from_rgb(60, 60, 60))
                                .min_size(Vec2::new(44.0, 44.0))
                                .rounding(8.0);

                                if ui.add(mic_button).clicked() {
                                    // Toggle microphone
                                    state.mic_on = !is_mic_on;

                                    // Update participant state
                                    if let Some(index) = meeting.participants.iter().position(|p| p.id == user_id) {
                                        meeting.participants[index].mic_on = !is_mic_on;
                                    }
                                }

                                ui.label(
                                    RichText::new(if is_mic_on { "Mute" } else { "Unmute" })
                                        .size(11.0)
                                        .color(Color32::WHITE)
                                );
                            });

                            ui.add_space(15.0);

                            // Camera toggle
                            ui.vertical(|ui| {
                                let camera_button = egui::Button::new(
                                    if is_camera_on {
                                        RichText::new("📹").size(20.0)
                                    } else {
                                        RichText::new("🚫").size(20.0)
                                    }
                                )
                                .fill(Color32::from_rgb(60, 60, 60))
                                .min_size(Vec2::new(44.0, 44.0))
                                .rounding(8.0);

                                if ui.add(camera_button).clicked() {
                                    // Toggle camera
                                    state.camera_on = !is_camera_on;

                                    // Update participant state
                                    if let Some(index) = meeting.participants.iter().position(|p| p.id == user_id) {
                                        meeting.participants[index].camera_on = !is_camera_on;
                                    }
                                }

                                ui.label(
                                    RichText::new(if is_camera_on { "Stop Video" } else { "Start Video" })
                                        .size(11.0)
                                        .color(Color32::WHITE)
                                );
                            });

                            ui.add_space(15.0);

                            // Share screen button
                            ui.vertical(|ui| {
                                let share_button = egui::Button::new(
                                    RichText::new("🖥️").size(20.0)
                                )
                                .fill(Color32::from_rgb(60, 60, 60))
                                .min_size(Vec2::new(44.0, 44.0))
                                .rounding(8.0);

                                if ui.add(share_button).clicked() {
                                    // Share screen functionality would go here
                                    println!("Share screen clicked");
                                }

                                ui.label(
                                    RichText::new("Share Screen")
                                        .size(11.0)
                                        .color(Color32::WHITE)
                                );
                            });

                            ui.add_space(15.0);

                            // Chat button
                            ui.vertical(|ui| {
                                let chat_button = egui::Button::new(
                                    RichText::new("💬").size(20.0)
                                )
                                .fill(Color32::from_rgb(60, 60, 60))
                                .min_size(Vec2::new(44.0, 44.0))
                                .rounding(8.0);

                                if ui.add(chat_button).clicked() {
                                    // Toggle chat panel
                                    state.side_panel_open = !state.side_panel_open;
                                    if state.side_panel_open {
                                        state.side_panel_tab = crate::models::features::video_conf::SidePanelTab::Chat;
                                    }
                                }

                                ui.label(
                                    RichText::new("Chat")
                                        .size(11.0)
                                        .color(Color32::WHITE)
                                );
                            });

                            ui.add_space(15.0);

                            // Raise hand button
                            ui.vertical(|ui| {
                                let hand_button = egui::Button::new(
                                    RichText::new("✋").size(20.0).color(
                                        if is_hand_raised {
                                            Color32::from_rgb(255, 255, 0)
                                        } else {
                                            Color32::WHITE
                                        }
                                    )
                                )
                                .fill(Color32::from_rgb(60, 60, 60))
                                .min_size(Vec2::new(44.0, 44.0))
                                .rounding(8.0);

                                if ui.add(hand_button).clicked() {
                                    // Toggle hand raised
                                    if let Some(index) = meeting.participants.iter().position(|p| p.id == user_id) {
                                        meeting.participants[index].hand_raised = !is_hand_raised;
                                    }
                                }

                                ui.label(
                                    RichText::new("Raise Hand")
                                        .size(11.0)
                                        .color(Color32::WHITE)
                                );
                            });

                            ui.add_space(15.0);

                            // End/Leave meeting button
                            ui.vertical(|ui| {
                                let end_button = egui::Button::new(
                                    RichText::new("❌").size(20.0)
                                )
                                .fill(Color32::from_rgb(224, 60, 60))
                                .min_size(Vec2::new(44.0, 44.0))
                                .rounding(8.0);

                                if ui.add(end_button).clicked() {
                                    if is_host {
                                        state.end_confirmation_open = true;
                                    } else {
                                        state.leave_confirmation_open = true;
                                    }
                                }

                                ui.label(
                                    RichText::new(if is_host { "End" } else { "Leave" })
                                        .size(11.0)
                                        .color(Color32::WHITE)
                                );
                            });
                        });
                    });
                });
            });
        }
    }
}
