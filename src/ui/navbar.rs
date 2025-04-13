use crate::app::{ActiveFeature, CircleApp};
use crate::utils::config::Theme;
use eframe::egui::{
    self, Align, Button, Color32, Context, CursorIcon, Frame, Layout, RichText, Rounding, Sense,
    Stroke, TopBottomPanel, Ui, Vec2,
};

#[allow(dead_code)]
struct LayoutConfig {
    spacing: f32,
    panel_height: f32,
    button_size: Vec2,
    sidebar_width: f32,
}

impl LayoutConfig {
    fn new() -> Self {
        Self {
            spacing: 12.0,
            panel_height: 50.0,
            button_size: Vec2::new(90.0, 40.0),
            sidebar_width: 260.0,
        }
    }
}

// Header rendering functions
pub fn render_top_panel(app: &CircleApp, ctx: &Context, app_layout: &Frame, theme: &Theme) {
    TopBottomPanel::top("top_panel")
        .exact_height(60.0)
        .frame(
            app_layout
                .clone()
                .shadow(egui::epaint::Shadow {
                    extrusion: 6.0,
                    color: Color32::from_black_alpha(25),
                })
                .rounding(Rounding::same(0.0)),
        )
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.add_space(16.0);
                    ui.heading(
                        RichText::new("Circle Collaboration System")
                            .size(20.0)
                            .strong()
                            .color(theme.text),
                    );
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.add_space(16.0);
                        let circle_text = app
                            .active_circle()
                            .map_or("No circle selected".to_string(), |c| {
                                format!("Active: {}", c.name)
                            });
                        let circle_label = ui
                            .label(
                                RichText::new(circle_text)
                                    .size(14.0)
                                    .color(Color32::from_rgb(70, 80, 90)),
                            )
                            .on_hover_ui(|ui| {
                                ui.label(
                                    RichText::new("Current active circle")
                                        .size(12.0)
                                        .color(Color32::from_rgb(100, 110, 120)),
                                );
                            });
                        if app.active_circle().is_some() {
                            ui.add_space(6.0);
                            ui.painter().circle_filled(
                                circle_label.rect.left_center() + Vec2::new(-12.0, 0.0),
                                4.0,
                                theme.success,
                            );
                        }
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

pub fn render_navigation_bar(
    app: &mut CircleApp,
    ctx: &Context,
    app_layout: &Frame,
    theme: &Theme,
) {
    const NAV_ITEMS: &[(&str, ActiveFeature)] = &[
        ("AI Tools", ActiveFeature::AITools),
        ("Calendar", ActiveFeature::Calendar),
        ("Chat", ActiveFeature::Chat),
        ("Documents", ActiveFeature::Documents),
        ("Mail", ActiveFeature::Mail),
        ("Settings", ActiveFeature::Settings),
        ("Video", ActiveFeature::VideoConference),
    ];

    TopBottomPanel::top("navigation_bar")
        .exact_height(get_layout_config().panel_height)
        .frame(app_layout.clone().shadow(egui::epaint::Shadow {
            extrusion: 4.0,
            color: theme.shadow,
        }))
        .show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.style_mut().spacing.item_spacing = Vec2::new(8.0, 0.0);
                ui.style_mut().visuals.widgets.hovered.expansion = 1.0;
                for (text, feature) in NAV_ITEMS {
                    if create_nav_button(ui, text, *feature, app.active_feature, theme) {
                        app.set_active_feature(*feature);
                    }
                }
            });
        });
}

fn create_nav_button(
    ui: &mut Ui,
    text: &str,
    feature: ActiveFeature,
    active_feature: ActiveFeature,
    theme: &Theme,
) -> bool {
    let config = get_layout_config();
    let is_active = active_feature == feature;
    let button = Button::new(RichText::new(text).size(14.0).color(if is_active {
        Color32::WHITE
    } else {
        Color32::from_rgb(70, 80, 90)
    }))
    .min_size(config.button_size)
    .rounding(Rounding::same(6.0))
    .sense(Sense::click_and_drag())
    .fill(if is_active {
        theme.accent
    } else {
        Color32::from_rgb(230, 235, 240)
    })
    .stroke(if is_active {
        Stroke::new(1.0, Color32::from_rgb(45, 100, 200))
    } else {
        Stroke::NONE
    });

    let response = ui
        .add(button)
        .on_hover_ui(|ui| {
            ui.style_mut().visuals.widgets.hovered.bg_fill = theme.accent;
            ui.label(RichText::new(text).size(12.0).color(Color32::WHITE));
        })
        .on_hover_cursor(CursorIcon::PointingHand);

    if response.hovered() {
        ui.ctx().request_repaint();
    }
    response.clicked()
}

// Helper function to get layout config
fn get_layout_config() -> LayoutConfig {
    LayoutConfig::new()
}
