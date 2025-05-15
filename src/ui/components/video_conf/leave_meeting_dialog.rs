use eframe::egui::{self, Align2, Button, RichText, Vec2};
use uuid::Uuid;

use crate::models::features::video_conf::VideoConferenceState;
use crate::utils::config::Theme;

/// Leave meeting dialog component
pub struct LeaveMeetingDialog;

impl LeaveMeetingDialog {
    /// Render the leave meeting confirmation dialog
    pub fn render(ui: &mut egui::Ui, state: &mut VideoConferenceState, theme: &Theme, user_id: Uuid) {
        let ctx = ui.ctx();

        egui::Window::new("Leave Meeting")
            .fixed_size([300.0, 150.0])
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
                    ui.heading(RichText::new("Leave Meeting").color(theme.header_text));
                    ui.add_space(10.0);

                    ui.label(
                        RichText::new("Are you sure you want to leave the meeting?").color(theme.text),
                    );

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
                            state.leave_confirmation_open = false;
                        }

                        ui.add_space(10.0);

                        if ui
                            .add(
                                Button::new(RichText::new("Leave").color(theme.light_color))
                                    .fill(theme.error)
                                    .min_size(Vec2::new(80.0, 30.0)),
                            )
                            .clicked()
                        {
                            // Remove the user from the meeting
                            if let Some(meeting) = &mut state.current_meeting {
                                meeting.remove_participant(user_id);
                            }

                            // Reset meeting state
                            state.in_meeting = false;
                            state.current_meeting = None;
                            state.leave_confirmation_open = false;
                        }
                    });
                });
            });
    }
}
