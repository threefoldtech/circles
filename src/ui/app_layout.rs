use crate::app::{ActiveFeature, CircleApp};
use crate::ui::components::circle_dialog;
use crate::ui::features::{bot_channel, calendar, chat, documents, mail, video_conf, welcome}; // Added welcome
use eframe::egui::{
    self, Align, Button, CentralPanel, Color32, Context, CursorIcon, Frame, Layout, Margin,
    RichText, Rounding, ScrollArea, Sense, SidePanel, Stroke, TopBottomPanel, Ui, Vec2,
};
use uuid::Uuid;

// Constants for styling and layout
struct Theme {
    accent: Color32,
    background: Color32,
    panel: Color32,
    text: Color32,
    border: Color32,
    active: Color32,
    hover: Color32,
    shadow: Color32,
    success: Color32,
    team: Color32,
    private: Color32,
}

impl Theme {
    fn new() -> Self {
        Self {
            accent: Color32::from_rgb(66, 133, 244),
            background: Color32::from_rgb(245, 247, 250),
            panel: Color32::WHITE,
            text: Color32::from_rgb(40, 50, 60),
            border: Color32::from_rgb(230, 235, 240),
            active: Color32::from_rgb(220, 230, 240),
            hover: Color32::from_rgb(235, 240, 245),
            shadow: Color32::from_black_alpha(20),
            success: Color32::from_rgb(76, 175, 80),
            team: Color32::from_rgb(33, 150, 243),
            private: Color32::from_rgb(156, 39, 176),
        }
    }
}

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

// Main rendering function
pub fn render(app: &mut CircleApp, ctx: &Context) {
    let theme = Theme::new();
    let config = LayoutConfig::new();

    setup_style(ctx, &theme, &config);
    let app_layout = create_app_layout(&theme);

    render_top_panel(app, ctx, &app_layout, &theme);
    if !app.is_first_time {
        render_navigation_bar(app, ctx, &app_layout, &theme, &config);
    }
    render_status_bar(app, ctx, &app_layout, &theme);
    render_circle_selector(app, ctx, &app_layout, &theme, &config);
    render_feature_content(app, ctx, &app_layout);
    render_circle_dialog(app, ctx);
}

// Setup egui style
fn setup_style(ctx: &Context, theme: &Theme, config: &LayoutConfig) {
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = Vec2::new(config.spacing, config.spacing);
    style.spacing.window_margin = Margin::same(config.spacing);
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

fn apply_strokes(style: &mut egui::Style, theme: &Theme) {
    style.visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, theme.border);
    style.visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, theme.border);
    style.visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, theme.accent);
    style.visuals.widgets.hovered.fg_stroke = Stroke::new(1.5, theme.accent);
    style.visuals.widgets.active.bg_stroke = Stroke::new(1.5, theme.accent);
    style.visuals.widgets.active.fg_stroke = Stroke::new(1.5, theme.accent);
    style.visuals.selection.stroke = Stroke::new(1.0, Color32::WHITE);
}

fn apply_rounding(style: &mut egui::Style) {
    style.visuals.window_shadow = egui::epaint::Shadow {
        extrusion: 6.0,
        color: Color32::from_black_alpha(25),
    };
    style.visuals.window_rounding = Rounding::same(0.0);
    style.visuals.menu_rounding = Rounding::same(0.0);
}

fn create_app_layout(theme: &Theme) -> Frame {
    Frame::none()
        .fill(theme.background)
        .inner_margin(Margin::same(8.0))
}

// Panel rendering functions
fn render_top_panel(app: &CircleApp, ctx: &Context, app_layout: &Frame, theme: &Theme) {
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

fn render_navigation_bar(
    app: &mut CircleApp,
    ctx: &Context,
    app_layout: &Frame,
    theme: &Theme,
    config: &LayoutConfig,
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
        .exact_height(config.panel_height)
        .frame(app_layout.clone().shadow(egui::epaint::Shadow {
            extrusion: 4.0,
            color: theme.shadow,
        }))
        .show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.style_mut().spacing.item_spacing = Vec2::new(8.0, 0.0);
                ui.style_mut().visuals.widgets.hovered.expansion = 1.0;
                for (text, feature) in NAV_ITEMS {
                    if create_nav_button(ui, text, *feature, app.active_feature, theme, config) {
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
    config: &LayoutConfig,
) -> bool {
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

fn render_status_bar(app: &CircleApp, ctx: &Context, app_layout: &Frame, theme: &Theme) {
    TopBottomPanel::bottom("status_bar")
        .exact_height(40.0)
        .frame(
            app_layout
                .clone()
                .shadow(egui::epaint::Shadow {
                    extrusion: 4.0,
                    color: theme.shadow,
                })
                .rounding(Rounding::same(0.0)),
        )
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(16.0);
                ui.label(
                    RichText::new("Status: Connected")
                        .size(13.0)
                        .color(Color32::from_rgb(70, 80, 90)),
                );
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.add_space(16.0);
                    let user_name = app.user.as_ref().map_or("Guest", |u| &u.name);
                    ui.label(
                        RichText::new(format!("User: {}", user_name))
                            .size(13.0)
                            .color(Color32::from_rgb(70, 80, 90)),
                    );
                });
            });
        });
}

// Circle selector and related functions
fn render_circle_selector(
    app: &mut CircleApp,
    ctx: &Context,
    app_layout: &Frame,
    theme: &Theme,
    config: &LayoutConfig,
) {
    SidePanel::left("circle_selector")
        .resizable(false)
        .exact_width(config.sidebar_width)
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
                ui.add_space(16.0);
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
        ui.add_space(8.0);
        ui.vertical(|ui| {
            ui.add_space(5.0);
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
                .rounding(Rounding::same(8.0))
                .fill(theme.accent)
                .stroke(Stroke::new(1.0, Color32::from_rgb(45, 100, 200)));

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
    ui.painter().hline(
        ui.available_rect_before_wrap().x_range(),
        ui.cursor().top(),
        Stroke::new(1.0, theme.border),
    );
}

fn render_search_box(ui: &mut Ui, app: &mut CircleApp, theme: &Theme) {
    let search_frame = Frame::none()
        .fill(theme.hover)
        .rounding(Rounding::same(20.0))
        .inner_margin(Margin::same(10.0))
        .stroke(Stroke::new(1.0, theme.border));

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

fn render_circle_list(ui: &mut Ui, app: &mut CircleApp, theme: &Theme) {
    let all_circles: Vec<_> = app
        .circles
        .iter()
        .map(|c| (c.id, c.name.clone(), c.circle_type))
        .collect();

    let (default_circles, user_circles): (Vec<_>, Vec<_>) = all_circles
        .into_iter()
        .partition(|(_, name, _)| name == "Welcome to Circles" || name == "Circles Bot Channel");

    let active_circle_id = app.active_circle_id;

    // ALL CIRCLES section
    render_circle_section(
        ui,
        "ALL CIRCLES",
        &user_circles,
        active_circle_id,
        app,
        theme,
        "No circles yet. Click the + button to create one.",
    );

    // FAVORITES section
    render_circle_section(
        ui,
        "FAVORITES",
        &[],
        active_circle_id,
        app,
        theme,
        "No favorites yet",
    );

    // MY CIRCLES section
    render_circle_section(
        ui,
        "MY CIRCLES",
        &[],
        active_circle_id,
        app,
        theme,
        "No circles yet",
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

fn render_circle_section(
    ui: &mut Ui,
    title: &str,
    circles: &[(Uuid, String, crate::models::circle::CircleType)],
    active_circle_id: Option<Uuid>,
    app: &mut CircleApp,
    theme: &Theme,
    empty_message: &str,
) {
    ui.collapsing(
        RichText::new(title).size(14.0).strong().color(theme.accent),
        |ui| {
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
        },
    );
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

    let circle_rect = ui.available_rect_before_wrap();
    let circle_frame = Frame::none()
        .fill(if is_active {
            theme.hover
        } else {
            Color32::TRANSPARENT
        })
        .inner_margin(Margin::symmetric(8.0, 6.0))
        .rounding(Rounding::same(4.0));

    circle_frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.set_min_width(ui.available_width());
            let icon_frame = Frame::none()
                .fill(if is_active { color } else { theme.border })
                .rounding(Rounding::same(12.0))
                .inner_margin(Margin::same(6.0));

            icon_frame.show(ui, |ui| {
                ui.label(RichText::new(icon).size(16.0).color(if is_active {
                    Color32::WHITE
                } else {
                    color
                }));
            });

            ui.add_space(12.0);
            ui.vertical(|ui| {
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
        });
    });

    let response = ui.interact(circle_rect, ui.id().with(id), Sense::click());
    if response.clicked() {
        app.set_active_circle(id);
        match name {
            "Circles Bot Channel" => app.set_active_feature(ActiveFeature::BotChannel),
            "Welcome to Circles" => app.set_active_feature(ActiveFeature::Welcome),
            _ => {
                if matches!(
                    app.active_feature,
                    ActiveFeature::Welcome | ActiveFeature::BotChannel
                ) {
                    app.set_active_feature(ActiveFeature::Chat);
                }
            }
        }
    }
    response.on_hover_cursor(CursorIcon::PointingHand);
    ui.add_space(4.0);
}

fn circle_type_name(circle_type: crate::models::circle::CircleType) -> &'static str {
    match circle_type {
        crate::models::circle::CircleType::Personal => "Personal",
        crate::models::circle::CircleType::Team => "Team",
        crate::models::circle::CircleType::Private => "Private",
    }
}

// Feature content rendering
fn render_feature_content(app: &CircleApp, ctx: &Context, app_layout: &Frame) {
    CentralPanel::default()
        .frame(app_layout.clone())
        .show(ctx, |ui| match app.active_feature {
            ActiveFeature::Mail => mail::render_mail(app, ui),
            ActiveFeature::Calendar => calendar::render_calendar(app, ui),
            ActiveFeature::Chat => chat::render_chat(app, ui),
            ActiveFeature::Documents => documents::render_documents(app, ui),
            ActiveFeature::AITools => render_ai_tools(app, ui),
            ActiveFeature::VideoConference => video_conf::render_video_conference(app, ui),
            ActiveFeature::Settings => render_settings(app, ui),
            ActiveFeature::Welcome => welcome::render_welcome_screen(app, ui), // Updated to use the new module
            ActiveFeature::BotChannel => bot_channel::render_bot_channel(app, ui),
        });
}

fn render_circle_dialog(app: &mut CircleApp, ctx: &Context) {
    if app.circle_dialog_state.is_open {
        if let Some(user) = &app.user {
            if let Some(new_circle) =
                circle_dialog::render_circle_dialog(&mut app.circle_dialog_state, ctx, user.id)
            {
                app.add_circle(new_circle);
            }
        }
    }
}

#[allow(unused_variables)]
fn render_ai_tools(app: &CircleApp, ui: &mut Ui) {
    render_header(ui, "🤖", "AI Tools");
    ui.add_space(16.0);
    create_content_frame().show(ui, |ui| {
        ui.label(
            RichText::new("AI Tools feature not yet implemented")
                .size(14.0)
                .color(Color32::from_rgb(100, 110, 120)),
        );
    });
}

#[allow(unused_mut)]
#[allow(unused_variables)]
fn render_settings(app: &CircleApp, ui: &mut Ui) {
    render_header(ui, "⚙️", "Settings");
    ui.add_space(16.0);
    create_content_frame().show(ui, |ui| {
        ui.vertical(|ui| {
            render_settings_section(
                ui,
                "User Settings",
                &[
                    (true, "Enable notifications"),
                    (false, "Dark mode"),
                    (true, "Auto-save"),
                ],
            );
            ui.add_space(16.0);
            render_settings_section(
                ui,
                "Circle Settings",
                &[(true, "Show all circles"), (false, "Auto-join new circles")],
            );
            ui.add_space(16.0);
            if ui
                .add(
                    Button::new(
                        RichText::new("Save Settings")
                            .size(14.0)
                            .color(Color32::WHITE),
                    )
                    .fill(Color32::from_rgb(66, 133, 244))
                    .rounding(Rounding::same(6.0))
                    .min_size(Vec2::new(120.0, 36.0)),
                )
                .clicked()
            {
                // TODO: Implement save settings
            }
        });
    });
}

fn render_settings_section(ui: &mut Ui, title: &str, settings: &[(bool, &str)]) {
    let theme = Theme::new();
    ui.label(RichText::new(title).size(16.0).strong().color(theme.text));
    ui.add_space(8.0);
    ui.painter().hline(
        ui.available_rect_before_wrap().x_range(),
        ui.cursor().top(),
        Stroke::new(1.0, theme.border),
    );
    ui.add_space(12.0);
    for (mut value, text) in settings.iter() {
        ui.checkbox(
            &mut value,
            RichText::new(*text)
                .size(14.0)
                .color(Color32::from_rgb(70, 80, 90)),
        );
    }
}

// Utility functions
pub fn render_header(ui: &mut Ui, icon: &str, title: &str) {
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        Frame::none()
            .fill(Color32::from_rgb(235, 240, 245))
            .rounding(Rounding::same(0.0))
            .inner_margin(Margin::symmetric(12.0, 8.0))
            .show(ui, |ui| {
                ui.label(
                    RichText::new(format!("{} {}", icon, title))
                        .size(18.0)
                        .strong()
                        .color(Color32::from_rgb(40, 50, 60)),
                );
            });
    });
    ui.add_space(16.0);
}

pub fn create_action_button<'a>(text: &'a str, icon: &'a str) -> Button<'a> {
    Button::new(
        RichText::new(format!("{} {}", icon, text))
            .size(14.0)
            .color(Color32::WHITE),
    )
    .rounding(Rounding::same(20.0))
    .fill(Color32::from_rgb(66, 133, 244))
    .stroke(Stroke::new(1.0, Color32::from_rgb(45, 100, 200)))
    .min_size(Vec2::new(100.0, 36.0))
}

pub fn create_content_frame() -> Frame {
    Frame::none()
        .fill(Color32::WHITE)
        .stroke(Stroke::new(1.0, Color32::from_rgb(230, 235, 240)))
        .rounding(Rounding::same(0.0))
        .inner_margin(Margin::same(16.0))
        .shadow(egui::epaint::Shadow {
            extrusion: 4.0,
            color: Color32::from_black_alpha(20),
        })
}
