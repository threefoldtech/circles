use crate::app::{ActiveFeature, CircleApp};
use crate::utils::config::Theme;
use egui::{Button, Context, Margin, RichText, Window};

use super::helpers::add_log_to_circles_bot;

// Delete confirmation dialog
pub fn render_delete_confirmation_dialog(ctx: &Context, app: &mut CircleApp, theme: &Theme) {
    if app.delete_confirmation_state.open {
        let circle_id = app.delete_confirmation_state.circle_id;
        let circle_name = app.delete_confirmation_state.circle_name.clone();

        Window::new("Confirm Delete")
            .fixed_size([600.0, 620.0]) // Fixed size to match event dialog
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0]) // Centered horizontally
            .collapsible(false)
            .resizable(false)
            .frame(
                egui::Frame::window(&ctx.style())
                    .fill(theme.background)
                    .corner_radius(16)
                    .shadow(egui::epaint::Shadow {
                        color: theme.shadow,
                        offset: [0, 4],
                        blur: 8,
                        spread: 0,
                    })
                    .inner_margin(Margin::same(24)), // Appropriate padding for content
            )
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);
                    ui.heading(RichText::new("Delete Circle?").color(theme.error));
                    ui.add_space(10.0);
                    ui.label(format!(
                        "Are you sure you want to delete \"{}\"?",
                        circle_name
                    ));
                    ui.label("This action cannot be undone.");
                    ui.add_space(20.0);

                    ui.horizontal(|ui| {
                        let cancel_button = Button::new(RichText::new("Cancel").color(theme.text))
                            .fill(theme.accent);
                        if ui.add(cancel_button).clicked() {
                            app.delete_confirmation_state =
                                crate::models::features::DeleteConfirmationState::default();
                        }

                        let delete_button = Button::new(RichText::new("Delete").color(theme.white))
                            .fill(theme.error);

                        if ui.add(delete_button).clicked() {
                            if let Some(id) = circle_id {
                                app.circles.retain(|c| c.id != id);
                                app.circle_actions.remove(&id);
                                if app.active_circle_id == Some(id) {
                                    app.active_circle_id = None;
                                    if !app.circles.is_empty() {
                                        app.set_active_circle(app.circles[0].id);
                                        app.set_active_feature(ActiveFeature::Mail);
                                    }
                                }
                                add_log_to_circles_bot(
                                    app,
                                    format!("Circle \"{}\" deleted", circle_name),
                                );
                            }
                            app.delete_confirmation_state =
                                crate::models::features::DeleteConfirmationState::default();
                        }
                    });
                });
            });
    }
}
