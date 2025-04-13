use crate::app::{ActiveFeature, CircleApp};
use crate::ui::components::circle_dialog;
use crate::ui::features::bot_channel;
use crate::ui::features::calendar;
use crate::ui::features::chat;
use crate::ui::features::documents;
use crate::ui::features::mail;
use crate::ui::features::video_conf;
use eframe::egui::{self, Color32, RichText, Rounding, Stroke, Vec2};
use uuid::Uuid;

pub fn render(app: &mut CircleApp, ctx: &egui::Context) {
    setup_style(ctx);
    let app_layout = create_app_layout(ctx);

    // Always render the top panel
    render_top_panel(app, ctx, app_layout);

    // Only render the navigation bar if it's not the first time
    if !app.is_first_time {
        render_navigation_bar(app, ctx, app_layout);
    }

    // Always render the status bar
    render_status_bar(app, ctx, app_layout);

    // Always render the circle selector
    render_circle_selector(app, ctx, app_layout);

    // Render the feature content (removed duplicate call)
    render_feature_content(app, ctx, app_layout);

    // Render the circle creation dialog if it's open
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

fn create_app_layout(_ctx: &egui::Context) -> egui::Frame {
    egui::Frame::none()
        .fill(Color32::from_rgb(245, 247, 250))
        .inner_margin(egui::Margin::same(8.0))
}

fn setup_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = Vec2::new(12.0, 12.0);
    style.spacing.window_margin = egui::Margin::same(16.0);
    style.spacing.button_padding = Vec2::new(10.0, 6.0);

    let accent_color = Color32::from_rgb(66, 133, 244);
    let bg_color = Color32::from_rgb(245, 247, 250);
    let panel_color = Color32::from_rgb(255, 255, 255);
    let text_color = Color32::from_rgb(40, 50, 60);

    style.visuals.override_text_color = Some(text_color);
    style.visuals.widgets.noninteractive.bg_fill = bg_color;
    style.visuals.widgets.inactive.bg_fill = panel_color;
    style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(235, 240, 245);
    style.visuals.widgets.active.bg_fill = Color32::from_rgb(220, 230, 240);
    style.visuals.selection.bg_fill = accent_color;

    apply_strokes(&mut style, accent_color);
    apply_rounding(&mut style);
    ctx.set_style(style);
}

fn apply_strokes(style: &mut egui::Style, accent_color: Color32) {
    let border_color = Color32::from_rgb(230, 235, 240);
    style.visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, border_color);
    style.visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, border_color);
    style.visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, accent_color);
    style.visuals.widgets.hovered.fg_stroke = Stroke::new(1.5, accent_color);
    style.visuals.widgets.active.bg_stroke = Stroke::new(1.5, accent_color);
    style.visuals.widgets.active.fg_stroke = Stroke::new(1.5, accent_color);
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

fn render_top_panel(app: &CircleApp, ctx: &egui::Context, app_layout: egui::Frame) {
    egui::TopBottomPanel::top("top_panel")
        .exact_height(60.0)
        .frame(
            app_layout
                .shadow(egui::epaint::Shadow {
                    extrusion: 6.0,
                    color: Color32::from_black_alpha(25),
                })
                .rounding(Rounding {
                    nw: 0.0,
                    ne: 0.0,
                    sw: 0.0,
                    se: 0.0,
                }),
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
                            .color(Color32::from_rgb(40, 50, 60)),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
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
                                Color32::from_rgb(76, 175, 80),
                            );
                        }
                    });
                });
                ui.add_space(8.0);
                ui.painter().hline(
                    ui.available_rect_before_wrap().x_range(),
                    ui.cursor().top(),
                    Stroke::new(1.0, Color32::from_rgb(230, 235, 240)),
                );
            });
        });
}

fn render_navigation_bar(app: &mut CircleApp, ctx: &egui::Context, app_layout: egui::Frame) {
    egui::TopBottomPanel::top("navigation_bar")
        .exact_height(50.0)
        .frame(app_layout.shadow(egui::epaint::Shadow {
            extrusion: 4.0,
            color: Color32::from_black_alpha(20),
        }))
        .show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.style_mut().spacing.item_spacing = Vec2::new(8.0, 0.0);
                ui.style_mut().visuals.widgets.hovered.expansion = 1.0;
                let active_feature = app.active_feature;
                for (text, feature) in NAV_ITEMS {
                    if create_nav_button(ui, text, *feature, active_feature) {
                        app.set_active_feature(*feature);
                    }
                }
            });
        });
}

const NAV_ITEMS: &[(&str, ActiveFeature)] = &[
    // Ordered alphabetically
    ("AI Tools", ActiveFeature::AITools),
    ("Calendar", ActiveFeature::Calendar),
    ("Chat", ActiveFeature::Chat),
    ("Documents", ActiveFeature::Documents),
    ("Mail", ActiveFeature::Mail),
    ("Settings", ActiveFeature::Settings),
    ("Video", ActiveFeature::VideoConference),
];

fn create_nav_button(
    ui: &mut egui::Ui,
    text: &str,
    feature: ActiveFeature,
    active_feature: ActiveFeature,
) -> bool {
    let is_active = active_feature == feature;
    let mut button = egui::Button::new(RichText::new(text).size(14.0).color(if is_active {
        Color32::WHITE
    } else {
        Color32::from_rgb(70, 80, 90)
    }))
    .min_size(Vec2::new(90.0, 40.0))
    .rounding(Rounding::same(6.0))
    .sense(egui::Sense::click_and_drag());

    button = if is_active {
        button
            .fill(Color32::from_rgb(66, 133, 244))
            .stroke(Stroke::new(1.0, Color32::from_rgb(45, 100, 200)))
    } else {
        button
            .fill(Color32::from_rgb(230, 235, 240))
            .stroke(Stroke::NONE)
    };

    let response = ui
        .add(button)
        .on_hover_ui(|ui| {
            ui.style_mut().visuals.widgets.hovered.bg_fill = Color32::from_rgb(66, 133, 244);
            ui.label(RichText::new(text).size(12.0).color(Color32::WHITE));
        })
        .on_hover_cursor(egui::CursorIcon::PointingHand);

    if response.hovered() {
        ui.ctx().request_repaint();
    }
    response.clicked()
}

fn render_status_bar(app: &CircleApp, ctx: &egui::Context, app_layout: egui::Frame) {
    egui::TopBottomPanel::bottom("status_bar")
        .exact_height(40.0)
        .frame(
            app_layout
                .shadow(egui::epaint::Shadow {
                    extrusion: 4.0,
                    color: Color32::from_black_alpha(20),
                })
                .rounding(Rounding {
                    nw: 0.0,
                    ne: 0.0,
                    sw: 0.0,
                    se: 0.0,
                }),
        )
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(16.0);
                ui.label(
                    RichText::new("Status: Connected")
                        .size(13.0)
                        .color(Color32::from_rgb(70, 80, 90)),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
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

fn render_circle_selector(app: &mut CircleApp, ctx: &egui::Context, app_layout: egui::Frame) {
    egui::SidePanel::left("circle_selector")
        .resizable(false)
        .exact_width(260.0)
        .frame(
            app_layout
                .shadow(egui::epaint::Shadow {
                    extrusion: 6.0,
                    color: Color32::from_black_alpha(25),
                })
                .rounding(Rounding {
                    nw: 0.0,
                    ne: 0.0,
                    sw: 0.0,
                    se: 0.0,
                }),
        )
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.add_space(16.0);
                render_circle_header(ui, app);
                ui.add_space(12.0);
                render_search_box(ui, app);
                ui.add_space(16.0);
                render_circle_list(ui, app);
                ui.add_space(8.0);
            });
        });
}

fn render_circle_header(ui: &mut egui::Ui, app: &mut CircleApp) {
    ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
        ui.add_space(8.0);

        ui.vertical(|ui| {
            ui.add_space(5.0);
            ui.heading(
                RichText::new("Circles")
                    .size(18.0)
                    .strong()
                    .color(Color32::from_rgb(40, 50, 60)),
            );
        });

        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
            let add_button =
                egui::Button::new(RichText::new("➕").size(16.0).color(Color32::WHITE))
                    .min_size(Vec2::new(32.0, 32.0))
                    .rounding(Rounding::same(8.0))
                    .fill(Color32::from_rgb(66, 133, 244))
                    .stroke(Stroke::new(1.0, Color32::from_rgb(45, 100, 200)));

            if ui
                .add(add_button)
                .on_hover_text(
                    RichText::new("Create a new circle")
                        .size(12.0)
                        .color(Color32::WHITE),
                )
                .on_hover_cursor(egui::CursorIcon::PointingHand)
                .clicked()
            {
                app.open_circle_dialog();
            }
        });
    });

    ui.painter().hline(
        ui.available_rect_before_wrap().x_range(),
        ui.cursor().top(),
        Stroke::new(1.0, Color32::from_rgb(230, 235, 240)),
    );
}

fn render_search_box(ui: &mut egui::Ui, app: &mut CircleApp) {
    let search_frame = egui::Frame::none()
        .fill(Color32::from_rgb(235, 240, 245))
        .rounding(Rounding::same(20.0))
        .inner_margin(egui::Margin::same(10.0))
        .stroke(Stroke::new(1.0, Color32::from_rgb(230, 235, 240)));

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
                        .text_color(Color32::from_rgb(40, 50, 60))
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

fn render_circle_list(ui: &mut egui::Ui, app: &mut CircleApp) {
    let all_circles = app
        .circles
        .iter()
        .map(|c| (c.id, c.name.clone(), c.circle_type))
        .collect::<Vec<_>>();

    let default_circles: Vec<_> = all_circles
        .iter()
        .filter(|(_, name, _)| name == "Welcome to Circles" || name == "Circles Bot Channel")
        .cloned()
        .collect();

    let user_circles: Vec<_> = all_circles
        .iter()
        .filter(|(_, name, _)| name != "Welcome to Circles" && name != "Circles Bot Channel")
        .cloned()
        .collect();

    let active_circle_id = app.active_circle_id;
    let has_user_circles = !user_circles.is_empty();

    ui.collapsing(
        RichText::new("ALL CIRCLES")
            .size(14.0)
            .strong()
            .color(Color32::from_rgb(66, 133, 244)),
        |ui| {
            ui.add_space(8.0);

            if !has_user_circles {
                ui.label(
                    RichText::new("No circles yet. Click the + button to create one.")
                        .size(13.0)
                        .color(Color32::from_rgb(100, 110, 120)),
                );
            } else {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (id, name, circle_type) in &user_circles {
                        render_circle_item(ui, *id, name, *circle_type, active_circle_id, app);
                    }
                });
            }

            ui.add_space(8.0);
        },
    );
    ui.add_space(6.0);

    ui.collapsing(
        RichText::new("FAVORITES")
            .size(14.0)
            .strong()
            .color(Color32::from_rgb(66, 133, 244)),
        |ui| {
            ui.add_space(8.0);
            ui.label(
                RichText::new("No favorites yet")
                    .size(13.0)
                    .color(Color32::from_rgb(100, 110, 120)),
            );
            ui.add_space(8.0);
        },
    );
    ui.add_space(6.0);

    ui.collapsing(
        RichText::new("MY CIRCLES")
            .size(14.0)
            .strong()
            .color(Color32::from_rgb(66, 133, 244)),
        |ui| {
            ui.add_space(8.0);
            ui.label(
                RichText::new("No circles yet")
                    .size(13.0)
                    .color(Color32::from_rgb(100, 110, 120)),
            );
            ui.add_space(8.0);
        },
    );
    ui.add_space(6.0);

    egui::CollapsingHeader::new(
        RichText::new("OTHERS")
            .size(14.0)
            .strong()
            .color(Color32::from_rgb(66, 133, 244)),
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
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (id, name, circle_type) in &default_circles {
                    render_circle_item(ui, *id, name, *circle_type, active_circle_id, app);
                }
            });
        }

        ui.add_space(8.0);
    });
    ui.add_space(6.0);
}

fn render_circle_item(
    ui: &mut egui::Ui,
    id: Uuid,
    name: &str,
    circle_type: crate::models::circle::CircleType,
    active_circle_id: Option<Uuid>,
    app: &mut CircleApp,
) {
    let is_active = active_circle_id.map_or(false, |active_id| active_id == id);
    let (icon, color) = match circle_type {
        crate::models::circle::CircleType::Personal => ("👤", Color32::from_rgb(76, 175, 80)),
        crate::models::circle::CircleType::Team => ("👥", Color32::from_rgb(33, 150, 243)),
        crate::models::circle::CircleType::Private => ("🔒", Color32::from_rgb(156, 39, 176)),
    };

    let circle_rect = ui.available_rect_before_wrap();

    let circle_frame = egui::Frame::none()
        .fill(if is_active {
            Color32::from_rgb(235, 240, 250)
        } else {
            Color32::TRANSPARENT
        })
        .inner_margin(egui::Margin::symmetric(8.0, 6.0))
        .rounding(Rounding::same(4.0));

    circle_frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.set_min_width(ui.available_width());

            let icon_frame = egui::Frame::none()
                .fill(if is_active {
                    color
                } else {
                    Color32::from_rgb(230, 235, 240)
                })
                .rounding(Rounding::same(12.0))
                .inner_margin(egui::Margin::same(6.0));

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
                    Color32::from_rgb(66, 133, 244)
                } else {
                    Color32::from_rgb(40, 50, 60)
                }));
                ui.label(
                    RichText::new(format!("{} Circle", circle_type_name(circle_type)))
                        .size(12.0)
                        .color(Color32::from_rgb(100, 110, 120)),
                );
            });
        });
    });

    let response = ui.interact(circle_rect, ui.id().with(id), egui::Sense::click());

    if response.clicked() {
        app.set_active_circle(id);

        // Set the active feature based on the circle name
        if name == "Circles Bot Channel" {
            app.set_active_feature(ActiveFeature::BotChannel); // Correctly set to BotChannel
        } else if name == "Welcome to Circles" {
            app.set_active_feature(ActiveFeature::Welcome);
        } else {
            // For user-created circles, ensure we don't override the current feature unnecessarily
            if app.active_feature == ActiveFeature::Welcome
                || app.active_feature == ActiveFeature::BotChannel
            {
                app.set_active_feature(ActiveFeature::Chat); // Default to Chat for user circles
            }
        }
    }

    response.on_hover_cursor(egui::CursorIcon::PointingHand);

    ui.add_space(4.0);
}

fn circle_type_name(circle_type: crate::models::circle::CircleType) -> &'static str {
    match circle_type {
        crate::models::circle::CircleType::Personal => "Personal",
        crate::models::circle::CircleType::Team => "Team",
        crate::models::circle::CircleType::Private => "Private",
    }
}

fn render_feature_content(app: &CircleApp, ctx: &egui::Context, app_layout: egui::Frame) {
    egui::CentralPanel::default()
        .frame(app_layout)
        .show(ctx, |ui| match app.active_feature {
            ActiveFeature::Mail => mail::render_mail(app, ui),
            ActiveFeature::Calendar => calendar::render_calendar(app, ui),
            ActiveFeature::Chat => chat::render_chat(app, ui),
            ActiveFeature::Documents => documents::render_documents(app, ui),
            ActiveFeature::AITools => render_ai_tools(app, ui),
            ActiveFeature::VideoConference => video_conf::render_video_conference(app, ui),
            ActiveFeature::Settings => render_settings(app, ui),
            ActiveFeature::Welcome => render_welcome_screen(app, ui),
            ActiveFeature::BotChannel => bot_channel::render_bot_channel(app, ui),
        });
}

fn render_welcome_screen(_app: &CircleApp, ui: &mut egui::Ui) {
    let available_height = ui.available_height();

    let welcome_frame = egui::Frame::none()
        .fill(Color32::WHITE)
        .inner_margin(egui::Margin::same(0.0))
        .outer_margin(egui::Margin::same(0.0));

    welcome_frame.show(ui, |ui| {
        ui.set_min_height(available_height);

        ui.vertical_centered(|ui| {
            ui.add_space(available_height * 0.15);

            ui.heading(
                RichText::new("Welcome to Circles")
                    .size(32.0)
                    .strong()
                    .color(Color32::from_rgb(66, 133, 244)),
            );

            ui.add_space(20.0);

            ui.label(
                RichText::new("Your new collaboration platform for teams and individuals")
                    .size(18.0)
                    .color(Color32::from_rgb(70, 80, 90)),
            );

            ui.add_space(40.0);

            ui.label(
                RichText::new("To get started:")
                    .size(16.0)
                    .strong()
                    .color(Color32::from_rgb(40, 50, 60)),
            );

            ui.add_space(10.0);

            let instructions = [
                "• Check out the 'Welcome to Circles' guide in the OTHERS section",
                "• Explore the different features using the navigation bar above",
                "• Create your own circles using the + button in the sidebar",
                "• Stay updated with the Circles Bot Channel in the OTHERS section",
            ];

            for instruction in instructions {
                ui.label(
                    RichText::new(instruction)
                        .size(16.0)
                        .color(Color32::from_rgb(70, 80, 90)),
                );
                ui.add_space(8.0);
            }

            ui.add_space(40.0);

            let get_started_button = egui::Button::new(
                RichText::new("Get Started")
                    .size(18.0)
                    .color(Color32::WHITE),
            )
            .min_size(Vec2::new(180.0, 50.0))
            .rounding(Rounding::same(8.0))
            .fill(Color32::from_rgb(66, 133, 244))
            .stroke(Stroke::new(1.0, Color32::from_rgb(45, 100, 200)));

            if ui
                .add(get_started_button)
                .on_hover_cursor(egui::CursorIcon::PointingHand)
                .clicked()
            {
                // This would need to be handled in the app state
            }

            ui.add_space(available_height * 0.15);
        });
    });
}

#[allow(unused_variables)]
fn render_ai_tools(app: &CircleApp, ui: &mut egui::Ui) {
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
fn render_settings(app: &CircleApp, ui: &mut egui::Ui) {
    render_header(ui, "⚙️", "Settings");
    ui.add_space(16.0);
    create_content_frame().show(ui, |ui| {
        ui.vertical(|ui| {
            ui.label(
                RichText::new("User Settings")
                    .size(16.0)
                    .strong()
                    .color(Color32::from_rgb(40, 50, 60)),
            );
            ui.add_space(8.0);
            ui.painter().hline(
                ui.available_rect_before_wrap().x_range(),
                ui.cursor().top(),
                Stroke::new(1.0, Color32::from_rgb(230, 235, 240)),
            );
            ui.add_space(12.0);
            for (mut value, text) in [
                (true, "Enable notifications"),
                (false, "Dark mode"),
                (true, "Auto-save"),
            ] {
                ui.checkbox(
                    &mut value,
                    RichText::new(text)
                        .size(14.0)
                        .color(Color32::from_rgb(70, 80, 90)),
                );
            }
            ui.add_space(16.0);
            ui.label(
                RichText::new("Circle Settings")
                    .size(16.0)
                    .strong()
                    .color(Color32::from_rgb(40, 50, 60)),
            );
            ui.add_space(8.0);
            ui.painter().hline(
                ui.available_rect_before_wrap().x_range(),
                ui.cursor().top(),
                Stroke::new(1.0, Color32::from_rgb(230, 235, 240)),
            );
            ui.add_space(12.0);
            for (mut value, text) in [(true, "Show all circles"), (false, "Auto-join new circles")]
            {
                ui.checkbox(
                    &mut value,
                    RichText::new(text)
                        .size(14.0)
                        .color(Color32::from_rgb(70, 80, 90)),
                );
            }
            ui.add_space(16.0);
            if ui
                .add(
                    egui::Button::new(
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

pub fn render_header(ui: &mut egui::Ui, icon: &str, title: &str) {
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        egui::Frame::none()
            .fill(Color32::from_rgb(235, 240, 245))
            .rounding(Rounding::same(0.0))
            .inner_margin(egui::Margin::symmetric(12.0, 8.0))
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

pub fn create_action_button<'a>(text: &'a str, icon: &'a str) -> egui::Button<'a> {
    egui::Button::new(
        RichText::new(format!("{} {}", icon, text))
            .size(14.0)
            .color(Color32::WHITE),
    )
    .rounding(Rounding::same(20.0))
    .fill(Color32::from_rgb(66, 133, 244))
    .stroke(Stroke::new(1.0, Color32::from_rgb(45, 100, 200)))
    .min_size(Vec2::new(100.0, 36.0))
}

pub fn create_content_frame() -> egui::Frame {
    egui::Frame::none()
        .fill(Color32::WHITE)
        .stroke(Stroke::new(1.0, Color32::from_rgb(230, 235, 240)))
        .rounding(Rounding::same(0.0))
        .inner_margin(egui::Margin::same(16.0))
        .shadow(egui::epaint::Shadow {
            extrusion: 4.0,
            color: Color32::from_black_alpha(20),
        })
}
