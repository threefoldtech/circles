use crate::app::{ActiveFeature, CircleApp};
use crate::utils::config::{LayoutConfig, Theme};
use eframe::egui::{Context, Frame, SidePanel};
use egui::{
    Align, Button, CursorIcon, Layout, Margin, RichText, ScrollArea, Sense, Stroke, Ui, Vec2,
};
use uuid::Uuid;

// Constants for spacing and sizes to maintain exact styling
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
    let circles: Vec<_> = app
        .circles
        .iter()
        .map(|c| (c.id, c.name.clone(), c.circle_type))
        .collect();

    let (default_circles, user_circles): (Vec<_>, Vec<_>) = circles
        .into_iter()
        .partition(|(_, name, _)| name == "WelcomeBot" || name == "CirclesBot");

    let active_circle_id = app.active_circle_id;

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
        &[],
        active_circle_id,
        app,
        theme,
        "No favorites yet",
        true,
    );

    render_others_section(ui, &default_circles, active_circle_id, app, theme);
}

// Individual circle section
fn render_circle_section(
    ui: &mut Ui,
    title: &str,
    circles: &[(Uuid, String, crate::models::circle::CircleType)],
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
                    for (id, name, circle_type) in circles {
                        render_circle_item(
                            ui,
                            *id,
                            name,
                            *circle_type,
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

// Others section for default circles
fn render_others_section(
    ui: &mut Ui,
    default_circles: &[(Uuid, String, crate::models::circle::CircleType)],
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
        if default_circles.is_empty() {
            ui.label(
                RichText::new("No system circles available")
                    .size(13.0)
                    .color(theme.secondary_text),
            );
        } else {
            ScrollArea::vertical().show(ui, |ui| {
                for (id, name, circle_type) in default_circles {
                    render_circle_item(ui, *id, name, *circle_type, active_circle_id, app, theme);
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
                render_context_menu(ui, theme);
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

// Context menu for circle items
fn render_context_menu(ui: &mut Ui, theme: &Theme) {
    ui.style_mut().visuals.widgets.hovered.weak_bg_fill = theme.hover;
    ui.style_mut().visuals.widgets.active.weak_bg_fill = theme.hover;
    ui.style_mut().spacing.indent = 16.0;
    ui.style_mut().spacing.item_spacing = Vec2::new(4.0, 4.0);
    ui.style_mut().spacing.button_padding = Vec2::new(10.0, 10.0);
    ui.set_min_width(200.0);

    let menu_items = [
        ("Add to favorites", false),
        ("Add members", false),
        ("Rename", false),
        ("Mute notifications", false),
        ("Delete circle", true),
    ];

    for (i, (label, is_destructive)) in menu_items.iter().enumerate() {
        let button = Button::new(RichText::new(*label).size(14.0).color(if *is_destructive {
            theme.error
        } else {
            theme.text
        }))
        .min_size(Vec2::new(180.0, 32.0));

        let button_response = ui.add(button).on_hover_cursor(CursorIcon::PointingHand);

        if button_response.clicked() {
            // TODO: Implement respective functionality
            ui.memory_mut(|mem| mem.close_popup());
        }

        if i < menu_items.len() - 1 {
            ui.separator();
        }
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
