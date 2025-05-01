use crate::app::{ActiveFeature, CircleApp};
use crate::utils::config::Theme;
use egui::{CursorIcon, Frame, Margin, RichText, Sense, Ui};
use uuid::Uuid;

use super::ITEM_SPACE;
use super::context_menu::render_circle_context_menu;
use super::helpers::circle_type_name;

// Individual circle item
pub fn render_circle_item(
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
                render_circle_context_menu(ui, app, id, is_system_circle, name, theme);
            });
        });
    });
    ui.add_space(ITEM_SPACE);
}

// Handle circle selection logic
pub fn handle_circle_selection(app: &mut CircleApp, id: Uuid, name: &str) {
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
