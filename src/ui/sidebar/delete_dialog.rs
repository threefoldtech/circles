use crate::app::{ActiveFeature, CircleApp};
use crate::utils::config::Theme;
use egui::{Context, Window};

use super::helpers::add_log_to_circles_bot;

// Delete confirmation dialog
pub fn render_delete_confirmation_dialog(ctx: &Context, app: &mut CircleApp, theme: &Theme) {
    // Only proceed if the dialog should be shown
    if !app.delete_confirmation_state.open {
        return;
    }

    let circle_id = app.delete_confirmation_state.circle_id;
    let circle_name = app.delete_confirmation_state.circle_name.clone();

    let body_text = format!(
        "Are you sure you want to delete \"{}\"?\nThis action cannot be undone.",
        circle_name
    );

    // Create a simple confirmation dialog
    let mut result = None;

    Window::new("Delete Circle?")
        .fixed_size([400.0, 200.0])
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
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
                .inner_margin(egui::Margin::same(24)),
        )
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.heading(egui::RichText::new("Delete Circle?").color(theme.error));
                ui.add_space(10.0);
                ui.label(body_text);
                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        result = Some(false);
                    }

                    if ui.button("Delete").clicked() {
                        result = Some(true);
                    }
                });
            });
        });

    // Handle the result
    if let Some(confirmed) = result {
        if confirmed {
            // User confirmed deletion
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
                add_log_to_circles_bot(app, format!("Circle \"{}\" deleted", circle_name));
            }
        }
        // Reset the state in either case
        app.delete_confirmation_state = crate::models::features::DeleteConfirmationState::default();
    }
}
