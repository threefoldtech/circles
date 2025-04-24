use crate::app::{ActiveFeature, CircleApp};
use crate::ui::footer::{
    StatusFrameProps, create_styled_text, render_status_dot, set_hover_cursor,
};
use crate::utils::config::{LayoutConfig, NAV_ITEMS, Theme};
use eframe::egui::{
    Align, Button, Color32, Context, CursorIcon, Frame, Layout, Margin, RichText, Sense, Stroke,
    TopBottomPanel, Ui, Vec2,
};
use egui::Direction;

// Header rendering functions
pub fn render_top_panel(
    app: &mut CircleApp,
    ctx: &Context,
    app_layout: &Frame,
    theme: &Theme,
    config: &LayoutConfig,
) {
    TopBottomPanel::top("top_panel")
        .exact_height(config.navbar_height)
        .frame(app_layout.clone().corner_radius(0))
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.add_space(16.0);
                ui.horizontal(|ui| {
                    // Left side - App logo
                    ui.add_space(16.0);

                    // Logo circle
                    let logo_size = 28.0;
                    let (_, logo_rect) = ui.allocate_space(Vec2::new(logo_size, logo_size));
                    ui.painter()
                        .circle_filled(logo_rect.center(), logo_size / 2.0, theme.accent);
                    ui.painter().text(
                        logo_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        "C",
                        egui::FontId::proportional(18.0),
                        Color32::WHITE,
                    );

                    // ui.add_space(150.0);
                    // App logo and name
                    ui.horizontal(|ui| {
                        ui.heading(
                            RichText::new("Circle Collaboration System")
                                .size(20.0)
                                .strong()
                                .color(theme.text),
                        );
                    });

                    ui.add_space(150.0); // Set any space to center the buttons

                    // Render the navigation bar with feature buttons
                    if !app.is_first_time {
                        // Center - Navigation buttons
                        ui.with_layout(
                            Layout::centered_and_justified(Direction::LeftToRight),
                            |ui| {
                                ui.add_space(16.0);
                                ui.horizontal(|ui| {
                                    for (icon, text, feature) in NAV_ITEMS {
                                        if create_nav_button(
                                            ui,
                                            icon,
                                            text,
                                            *feature,
                                            app.get_active_feature(),
                                            theme,
                                        ) {
                                            app.set_active_feature(*feature);
                                        }
                                    }
                                });
                            },
                        );
                    }

                    // Right side - Active circle
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.add_space(16.0);
                        let circle_text = app
                            .active_circle()
                            .map_or("No circle selected".to_string(), |c| {
                                format!("Active: {}", c.name)
                            });

                        // Use the StatusFrameProps pattern from footer.rs
                        let status_frame = StatusFrameProps::new(theme)
                            .with_margin(Margin::symmetric(10, 4))
                            .build()
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(create_styled_text(circle_text, theme, 13.0, true));

                                    if app.active_circle().is_some() {
                                        ui.add_space(4.0);
                                        render_status_dot(ui, theme, 6.0);
                                    }
                                });
                            });

                        set_hover_cursor(ui, &status_frame.response);
                    });
                });
                ui.add_space(8.0);
                ui.painter().hline(
                    ui.available_rect_before_wrap().x_range(),
                    ui.cursor().top(),
                    Stroke::new(1.0, theme.border),
                );
            });
        });
}

fn create_nav_button(
    ui: &mut Ui,
    icon: &str,
    text: &str,
    feature: ActiveFeature,
    active_feature: ActiveFeature,
    theme: &Theme,
) -> bool {
    // let config = get_layout_config();
    let is_active = active_feature == feature;

    // Create a button with icon and text
    let button_text = RichText::new(format!("{} {}", icon, text))
        .size(14.0)
        .color(if is_active {
            Color32::WHITE
        } else {
            theme.text
        });

    let button = Button::new(button_text)
        .min_size(Vec2::new(110.0, 40.0)) // Slightly wider to accommodate icons
        .corner_radius(6)
        .sense(Sense::click_and_drag())
        .fill(if is_active {
            theme.accent
        } else {
            theme.secondary_background
        })
        .stroke(Stroke::NONE);

    let response = ui
        .add(button)
        .on_hover_ui(|ui| {
            ui.style_mut().visuals.widgets.hovered.bg_fill = theme.accent;
            ui.label(
                RichText::new(format!("{} {}", icon, text))
                    .size(12.0)
                    .color(Color32::WHITE),
            );
        })
        .on_hover_cursor(CursorIcon::PointingHand);

    if response.hovered() {
        ui.ctx().request_repaint();
    }
    response.clicked()
}
