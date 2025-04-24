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
        .frame(
            app_layout
                .clone()
                .corner_radius(0)
                .fill(theme.secondary_background),
        )
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                // ui.add_space(16.0);

                // Entire header row
                ui.horizontal_top(|ui| {
                    // === Left Side: Logo ===
                    ui.horizontal(|ui| {
                        // ui.add_space(16.0);

                        let logo_size = 28.0;
                        let (_, logo_rect) = ui.allocate_space(Vec2::new(logo_size, logo_size));
                        ui.painter().circle_filled(
                            logo_rect.center(),
                            logo_size / 2.0,
                            theme.accent,
                        );
                        ui.painter().text(
                            logo_rect.center(),
                            egui::Align2::CENTER_CENTER,
                            "C",
                            egui::FontId::proportional(18.0),
                            Color32::WHITE,
                        );

                        ui.heading(
                            RichText::new("Circle Collaboration System")
                                .size(20.0)
                                .strong()
                                .color(theme.text),
                        );
                    });

                    // Render the navigation bar with feature buttons
                    if !app.is_first_time {
                        // === Centered Buttons (if circle is active) ===
                        ui.add_space(280.0); // Set any space to center the buttons
                        ui.with_layout(
                            Layout::centered_and_justified(Direction::LeftToRight),
                            |ui| {
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

                    // === Right Side: Circle Status ===
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        let circle_text = app
                            .active_circle()
                            .map_or("No circle selected".to_string(), |c| {
                                format!("Active: {}", c.name)
                            });

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
            theme.light_color
        } else {
            theme.text
        });

    let button = Button::new(button_text)
        .min_size(Vec2::new(100.0, 35.0)) // Slightly wider to accommodate icons
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
                    .color(theme.light_color),
            );
        })
        .on_hover_cursor(CursorIcon::PointingHand);

    if response.hovered() {
        ui.ctx().request_repaint();
    }
    response.clicked()
}
