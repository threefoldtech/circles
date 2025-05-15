use eframe::egui::{self, Align, Layout, Margin};
use uuid::Uuid;

use crate::app::CircleApp;
use crate::models::features::video_conf::{SidePanelTab, VideoConferenceState};
use crate::ui::app_layout::create_content_frame;
use crate::ui::components::video_conf::{
    CreateMeetingDialog, EndMeetingDialog, JoinMeetingDialog, LeaveMeetingDialog, MeetingAppBar,
    MeetingChat, MeetingFooter, MeetingLobby, ParticipantsTab, VideoGrid,
};
use crate::utils::config::Theme;

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
                MeetingLobby::render(ui, state, &theme, &user_name);
            }
        });

        // Clone the state for dialogs to avoid borrowing issues
        let mut dialog_state = state.clone();

        // Render dialogs
        if dialog_state.join_dialog_open {
            JoinMeetingDialog::render(ui, &mut dialog_state, &theme, user_id, &user_name);
            // Update the original state
            state.join_dialog_open = dialog_state.join_dialog_open;
            state.current_meeting = dialog_state.current_meeting.clone();
            state.in_meeting = dialog_state.in_meeting;
        }

        if dialog_state.create_dialog_open {
            CreateMeetingDialog::render(ui, &mut dialog_state, &theme, user_id, &user_name);
            // Update the original state
            state.create_dialog_open = dialog_state.create_dialog_open;
            state.current_meeting = dialog_state.current_meeting.clone();
            state.in_meeting = dialog_state.in_meeting;
        }

        if dialog_state.leave_confirmation_open {
            LeaveMeetingDialog::render(ui, &mut dialog_state, &theme, user_id);
            // Update the original state
            state.leave_confirmation_open = dialog_state.leave_confirmation_open;
            state.current_meeting = dialog_state.current_meeting.clone();
            state.in_meeting = dialog_state.in_meeting;
        }

        if dialog_state.end_confirmation_open {
            EndMeetingDialog::render(ui, &mut dialog_state, &theme);
            // Update the original state
            state.end_confirmation_open = dialog_state.end_confirmation_open;
            state.current_meeting = dialog_state.current_meeting.clone();
            state.in_meeting = dialog_state.in_meeting;
        }
    }
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

    // Get available height and width
    let available_height = ui.available_height();
    let available_width = ui.available_width();

    // Calculate the optimal height for the video area (85% of available height)
    let optimal_height = available_height * 0.85;

    // Allocate space: 40px for top bar, 70px for bottom controls, rest for content
    let top_bar_height = 40.0;
    let bottom_bar_height = 70.0;
    let content_height = optimal_height - top_bar_height - bottom_bar_height;

    // Create a frame with the optimal height
    egui::Frame::none()
        .fill(theme.panel) // Use panel color for the outer frame
        .show(ui, |ui| {
            // Create a centered layout
            ui.vertical_centered(|ui| {
                ui.set_max_height(optimal_height);

                // Use a dark background for the video area
                egui::Frame::none().fill(theme.black).show(ui, |ui| {
                    ui.set_width(available_width);
                    ui.set_max_height(optimal_height);

                    // Top bar with meeting info (fixed height)
                    egui::Frame::none()
                        .fill(theme.black)
                        .inner_margin(10.0)
                        .show(ui, |ui| {
                            ui.set_max_height(top_bar_height);
                            MeetingAppBar::render(ui, state, theme, user_id);
                        });

                    // Main content area (video grid)
                    egui::Frame::none().fill(theme.black).show(ui, |ui| {
                        ui.set_min_height(content_height);

                        ui.horizontal(|ui| {
                            // Main content area with video grid
                            let main_width = if state.side_panel_open {
                                ui.available_width() * 0.8 // Give more space to the video grid
                            } else {
                                ui.available_width()
                            };

                            ui.set_max_width(main_width);

                            // Video grid takes all available space
                            VideoGrid::render(ui, state, theme, user_id, content_height);

                            // Side panel
                            if state.side_panel_open {
                                ui.separator();
                                let side_panel_width = ui.available_width();
                                ui.set_min_width(side_panel_width);

                                egui::Frame::none().fill(theme.panel).show(ui, |ui| {
                                    ui.vertical(|ui| {
                                        // Tab buttons
                                        ui.horizontal(|ui| {
                                            if ui
                                                .selectable_label(
                                                    state.side_panel_tab
                                                        == SidePanelTab::Participants,
                                                    "Participants",
                                                )
                                                .clicked()
                                            {
                                                state.side_panel_tab = SidePanelTab::Participants;
                                            }

                                            if ui
                                                .selectable_label(
                                                    state.side_panel_tab == SidePanelTab::Chat,
                                                    "Chat",
                                                )
                                                .clicked()
                                            {
                                                state.side_panel_tab = SidePanelTab::Chat;
                                            }
                                        });

                                        ui.separator();

                                        // Tab content
                                        match state.side_panel_tab {
                                            SidePanelTab::Participants => {
                                                ParticipantsTab::render(ui, state, theme, user_id);
                                            }
                                            SidePanelTab::Chat => {
                                                MeetingChat::render(ui, state, theme, user_id);
                                            }
                                        }
                                    });
                                });
                            }
                        });
                    });

                    // Bottom controls (fixed height) - Zoom-like control bar
                    egui::Frame::none()
                        .fill(theme.black)
                        .inner_margin(5.0)
                        .show(ui, |ui| {
                            ui.set_min_height(bottom_bar_height);
                            MeetingFooter::render(ui, state, theme, user_id);
                        });
                });
            });
        });
}
