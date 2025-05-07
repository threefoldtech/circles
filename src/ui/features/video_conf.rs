use eframe::egui::{self, Align, Align2, Button, Color32, Layout, RichText, Stroke, Vec2};
use egui::StrokeKind;
use egui_phosphor::regular::{
    CAMERA, CAMERA_SLASH, MICROPHONE, MICROPHONE_SLASH, PHONE_DISCONNECT,
};
use rand::prelude::*;

use crate::app::CircleApp;
use crate::models::features::video_conf::{
    Meeting, Participant, ParticipantRole, VideoConferenceState,
};
use crate::ui::app_layout::create_content_frame;
use crate::utils::config::Theme;
use chrono::Utc;
use uuid::Uuid;

/// Main rendering function for the video conference feature
pub fn render_video_conference(app: &mut CircleApp, ui: &mut egui::Ui) {
    ui.add_space(16.0);
    let theme = app.get_current_theme();

    // Initialize video conference state if it doesn't exist
    if !app
        .circle_feature_data
        .contains_key(&app.active_circle_id.unwrap_or_default())
    {
        return;
    }

    let circle_id = app.active_circle_id.unwrap_or_default();
    let user_id = app.user.as_ref().map_or_else(Uuid::nil, |u| u.id);
    let user_name = app
        .user
        .as_ref()
        .map_or("User".to_string(), |u| u.name.clone());

    // Get a reference to the feature data
    if let Some(feature_data) = app.circle_feature_data.get_mut(&circle_id) {
        // Initialize video conference state if it doesn't exist
        if feature_data.video_conf_state.is_none() {
            feature_data.video_conf_state = Some(VideoConferenceState::default());
        }

        // Get a mutable reference to the state
        let state = feature_data.video_conf_state.as_mut().unwrap();

        create_content_frame(&theme).show(ui, |ui| {
            if state.in_meeting {
                render_active_meeting(ui, state, &theme, user_id);
            } else {
                render_meeting_lobby(ui, state, &theme, &user_name);
            }
        });

        // Clone the state for dialogs to avoid borrowing issues
        let mut dialog_state = state.clone();

        // Render dialogs
        if dialog_state.join_dialog_open {
            render_join_meeting_dialog(ui, &mut dialog_state, &theme, user_id, &user_name);
            // Update the original state
            state.join_dialog_open = dialog_state.join_dialog_open;
            state.current_meeting = dialog_state.current_meeting.clone();
            state.in_meeting = dialog_state.in_meeting;
        }

        if dialog_state.create_dialog_open {
            render_create_meeting_dialog(ui, &mut dialog_state, &theme, user_id, &user_name);
            // Update the original state
            state.create_dialog_open = dialog_state.create_dialog_open;
            state.current_meeting = dialog_state.current_meeting.clone();
            state.in_meeting = dialog_state.in_meeting;
        }

        // Clone the state again for more dialogs
        let mut dialog_state2 = state.clone();

        if dialog_state2.leave_confirmation_open {
            render_leave_meeting_dialog(ui, &mut dialog_state2, &theme);
            // Update the original state
            state.leave_confirmation_open = dialog_state2.leave_confirmation_open;
            state.in_meeting = dialog_state2.in_meeting;
            state.current_meeting = dialog_state2.current_meeting.clone();
        }

        if dialog_state2.end_confirmation_open {
            render_end_meeting_dialog(ui, &mut dialog_state2, &theme);
            // Update the original state
            state.end_confirmation_open = dialog_state2.end_confirmation_open;
            state.in_meeting = dialog_state2.in_meeting;
            state.current_meeting = dialog_state2.current_meeting.clone();
        }
    }
}

/// Render the meeting lobby where users can start or join a meeting
fn render_meeting_lobby(
    ui: &mut egui::Ui,
    state: &mut VideoConferenceState,
    theme: &Theme,
    user_name: &str,
) {
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
        ui.group(|ui| {
            ui.heading("Recent Meetings");
            ui.add_space(10.0);
            ui.separator();

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

/// Render the active meeting UI
fn render_active_meeting(
    ui: &mut egui::Ui,
    state: &mut VideoConferenceState,
    theme: &Theme,
    user_id: Uuid,
) {
    // Check if there's a current meeting
    let meeting_option = state.current_meeting.as_mut();
    if meeting_option.is_none() {
        state.in_meeting = false;
        return;
    }

    let meeting = meeting_option.unwrap();
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

        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            // End/Leave button
            let button_text = if is_host { "End Meeting" } else { "Leave" };
            if ui.button(button_text).clicked() {
                if is_host {
                    state.end_confirmation_open = true;
                } else {
                    state.leave_confirmation_open = true;
                }
            }
        });
    });
    ui.separator();

    // Video grid - Zoom-like layout
    let available_width = ui.available_width();
    let available_height = ui.available_height() - 60.0; // Leave space for bottom bar

    ui.vertical(|ui| {
        ui.add_space(10.0);

        // Display participants in a grid
        let participants = &meeting.participants;
        if participants.is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(available_height / 3.0);
                ui.label(
                    RichText::new("No participants in the meeting")
                        .size(18.0)
                        .color(theme.secondary_text),
                );
            });
        } else {
            // Calculate grid dimensions
            let participant_count = participants.len();
            let (cols, rows) = calculate_grid_dimensions(participant_count);

            // Calculate video tile size
            let padding = 8.0;
            let tile_width = (available_width / cols as f32) - padding;
            let tile_height = (available_height / rows as f32) - padding;
            let tile_size = Vec2::new(tile_width, tile_height);

            // Render video grid
            let mut row = 0;
            let mut col = 0;

            for participant in participants {
                if col >= cols {
                    col = 0;
                    row += 1;
                }

                let rect = egui::Rect::from_min_size(
                    egui::Pos2::new(
                        ui.min_rect().min.x + col as f32 * (tile_width + padding),
                        ui.min_rect().min.y + row as f32 * (tile_height + padding),
                    ),
                    tile_size,
                );

                // Draw video tile
                render_video_tile(ui, rect, participant, theme, participant.id == user_id);

                col += 1;
            }
        }
    });

    // Bottom bar with controls
    ui.add_space(20.0);
    ui.separator();

    ui.horizontal_centered(|ui| {
        // Get current state
        let is_mic_on = state.mic_on;
        let is_camera_on = state.camera_on;

        // Microphone toggle
        let mic_icon = if is_mic_on {
            MICROPHONE
        } else {
            MICROPHONE_SLASH
        };
        let mic_text = if is_mic_on { "Mute" } else { "Unmute" };

        if ui.button(format!("{} {}", mic_icon, mic_text)).clicked() {
            // Toggle microphone
            state.mic_on = !is_mic_on;

            // Update participant state
            if let Some(index) = meeting.participants.iter().position(|p| p.id == user_id) {
                meeting.participants[index].mic_on = !is_mic_on;
            }
        }

        ui.add_space(10.0);

        // Camera toggle
        let camera_icon = if is_camera_on { CAMERA } else { CAMERA_SLASH };
        let camera_text = if is_camera_on {
            "Stop Video"
        } else {
            "Start Video"
        };

        if ui
            .button(format!("{} {}", camera_icon, camera_text))
            .clicked()
        {
            // Toggle camera
            state.camera_on = !is_camera_on;

            // Update participant state
            if let Some(index) = meeting.participants.iter().position(|p| p.id == user_id) {
                meeting.participants[index].camera_on = !is_camera_on;
            }
        }

        ui.add_space(10.0);

        // Leave/End meeting button
        let leave_text = if is_host { "End Meeting" } else { "Leave" };
        let leave_button = ui.add(
            Button::new(format!("{} {}", PHONE_DISCONNECT, leave_text))
                .fill(Color32::from_rgb(220, 38, 38)),
        );

        if leave_button.clicked() {
            if is_host {
                state.end_confirmation_open = true;
            } else {
                state.leave_confirmation_open = true;
            }
        }
    });
}

/// Calculate the number of columns and rows for the video grid
fn calculate_grid_dimensions(count: usize) -> (usize, usize) {
    match count {
        0 => (1, 1),
        1 => (1, 1),
        2 => (2, 1),
        3..=4 => (2, 2),
        5..=6 => (3, 2),
        7..=9 => (3, 3),
        10..=12 => (4, 3),
        13..=16 => (4, 4),
        _ => {
            let cols = (count as f64).sqrt().ceil() as usize;
            let rows = (count + cols - 1) / cols;
            (cols, rows)
        }
    }
}

/// Render a single video tile for a participant (Zoom-like style)
fn render_video_tile(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    participant: &Participant,
    theme: &Theme,
    is_self: bool,
) {
    let painter = ui.painter();

    // Draw video background
    let bg_color = if participant.is_active_speaker {
        theme.accent.linear_multiply(0.2)
    } else {
        Color32::from_rgb(20, 20, 30)
    };

    let border_color = if participant.is_active_speaker {
        theme.accent
    } else if is_self {
        Color32::from_rgb(100, 100, 255)
    } else {
        Color32::from_rgb(60, 60, 80)
    };

    // Draw the video tile background with rounded corners
    painter.rect_filled(rect, 8.0, bg_color);
    let stroke = Stroke::new(1.5, border_color);
    painter.rect_stroke(rect, 8.0, stroke, StrokeKind::Inside);

    if !participant.camera_on {
        // Camera is off - show avatar with initials
        let initial = participant
            .name
            .chars()
            .next()
            .unwrap_or('?')
            .to_uppercase()
            .next()
            .unwrap_or('?');
        let avatar_radius = rect.height() / 5.0;
        let avatar_pos = egui::pos2(rect.center().x, rect.center().y - avatar_radius / 2.0);

        // Draw avatar circle
        painter.circle_filled(avatar_pos, avatar_radius, theme.accent.linear_multiply(0.7));

        // Draw initial
        painter.text(
            avatar_pos,
            Align2::CENTER_CENTER,
            initial.to_string(),
            egui::FontId::proportional(avatar_radius * 1.2),
            Color32::WHITE,
        );

        // Draw camera off indicator
        painter.text(
            egui::pos2(rect.center().x, rect.center().y + avatar_radius * 1.5),
            Align2::CENTER_CENTER,
            "Camera Off",
            egui::FontId::proportional(14.0),
            Color32::from_rgb(200, 200, 200),
        );
    } else {
        // Camera is on - simulate video feed with a placeholder
        // In a real implementation, this would show the actual video feed
        let video_rect = rect.shrink(10.0);
        painter.rect_filled(video_rect, 4.0, Color32::from_rgb(40, 40, 60));

        // Draw a simple avatar silhouette to simulate a person
        let head_radius = video_rect.height() / 8.0;
        let head_pos = egui::pos2(
            video_rect.center().x,
            video_rect.min.y + video_rect.height() * 0.3,
        );

        // Draw head
        painter.circle_filled(head_pos, head_radius, Color32::from_rgb(100, 100, 120));

        // Draw body
        let body_top = egui::pos2(head_pos.x, head_pos.y + head_radius);
        let body_width = head_radius * 1.5;
        let body_height = head_radius * 3.0;
        let body_rect = egui::Rect::from_center_size(
            egui::pos2(body_top.x, body_top.y + body_height / 2.0),
            egui::vec2(body_width, body_height),
        );

        painter.rect_filled(body_rect, 4.0, Color32::from_rgb(100, 100, 120));
    }

    // Draw participant name and status indicators at the bottom of the tile
    let bottom_rect = egui::Rect::from_min_max(egui::pos2(rect.min.x, rect.max.y - 30.0), rect.max);

    painter.rect_filled(
        bottom_rect,
        0.0,
        Color32::from_rgba_premultiplied(0, 0, 0, 180),
    );

    // Allocate UI for the bottom bar
    #[allow(deprecated)]
    ui.allocate_ui_at_rect(bottom_rect, |ui| {
        ui.horizontal(|ui| {
            // Participant name
            ui.label(RichText::new(&participant.name).color(Color32::WHITE));

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                // Mic status
                let mic_icon = if participant.mic_on {
                    MICROPHONE
                } else {
                    MICROPHONE_SLASH
                };
                ui.label(RichText::new(mic_icon).color(Color32::WHITE));

                // Host/Co-host badge
                match participant.role {
                    ParticipantRole::Host => {
                        ui.label(RichText::new("Host").color(theme.accent));
                    }
                    ParticipantRole::CoHost => {
                        ui.label(RichText::new("Co-Host").color(theme.accent));
                    }
                    _ => {}
                }
            });
        });
    });
}

/// Render the join meeting dialog
fn render_join_meeting_dialog(
    ui: &mut egui::Ui,
    state: &mut VideoConferenceState,
    _theme: &Theme,
    user_id: Uuid,
    user_name: &str,
) {
    let ctx = ui.ctx();

    egui::Window::new("Join Meeting")
        .fixed_size([400.0, 200.0])
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .collapsible(false)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Join Meeting");
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.label("Meeting ID:");
                    ui.text_edit_singleline(&mut state.meeting_id_input);
                });

                ui.horizontal(|ui| {
                    ui.label("Password (if required):");
                    ui.text_edit_singleline(&mut state.password_input);
                });

                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        state.join_dialog_open = false;
                    }

                    if ui.button("Join").clicked() {
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

/// Render the create meeting dialog
fn render_create_meeting_dialog(
    ui: &mut egui::Ui,
    state: &mut VideoConferenceState,
    _theme: &Theme,
    user_id: Uuid,
    user_name: &str,
) {
    let ctx = ui.ctx();

    egui::Window::new("Create Meeting")
        .fixed_size([400.0, 250.0])
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .collapsible(false)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Create Meeting");
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.label("Meeting Name:");
                    ui.text_edit_singleline(&mut state.meeting_name_input);
                });

                ui.checkbox(&mut state.require_password, "Require Password");

                if state.require_password {
                    ui.horizontal(|ui| {
                        ui.label("Password:");
                        ui.text_edit_singleline(&mut state.create_password_input);
                    });
                }

                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        state.create_dialog_open = false;
                    }

                    if ui.button("Create").clicked() {
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
                                &crate::models::user::User {
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

/// Render the leave meeting confirmation dialog
fn render_leave_meeting_dialog(
    ui: &mut egui::Ui,
    state: &mut VideoConferenceState,
    _theme: &Theme,
) {
    let ctx = ui.ctx();

    egui::Window::new("Leave Meeting")
        .fixed_size([300.0, 150.0])
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .collapsible(false)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Leave Meeting");
                ui.add_space(10.0);

                ui.label("Are you sure you want to leave the meeting?");

                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        state.leave_confirmation_open = false;
                    }

                    if ui.button("Leave").clicked() {
                        state.leave_confirmation_open = false;

                        // In a real implementation, we would remove the participant from the meeting
                        // but we'll simplify it to avoid borrow issues

                        state.in_meeting = false;
                        state.current_meeting = None;
                    }
                });
            });
        });
}

/// Render the end meeting confirmation dialog
fn render_end_meeting_dialog(ui: &mut egui::Ui, state: &mut VideoConferenceState, _theme: &Theme) {
    let ctx = ui.ctx();

    egui::Window::new("End Meeting")
        .fixed_size([300.0, 150.0])
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .collapsible(false)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("End Meeting");
                ui.add_space(10.0);

                ui.label("Are you sure you want to end the meeting for all participants?");

                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        state.end_confirmation_open = false;
                    }

                    if ui.button("End Meeting").clicked() {
                        state.end_confirmation_open = false;

                        // Check if the meeting has reached its time limit
                        if let Some(meeting) = &state.current_meeting {
                            if meeting.is_time_limit_reached() {
                                // If time limit reached, we would show a notification
                                println!("Meeting has reached its time limit");
                            }
                        }

                        state.in_meeting = false;
                        state.current_meeting = None;
                    }
                });
            });
        });
}
