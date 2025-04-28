use crate::app::{ActiveFeature, CircleApp};
use crate::models::dummy_data::{Conversation, Message};
use crate::models::features::{CircleActionState, DeleteConfirmationState};
use crate::utils::config::{LayoutConfig, Theme};
use eframe::egui::{Context, Frame, SidePanel, Window};
use egui::{
    Align, Button, CursorIcon, Layout, Margin, RichText, ScrollArea, Sense, Stroke, Ui, Vec2,
};
use uuid::Uuid;

// Constants for spacing and sizes (unchanged)
const HEADER_SPACE: f32 = 12.0;
const SEARCH_SPACE: f32 = 16.0;
const SECTION_SPACE: f32 = 8.0;
const BOTTOM_SPACE: f32 = 8.0;
const SECTION_GAP: f32 = 6.0;
const ITEM_SPACE: f32 = 4.0;

// Sidebar rendering
pub fn render_sidebar(
    app: &mut CircleApp,
    ctx: &Context,
    app_layout: &Frame,
    theme: &Theme,
    config: &LayoutConfig,
) {
    SidePanel::left("sidebar")
        .resizable(false)
        .exact_width(config.sidebar_width)
        .frame(app_layout.clone().fill(theme.secondary_background))
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                render_circle_header(ui, app, theme);
                ui.add_space(HEADER_SPACE);
                render_search_box(ui, app, theme);
                ui.add_space(SEARCH_SPACE);
                render_circle_sections(ui, app, theme);
                ui.add_space(SECTION_SPACE);
                ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                    ui.add_space(BOTTOM_SPACE);
                    render_settings_button(ui, app, theme);
                    ui.add_space(BOTTOM_SPACE);
                });
            });
        });

    render_delete_confirmation_dialog(ctx, app, theme);
}

// Circle header with title and add button
fn render_circle_header(ui: &mut Ui, app: &mut CircleApp, theme: &Theme) {
    ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
        ui.vertical(|ui| {
            ui.heading(
                RichText::new("Circles")
                    .size(18.0)
                    .strong()
                    .color(theme.text),
            );
        });
        ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
            if ui
                .add(create_add_button(theme))
                .on_hover_text(
                    RichText::new("Create a new circle")
                        .size(12.0)
                        .color(theme.white),
                )
                .on_hover_cursor(CursorIcon::PointingHand)
                .clicked()
            {
                app.open_circle_dialog();
            }
        });
    });
}

// Search box
fn render_search_box(ui: &mut Ui, app: &mut CircleApp, theme: &Theme) {
    let search_frame = Frame::new()
        .fill(theme.hover)
        .corner_radius(20.0)
        .inner_margin(Margin::same(10))
        .stroke(Stroke::NONE);

    ui.horizontal(|ui| {
        ui.add_space(SEARCH_SPACE);
        search_frame.show(ui, |ui| {
            ui.set_max_width(228.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new("🔍").size(16.0).color(theme.text));
                ui.add_space(SECTION_SPACE);
                let original_style = ui.style().clone();
                ui.style_mut().visuals.widgets.inactive.bg_fill = theme.transparent;
                ui.style_mut().visuals.widgets.active.bg_fill = theme.transparent;
                ui.style_mut().visuals.widgets.hovered.bg_fill = theme.transparent;

                ui.add(
                    egui::TextEdit::singleline(&mut app.search_query)
                        .hint_text(RichText::new("Search circles...").color(theme.text))
                        .text_color(theme.text)
                        .frame(false)
                        .margin(Vec2::ZERO)
                        .desired_width(180.0),
                );
                ui.set_style(original_style);
            });
        });
        ui.add_space(SEARCH_SPACE);
    });
}

// Settings button
fn render_settings_button(ui: &mut Ui, app: &mut CircleApp, theme: &Theme) {
    if ui
        .add(create_settings_button(theme))
        .on_hover_text(RichText::new("App Settings").size(12.0).color(theme.white))
        .on_hover_cursor(CursorIcon::PointingHand)
        .clicked()
    {
        app.set_active_feature(ActiveFeature::AppSettings);
    }
}

// Circle sections (ALL, FAVORITE, OTHERS)
fn render_circle_sections(ui: &mut Ui, app: &mut CircleApp, theme: &Theme) {
    let active_circle_id = app.active_circle_id;

    // Categorize circles
    let mut user_circles = Vec::new();
    let mut favorite_circles = Vec::new();
    let mut system_circles = Vec::new();

    for circle in &app.circles {
        let circle_data = (
            circle.id,
            circle.name.clone(),
            circle.circle_type,
            circle.is_system_circle,
        );
        let is_favorite = app
            .circle_actions
            .get(&circle.id)
            .map_or(false, |state| state.is_favorite);

        if is_favorite {
            favorite_circles.push(circle_data.clone());
        }
        if circle.is_system_circle {
            system_circles.push(circle_data);
        } else {
            user_circles.push(circle_data);
        }
    }

    render_circle_section(
        ui,
        "ALL",
        &user_circles,
        active_circle_id,
        app,
        theme,
        "No circles yet. Click the + button to create one.",
        true,
    );

    render_circle_section(
        ui,
        "FAVORITE",
        &favorite_circles,
        active_circle_id,
        app,
        theme,
        "No favorites yet",
        true,
    );

    render_others_section(ui, &system_circles, active_circle_id, app, theme);
}

// Individual circle section
fn render_circle_section(
    ui: &mut Ui,
    title: &str,
    circles: &[(Uuid, String, crate::models::circle::CircleType, bool)],
    active_circle_id: Option<Uuid>,
    app: &mut CircleApp,
    theme: &Theme,
    empty_message: &str,
    default_open: bool,
) {
    egui::CollapsingHeader::new(RichText::new(title).size(14.0).strong().color(theme.accent))
        .default_open(default_open)
        .show(ui, |ui| {
            ui.add_space(SECTION_SPACE);
            if circles.is_empty() {
                ui.label(
                    RichText::new(empty_message)
                        .size(13.0)
                        .color(theme.secondary_text),
                );
            } else {
                ScrollArea::vertical().show(ui, |ui| {
                    for (id, name, circle_type, is_system_circle) in circles {
                        render_circle_item(
                            ui,
                            *id,
                            name,
                            *circle_type,
                            *is_system_circle,
                            active_circle_id,
                            app,
                            theme,
                        );
                    }
                });
            }
            ui.add_space(SECTION_SPACE);
        });
    ui.add_space(SECTION_GAP);
}

// Others section for system circles
fn render_others_section(
    ui: &mut Ui,
    system_circles: &[(Uuid, String, crate::models::circle::CircleType, bool)],
    active_circle_id: Option<Uuid>,
    app: &mut CircleApp,
    theme: &Theme,
) {
    egui::CollapsingHeader::new(
        RichText::new("OTHERS")
            .size(14.0)
            .strong()
            .color(theme.accent),
    )
    .default_open(true)
    .show(ui, |ui| {
        ui.add_space(SECTION_SPACE);
        if system_circles.is_empty() {
            ui.label(
                RichText::new("No system circles available")
                    .size(13.0)
                    .color(theme.secondary_text),
            );
        } else {
            ScrollArea::vertical().show(ui, |ui| {
                for (id, name, circle_type, is_system_circle) in system_circles {
                    render_circle_item(
                        ui,
                        *id,
                        name,
                        *circle_type,
                        *is_system_circle,
                        active_circle_id,
                        app,
                        theme,
                    );
                }
            });
        }
        ui.add_space(SECTION_SPACE);
    });
    ui.add_space(SECTION_GAP);
}

// Individual circle item
fn render_circle_item(
    ui: &mut Ui,
    id: Uuid,
    name: &str,
    circle_type: crate::models::circle::CircleType,
    is_system_circle: bool,
    active_circle_id: Option<Uuid>,
    app: &mut CircleApp,
    theme: &Theme,
) {
    let is_active = active_circle_id.map_or(false, |active_id| active_id == id);
    let (icon, color) = match circle_type {
        crate::models::circle::CircleType::Personal => ("👤", theme.success),
        crate::models::circle::CircleType::Team => ("👥", theme.team),
        crate::models::circle::CircleType::Private => ("🔒", theme.private),
    };

    let circle_frame = Frame::new()
        .fill(if is_active {
            theme.hover
        } else {
            theme.transparent
        })
        .inner_margin(Margin::symmetric(8, 6))
        .corner_radius(4.0);

    circle_frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.set_min_width(ui.available_width());
            let icon_frame = Frame::new()
                .fill(if is_active { color } else { theme.icon_bg })
                .corner_radius(12.0)
                .inner_margin(Margin::same(6));

            let icon_response = icon_frame.show(ui, |ui| {
                ui.label(RichText::new(icon).size(16.0).color(if is_active {
                    theme.white
                } else {
                    color
                }));
            });

            ui.add_space(12.0);
            let text_response = ui.vertical(|ui| {
                ui.label(RichText::new(name).size(14.0).strong().color(if is_active {
                    theme.accent
                } else {
                    theme.text
                }));
                ui.label(
                    RichText::new(format!("{} Circle", circle_type_name(circle_type)))
                        .size(12.0)
                        .color(if is_active {
                            theme.text
                        } else {
                            theme.secondary_text
                        }),
                );
            });

            let clickable_rect = icon_response
                .response
                .rect
                .union(text_response.response.rect);
            let response = ui.interact(clickable_rect, ui.id().with(id), Sense::click());

            if response.hovered() {
                ui.output_mut(|o| o.cursor_icon = CursorIcon::PointingHand);
            }

            if response.clicked() {
                handle_circle_selection(app, id, name);
            }

            response.context_menu(|ui| {
                render_context_menu(ui, app, id, is_system_circle, name, theme);
            });
        });
    });
    ui.add_space(ITEM_SPACE);
}

// Handle circle selection logic
fn handle_circle_selection(app: &mut CircleApp, id: Uuid, name: &str) {
    app.set_active_circle(id);
    match name {
        "CirclesBot" => app.set_active_feature(ActiveFeature::BotChannel),
        "WelcomeBot" => app.set_active_feature(ActiveFeature::Welcome),
        _ => {
            if matches!(
                app.active_feature,
                ActiveFeature::Welcome | ActiveFeature::BotChannel
            ) {
                app.set_active_feature(ActiveFeature::Mail);
            }
        }
    }
}

// Helper function to add a message to CirclesBot
fn add_log_to_circles_bot(app: &mut CircleApp, message: String) {
    let circles_bot = app.circles.iter().find(|c| c.name == "CirclesBot");
    if let Some(bot) = circles_bot {
        if let Some(feature_data) = app.circle_feature_data.get_mut(&bot.id) {
            let conversations = &mut feature_data.chat_data.conversations;
            if conversations.is_empty() {
                conversations.push(Conversation {
                    id: Uuid::new_v4(),
                    name: "Circles Bot".to_string(),
                    participants: vec!["Circles Bot".to_string(), "Me".to_string()],
                    messages: Vec::new(),
                });
            }
            conversations[0].messages.push(Message {
                id: Uuid::new_v4(),
                sender: "CirclesBot".to_string(),
                content: message,
                timestamp: chrono::Utc::now(),
                read: false,
            });
        }
    }
}

// Context menu for circle items
fn render_context_menu(
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
                    app.delete_confirmation_state = DeleteConfirmationState {
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

// Delete confirmation dialog
fn render_delete_confirmation_dialog(ctx: &Context, app: &mut CircleApp, theme: &Theme) {
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
                    .inner_margin(egui::Margin::same(24)), // Appropriate padding for content
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
                            app.delete_confirmation_state = DeleteConfirmationState::default();
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
                            app.delete_confirmation_state = DeleteConfirmationState::default();
                        }
                    });
                });
            });
    }
}

// Helper functions for creating UI elements
fn create_add_button(theme: &Theme) -> Button {
    Button::new(RichText::new("➕").size(16.0).color(theme.white))
        .min_size(Vec2::new(32.0, 32.0))
        .corner_radius(8.0)
        .fill(theme.accent)
        .stroke(Stroke::NONE)
}

fn create_settings_button(theme: &Theme) -> Button {
    Button::new(RichText::new("⚙️ Settings").size(14.0).color(theme.text))
        .min_size(Vec2::new(200.0, 36.0))
        .corner_radius(8.0)
        .fill(theme.secondary_background)
        .stroke(Stroke::new(1.0, theme.border))
}

// Helper function for circle type name
fn circle_type_name(circle_type: crate::models::circle::CircleType) -> &'static str {
    match circle_type {
        crate::models::circle::CircleType::Personal => "Personal",
        crate::models::circle::CircleType::Team => "Team",
        crate::models::circle::CircleType::Private => "Private",
    }
}
