mod add_member_dialog;
mod circle_item;
mod context_menu;
mod delete_dialog;
mod header;
mod helpers;
mod search;
mod sections;

use crate::app::{ActiveFeature, CircleApp};
use crate::utils::config::{LayoutConfig, Theme};
use eframe::egui::{Context, Frame, SidePanel};
use egui::{Align, Layout, Ui};

// Constants for spacing and sizes
pub const HEADER_SPACE: f32 = 12.0;
pub const SEARCH_SPACE: f32 = 16.0;
pub const SECTION_SPACE: f32 = 8.0;
pub const BOTTOM_SPACE: f32 = 8.0;
pub const SECTION_GAP: f32 = 6.0;
pub const ITEM_SPACE: f32 = 4.0;

// Re-export all components
// These re-exports are for other parts of the codebase that might need these functions
pub use add_member_dialog::render_add_member_dialog;
#[allow(unused_imports)]
pub use circle_item::{handle_circle_selection, render_circle_item};
#[allow(unused_imports)]
pub use context_menu::{render_circle_context_menu, render_user_status_menu};
pub use delete_dialog::render_delete_confirmation_dialog;
pub use header::render_circle_header;
#[allow(unused_imports)]
pub use helpers::{
    add_log_to_circles_bot, circle_type_name, create_add_button, create_settings_button,
};
pub use search::render_search_box;
#[allow(unused_imports)]
pub use sections::{render_circle_section, render_circle_sections, render_others_section};

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
    render_add_member_dialog(ctx, app, theme);
}

// Settings button
fn render_settings_button(ui: &mut Ui, app: &mut CircleApp, theme: &Theme) {
    if ui
        .add(create_settings_button(theme))
        .on_hover_text(
            egui::RichText::new("App Settings")
                .size(12.0)
                .color(theme.white),
        )
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .clicked()
    {
        app.set_active_feature(ActiveFeature::AppSettings);
    }
}
