use crate::app::CircleApp;
use crate::utils::config::Theme;
use egui::{RichText, ScrollArea, Ui};
use uuid::Uuid;

use super::circle_item::render_circle_item;
use super::{SECTION_GAP, SECTION_SPACE};

// Circle sections (ALL, FAVORITE, OTHERS)
pub fn render_circle_sections(ui: &mut Ui, app: &mut CircleApp, theme: &Theme) {
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
            // Add to favorites regardless of circle type
            favorite_circles.push(circle_data.clone());
        }

        if circle.is_system_circle && !is_favorite {
            // Only add to system circles if it's not already in favorites
            system_circles.push(circle_data);
        } else if !circle.is_system_circle && !is_favorite {
            // Only add to user circles if it's not a system circle and not a favorite
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
pub fn render_circle_section(
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
pub fn render_others_section(
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
