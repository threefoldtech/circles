use crate::app::CircleApp;
use crate::models::features::CircleActionState;
use crate::utils::config::Theme;
use egui::{Button, CursorIcon, RichText, Ui, Vec2};
use uuid::Uuid;

use super::helpers::add_log_to_circles_bot;

// Context menu for circle items
pub fn render_context_menu(
    ui: &mut Ui,
    app: &mut CircleApp,
    circle_id: Uuid,
    is_system_circle: bool,
    circle_name: &str,
    theme: &Theme,
) {
    ui.style_mut().visuals.widgets.hovered.weak_bg_fill = theme.hover;
    ui.style_mut().visuals.widgets.active.weak_bg_fill = theme.hover;
    ui.style_mut().spacing.indent = 16.0;
    ui.style_mut().spacing.item_spacing = Vec2::new(4.0, 4.0);
    ui.style_mut().spacing.button_padding = Vec2::new(10.0, 10.0);
    ui.set_min_width(200.0);

    // Get favorite and mute status in a scoped block to release the borrow
    let (is_favorite, is_muted) = {
        let circle_action_state = app
            .circle_actions
            .entry(circle_id)
            .or_insert_with(CircleActionState::default);
        (
            circle_action_state.is_favorite,
            circle_action_state.is_muted,
        )
    };

    let menu_items = if is_system_circle {
        vec![
            (
                if is_favorite {
                    "Remove from favorites"
                } else {
                    "Add to favorites"
                },
                false,
            ),
            (
                if is_muted {
                    "Unmute notifications"
                } else {
                    "Mute notifications"
                },
                false,
            ),
        ]
    } else {
        vec![
            (
                if is_favorite {
                    "Remove from favorites"
                } else {
                    "Add to favorites"
                },
                false,
            ),
            ("Add members", false),
            ("Rename", false),
            (
                if is_muted {
                    "Unmute notifications"
                } else {
                    "Mute notifications"
                },
                false,
            ),
            ("Delete circle", true),
        ]
    };

    for (i, (label, is_destructive)) in menu_items.iter().enumerate() {
        let button = Button::new(RichText::new(*label).size(14.0).color(if *is_destructive {
            theme.error
        } else {
            theme.text
        }))
        .min_size(Vec2::new(180.0, 32.0));

        let button_response = ui.add(button).on_hover_cursor(CursorIcon::PointingHand);

        if button_response.clicked() {
            // Re-borrow app.circle_actions mutably for state changes
            let circle_action_state = app
                .circle_actions
                .entry(circle_id)
                .or_insert_with(CircleActionState::default);

            match (is_system_circle, i) {
                (true, 0) | (false, 0) => {
                    // Toggle favorite status
                    let was_favorite = circle_action_state.is_favorite;
                    circle_action_state.is_favorite = !was_favorite;
                    add_log_to_circles_bot(
                        app,
                        format!(
                            "{} {} to favorites",
                            if !was_favorite { "Added" } else { "Removed" },
                            circle_name
                        ),
                    );
                }
                (true, 1) | (false, 3) => {
                    // Toggle mute status
                    let was_muted = circle_action_state.is_muted;
                    circle_action_state.is_muted = !was_muted;
                    add_log_to_circles_bot(
                        app,
                        format!(
                            "Notifications {} for {}",
                            if !was_muted { "muted" } else { "unmuted" },
                            circle_name
                        ),
                    );
                }
                (false, 1) => {
                    // Add members
                    add_log_to_circles_bot(
                        app,
                        format!("Add members dialog for {} would open here", circle_name),
                    );
                }
                (false, 2) => {
                    // Rename
                    add_log_to_circles_bot(
                        app,
                        format!("Rename dialog for {} would open here", circle_name),
                    );
                }
                (false, 4) => {
                    // Delete circle
                    app.delete_confirmation_state =
                        crate::models::features::DeleteConfirmationState {
                            open: true,
                            circle_id: Some(circle_id),
                            circle_name: circle_name.to_string(),
                        };
                }
                _ => {}
            }
            ui.memory_mut(|mem| mem.close_popup());
        }

        if i < menu_items.len() - 1 {
            ui.separator();
        }
    }
}
