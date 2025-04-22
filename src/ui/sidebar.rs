use crate::app::{ActiveFeature, CircleApp};

use crate::utils::config::{LayoutConfig, Theme};
use eframe::egui::{Context, Frame, SidePanel};
use egui::{
    Align, Button, Color32, CursorIcon, Layout, Margin, RichText, ScrollArea, Sense, Stroke, Ui,
    Vec2,
};
use uuid::Uuid;

// Panel rendering functions
// Sidebar with circle selector
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
        .frame(app_layout.clone())
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                // Circle selector
                render_circle_header(ui, app, theme);
                ui.add_space(12.0);
                render_search_box(ui, app, theme);
                ui.add_space(16.0);
                render_circle_list(ui, app, theme);
                ui.add_space(8.0);
            });
        });
}

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
            let add_button = Button::new(RichText::new("➕").size(16.0).color(Color32::WHITE))
                .min_size(Vec2::new(32.0, 32.0))
                .corner_radius(8.0)
                .fill(theme.accent)
                .stroke(Stroke::NONE);

            if ui
                .add(add_button)
                .on_hover_text(
                    RichText::new("Create a new circle")
                        .size(12.0)
                        .color(Color32::WHITE),
                )
                .on_hover_cursor(CursorIcon::PointingHand)
                .clicked()
            {
                app.open_circle_dialog();
            }
        });
    });
}

fn render_circle_list(ui: &mut Ui, app: &mut CircleApp, theme: &Theme) {
    let all_circles: Vec<_> = app
        .circles
        .iter()
        .map(|c| (c.id, c.name.clone(), c.circle_type))
        .collect();

    let (default_circles, user_circles): (Vec<_>, Vec<_>) = all_circles
        .into_iter()
        .partition(|(_, name, _)| name == "WelcomeBot" || name == "CirclesBot");

    let active_circle_id = app.active_circle_id;

    // ALL CIRCLES section
    render_circle_section(
        ui,
        "ALL",
        &user_circles,
        active_circle_id,
        app,
        theme,
        "No circles yet. Click the + button to create one.",
        true, // Expand if there are circles
    );

    // FAVORITES section
    render_circle_section(
        ui,
        "FAVORITE",
        &[],
        active_circle_id,
        app,
        theme,
        "No favorites yet",
        true, // Not expanded by default
    );

    // OTHERS section
    egui::CollapsingHeader::new(
        RichText::new("OTHERS")
            .size(14.0)
            .strong()
            .color(theme.accent),
    )
    .default_open(true)
    .show(ui, |ui| {
        ui.add_space(8.0);
        if default_circles.is_empty() {
            ui.label(
                RichText::new("No system circles available")
                    .size(13.0)
                    .color(Color32::from_rgb(100, 110, 120)),
            );
        } else {
            ScrollArea::vertical().show(ui, |ui| {
                for (id, name, circle_type) in &default_circles {
                    render_circle_item(ui, *id, name, *circle_type, active_circle_id, app, theme);
                }
            });
        }
        ui.add_space(8.0);
    });
    ui.add_space(6.0);
}

fn render_search_box(ui: &mut Ui, app: &mut CircleApp, theme: &Theme) {
    let search_frame = Frame::new()
        .fill(theme.hover)
        .corner_radius(20)
        .inner_margin(Margin::same(10))
        .stroke(Stroke::NONE);

    ui.horizontal(|ui| {
        ui.add_space(16.0);
        search_frame.show(ui, |ui| {
            ui.set_max_width(228.0);
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("🔍")
                        .size(16.0)
                        .color(Color32::from_rgb(70, 80, 90)),
                );
                ui.add_space(8.0);
                let original_style = ui.style().clone();
                ui.style_mut().visuals.widgets.inactive.bg_fill = Color32::TRANSPARENT;
                ui.style_mut().visuals.widgets.active.bg_fill = Color32::TRANSPARENT;
                ui.style_mut().visuals.widgets.hovered.bg_fill = Color32::TRANSPARENT;

                ui.add(
                    egui::TextEdit::singleline(&mut app.search_query)
                        .hint_text(
                            RichText::new("Search circles...")
                                .color(Color32::from_rgb(120, 130, 140)),
                        )
                        .text_color(theme.text)
                        .frame(false)
                        .margin(Vec2::new(0.0, 0.0))
                        .desired_width(180.0),
                );
                ui.set_style(original_style);
            });
        });
        ui.add_space(16.0);
    });
}

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
            ui.add_space(8.0);
            if circles.is_empty() {
                ui.label(
                    RichText::new(empty_message)
                        .size(13.0)
                        .color(Color32::from_rgb(100, 110, 120)),
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
            ui.add_space(8.0);
        });
    ui.add_space(6.0);
}

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
            Color32::TRANSPARENT
        })
        .inner_margin(Margin::symmetric(8, 6))
        .corner_radius(4);

    let mut clicked = false;

    circle_frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.set_min_width(ui.available_width());
            let icon_frame = Frame::new()
                .fill(if is_active {
                    color
                } else {
                    Color32::from_rgb(230, 235, 240)
                })
                .corner_radius(12)
                .inner_margin(Margin::same(6));

            // Allocate space for the icon and get its rectangle
            let icon_response = icon_frame.show(ui, |ui| {
                ui.label(RichText::new(icon).size(16.0).color(if is_active {
                    Color32::WHITE
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
                        .color(Color32::from_rgb(100, 110, 120)),
                );
            });

            // Create a rectangle that encompasses the icon and text for interaction
            let icon_rect = icon_response.response.rect;
            let text_rect = text_response.response.rect;
            let clickable_rect = icon_rect.union(text_rect);

            // Interact only with the clickable rectangle
            let response = ui.interact(clickable_rect, ui.id().with(id), Sense::click());
            if response.clicked() {
                clicked = true;
            }
            if response.hovered() {
                ui.output_mut(|o| o.cursor_icon = CursorIcon::PointingHand);
            }
        });
    });

    // Perform circle selection only if the specific clickable area was clicked
    if clicked {
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
    ui.add_space(4.0);
}

fn circle_type_name(circle_type: crate::models::circle::CircleType) -> &'static str {
    match circle_type {
        crate::models::circle::CircleType::Personal => "Personal",
        crate::models::circle::CircleType::Team => "Team",
        crate::models::circle::CircleType::Private => "Private",
    }
}
