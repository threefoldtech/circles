use crate::app::CircleApp;
use crate::ui::footer;
use crate::ui::navbar;
use crate::ui::sidebar;
use crate::utils::config::{LayoutConfig, Theme};
use eframe::egui::{self, Color32, Context, Frame, Margin, Stroke, Vec2};

use super::feature_content;

// Main rendering function
pub fn render(app: &mut CircleApp, ctx: &Context) {
    let theme = Theme::new();
    let config = LayoutConfig::new();

    setup_style(ctx, &theme, &config);
    let app_layout = create_app_layout(&theme);

    // Render the navbar at the top with app name/logo on right and active circle on left
    navbar::render_top_panel(app, ctx, &app_layout, &theme, &config);

    // Render the sidebar with available circles
    sidebar::render_sidebar(app, ctx, &app_layout, &theme, &config);

    // Render the main content area
    feature_content::render_feature_content(app, ctx, &app_layout);

    // Render the footer with connection status, user status, date, and notifications
    footer::render_status_bar(app, ctx, &app_layout, &theme);
}

// Setup egui style
fn setup_style(ctx: &Context, theme: &Theme, config: &LayoutConfig) {
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = Vec2::new(config.spacing, config.spacing);
    style.spacing.window_margin = Margin::same(config.spacing as i8);
    style.spacing.button_padding = Vec2::new(10.0, 6.0);

    style.visuals.override_text_color = Some(theme.text);
    style.visuals.widgets.noninteractive.bg_fill = theme.background;
    style.visuals.widgets.inactive.bg_fill = theme.panel;
    style.visuals.widgets.hovered.bg_fill = theme.hover;
    style.visuals.widgets.active.bg_fill = theme.active;
    style.visuals.selection.bg_fill = theme.accent;

    apply_strokes(&mut style, theme);
    apply_rounding(&mut style);
    ctx.set_style(style);
}

fn apply_strokes(style: &mut egui::Style, _: &Theme) {
    // Remove all strokes/borders
    style.visuals.widgets.noninteractive.bg_stroke = Stroke::NONE;
    style.visuals.widgets.inactive.bg_stroke = Stroke::NONE;
    style.visuals.widgets.hovered.bg_stroke = Stroke::NONE;
    style.visuals.widgets.hovered.fg_stroke = Stroke::NONE;
    style.visuals.widgets.active.bg_stroke = Stroke::NONE;
    style.visuals.widgets.active.fg_stroke = Stroke::NONE;
    style.visuals.selection.stroke = Stroke::NONE;
}

fn apply_rounding(style: &mut egui::Style) {
    // Remove window shadows
    style.visuals.window_shadow = egui::epaint::Shadow::NONE;
    style.visuals.popup_shadow = egui::epaint::Shadow::NONE;
    // style.visuals.window_rounding = Rounding::same(0.0);
    // style.visuals.menu_rounding = Rounding::same(0.0);
}

fn create_app_layout(theme: &Theme) -> Frame {
    Frame::new()
        .fill(theme.background)
        .inner_margin(Margin::same(8))
}

pub fn create_content_frame() -> Frame {
    Frame::new()
        .fill(Color32::WHITE)
        .stroke(Stroke::NONE)
        .corner_radius(0)
        .inner_margin(Margin::same(16))
}
