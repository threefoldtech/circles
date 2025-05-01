use crate::app::CircleApp;
use crate::models::features::CircleActionState;
use crate::ui::components::context_menu::{ContextMenuConfig, MenuItem};
use crate::utils::config::Theme;
use egui::Ui;
use uuid::Uuid;

use super::helpers::add_log_to_circles_bot;

// Context menu for circle items
pub fn render_circle_context_menu(
    ui: &mut Ui,
    app: &mut CircleApp,
    circle_id: Uuid,
    is_system_circle: bool,
    circle_name: &str,
    theme: &Theme,
) {
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

    // Create menu items
    let mut menu_items = Vec::new();

    // Favorite item
    menu_items.push(
        MenuItem::new(if is_favorite {
            "Remove from favorites"
        } else {
            "Add to favorites"
        })
        .with_icon("⭐"),
    );

    if !is_system_circle {
        // Add members (only for non-system circles)
        menu_items.push(MenuItem::new("Add members").with_icon("👥"));

        // Rename (only for non-system circles)
        menu_items.push(MenuItem::new("Rename").with_icon("✏️"));
    }

    // Mute notifications
    menu_items.push(
        MenuItem::new(if is_muted {
            "Unmute notifications"
        } else {
            "Mute notifications"
        })
        .with_icon(if is_muted { "🔔" } else { "🔕" }),
    );

    if !is_system_circle {
        // Delete circle (only for non-system circles)
        menu_items.push(MenuItem::new("Delete circle").with_icon("🗑️").destructive());
    }

    // Create menu configuration
    let config = ContextMenuConfig::new(theme);

    // Render the menu and get clicked indices
    let clicked_indices =
        crate::ui::components::context_menu::render_context_menu(ui, &menu_items, &config);

    // Handle clicked items
    if !clicked_indices.is_empty() {
        for &index in &clicked_indices {
            // Re-borrow app.circle_actions mutably for state changes
            let circle_action_state = app
                .circle_actions
                .entry(circle_id)
                .or_insert_with(CircleActionState::default);

            // Map the index to the appropriate action
            if is_system_circle {
                match index {
                    0 => {
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
                    1 => {
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
                    _ => {}
                }
            } else {
                match index {
                    0 => {
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
                    1 => {
                        // Add members
                        add_log_to_circles_bot(
                            app,
                            format!("Add members dialog for {} would open here", circle_name),
                        );
                    }
                    2 => {
                        // Rename
                        add_log_to_circles_bot(
                            app,
                            format!("Rename dialog for {} would open here", circle_name),
                        );
                    }
                    3 => {
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
                    4 => {
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
            }
        }
    }
}

// Implement a similar context menu for the user status in the footer
pub fn render_user_status_menu(ui: &mut Ui, _app: &mut CircleApp, theme: &Theme) -> Vec<usize> {
    // Create menu items for the user menu
    let menu_items = vec![
        MenuItem::new("Profile").with_icon("👤"),
        MenuItem::new("Edit Profile").with_icon("✏️"),
        MenuItem::new("Snooze").with_icon("💤"),
        MenuItem::new("Logout").with_icon("🚪").destructive(),
    ];

    // Create menu configuration
    let config = ContextMenuConfig::new(theme)
        .with_min_width(200.0)
        .with_corner_radius(8.0);

    // Render the menu and get clicked indices
    let clicked_indices =
        crate::ui::components::context_menu::render_context_menu(ui, &menu_items, &config);

    // Return the clicked indices for handling in the footer
    clicked_indices
}
