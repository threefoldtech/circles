use crate::app::CircleApp;
use crate::ui::components::notifications_panel::{
    NotificationsPanelProps, render_notifications_panel,
};
use crate::utils::config::Theme;
use chrono::Local;
use eframe::egui::{
    Align, Color32, Context, Frame, Id, Layout, Margin, Pos2, Response, RichText, Sense, Stroke,
    TopBottomPanel, Ui, Vec2, Window,
};
use std::fs;

pub struct StatusFrameProps<'a> {
    theme: &'a Theme,
    has_unread: bool,
    inner_margin: Margin,
    corner_radius: f32,
}

impl<'a> StatusFrameProps<'a> {
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            has_unread: false,
            inner_margin: Margin::symmetric(10, 4),
            corner_radius: 12.0,
        }
    }

    pub fn with_unread(self, has_unread: bool) -> Self {
        Self { has_unread, ..self }
    }

    pub fn with_margin(self, margin: Margin) -> Self {
        Self {
            inner_margin: margin,
            ..self
        }
    }

    pub fn build(&self) -> Frame {
        Frame::new()
            .fill(if self.has_unread {
                self.theme.error
            } else {
                self.theme.secondary_background
            })
            .corner_radius(self.corner_radius)
            .inner_margin(self.inner_margin)
            .stroke(Stroke::new(1.0, self.theme.border))
    }
}

pub fn create_styled_text(
    text: impl Into<String>,
    color: Color32,
    size: f32,
    strong: bool,
) -> RichText {
    let mut rich_text = RichText::new(text).size(size).color(color);
    if strong {
        rich_text = rich_text.strong();
    }
    rich_text
}

pub fn render_status_dot(ui: &mut Ui, theme: &Theme, offset: f32) {
    ui.painter().circle_filled(
        ui.min_rect().left_center() + Vec2::new(offset, 0.0),
        4.0,
        theme.active,
    );
}

pub fn set_hover_cursor(ui: &mut Ui, response: &Response) {
    if response.hovered() {
        ui.output_mut(|o| o.cursor_icon = eframe::egui::CursorIcon::PointingHand);
    }
}

pub fn render_status_bar(app: &mut CircleApp, ctx: &Context, app_layout: &Frame, theme: &Theme) {
    TopBottomPanel::bottom("status_bar")
        .exact_height(40.0)
        .frame(
            app_layout
                .clone()
                .corner_radius(0)
                .fill(theme.secondary_background),
        )
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                render_left_section(ui, theme);
                render_right_section(app, ctx, ui, theme);
            });
        });

    ctx.request_repaint_after(std::time::Duration::from_secs(1));
}

fn render_date_time(ui: &mut Ui, theme: &Theme) {
    let now = Local::now();
    let date_str = now.format("%a, %d %b %Y").to_string();
    let time_str = now.format("%H:%M:%S").to_string();

    ui.label(create_styled_text(
        format!("📅 {}", date_str),
        theme.text,
        13.0,
        false,
    ));
    ui.add_space(8.0);
    ui.label(create_styled_text(
        format!("🕒 {}", time_str),
        theme.text,
        13.0,
        false,
    ));
}

fn render_left_section(ui: &mut Ui, theme: &Theme) {
    ui.add_space(16.0);

    let status_frame = StatusFrameProps::new(theme)
        .with_margin(Margin::symmetric(10, 4))
        .build()
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                render_status_dot(ui, theme, 6.0);
                ui.add_space(12.0);
                ui.label(create_styled_text("Connected", theme.text, 13.0, true));
            });
        });

    set_hover_cursor(ui, &status_frame.response);
    ui.add_space(12.0);
    render_date_time(ui, theme);
}

// User menu state
#[derive(Debug)]
pub struct UserMenuState {
    pub show_menu: bool,
    pub menu_rect: Option<eframe::epaint::Rect>,
}

impl Default for UserMenuState {
    fn default() -> Self {
        Self {
            show_menu: false,
            menu_rect: None,
        }
    }
}

fn render_user_status(app: &mut CircleApp, ctx: &Context, ui: &mut Ui, theme: &Theme) {
    let user_name = app.user.as_ref().map_or("Guest", |u| &u.name);
    let user_frame = StatusFrameProps::new(theme)
        .with_margin(Margin::symmetric(10, 4))
        .build()
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let response = ui.label(create_styled_text(user_name, theme.text, 13.0, true));
                ui.add_space(4.0);
                render_status_dot(ui, theme, 6.0);
                if response.clicked() {
                    println!("The user frame was clicked!");
                    app.user_menu_state.show_menu = !app.user_menu_state.show_menu;
                }
            });
        });
    set_hover_cursor(ui, &user_frame.response);

    // Show user menu if enabled
    if app.user_menu_state.show_menu {
        let menu_id = Id::new("user_menu");

        // Create a menu window
        Window::new("User Menu")
            .id(menu_id)
            .fixed_pos(Pos2::new(
                user_frame.response.rect.right() - 200.0,
                user_frame.response.rect.bottom() + 5.0,
            ))
            .fixed_size([200.0, 180.0])
            .title_bar(false)
            .frame(Frame::window(&ctx.style()).fill(theme.panel))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.add_space(5.0);

                    // Profile option
                    if ui
                        .add(
                            egui::Button::new(create_styled_text(
                                "👤 Profile",
                                theme.text,
                                14.0,
                                false,
                            ))
                            .frame(false),
                        )
                        .clicked()
                    {
                        app.user_menu_state.show_menu = false;
                        // Handle profile action
                    }

                    ui.add_space(5.0);
                    ui.separator();
                    ui.add_space(5.0);

                    // Edit Profile option
                    if ui
                        .add(
                            egui::Button::new(create_styled_text(
                                "✏️ Edit Profile",
                                theme.text,
                                14.0,
                                false,
                            ))
                            .frame(false),
                        )
                        .clicked()
                    {
                        app.user_menu_state.show_menu = false;
                        // Handle edit profile action
                    }

                    ui.add_space(5.0);
                    ui.separator();
                    ui.add_space(5.0);

                    // Snooze option
                    if ui
                        .add(
                            egui::Button::new(create_styled_text(
                                "💤 Snooze",
                                theme.text,
                                14.0,
                                false,
                            ))
                            .frame(false),
                        )
                        .clicked()
                    {
                        app.user_menu_state.show_menu = false;
                        // Handle snooze action
                    }

                    ui.add_space(5.0);
                    ui.separator();
                    ui.add_space(5.0);

                    // Logout option
                    if ui
                        .add(
                            egui::Button::new(create_styled_text(
                                "🚪 Logout",
                                theme.error,
                                14.0,
                                false,
                            ))
                            .frame(false),
                        )
                        .clicked()
                    {
                        app.user_menu_state.show_menu = false;
                        handle_logout(app);
                    }
                });

                // Store the menu rect for click-outside detection
                app.user_menu_state.menu_rect = Some(ui.min_rect());
            });

        // Close menu when clicking outside
        if ui.input(|i| i.pointer.any_released()) {
            if let Some(pos) = ui.input(|i| i.pointer.interact_pos()) {
                if let Some(menu_rect) = app.user_menu_state.menu_rect {
                    let user_rect = user_frame.response.rect.expand(20.0);
                    if !menu_rect.contains(pos) && !user_rect.contains(pos) {
                        app.user_menu_state.show_menu = false;
                    }
                }
            }
        }
    }
}

// Handle logout action
fn handle_logout(app: &mut CircleApp) {
    // Clear user credentials file
    let mut path = dirs::home_dir().unwrap_or_default();
    path.push(".config");
    path.push("circles.json");

    // Delete the file if it exists
    if path.exists() {
        let _ = fs::remove_file(path);
    }

    // Reset user state
    app.user = None;
    app.is_first_time = true;

    // Switch to auth screen
    app.set_active_feature(crate::app::ActiveFeature::Auth);
}

fn render_right_section(app: &mut CircleApp, ctx: &Context, ui: &mut Ui, theme: &Theme) {
    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
        ui.add_space(16.0);

        let notif_response = render_notification_bell(app, ui, theme);
        handle_notifications_panel(app, ctx, ui, theme, notif_response.rect);
        ui.add_space(12.0);
        render_user_status(app, ctx, ui, theme);
    });
}

fn render_notification_bell(app: &mut CircleApp, ui: &mut Ui, theme: &Theme) -> Response {
    let unread_count = app.notification_manager.unread_count();
    let notif_frame = StatusFrameProps::new(theme)
        .with_unread(unread_count > 0)
        .with_margin(Margin::symmetric(8, 4))
        .build();

    notif_frame
        .show(ui, |ui| {
            let label = create_styled_text(
                format!("🔔 {}", unread_count),
                if unread_count > 0 {
                    theme.light_color
                } else {
                    theme.text
                },
                13.0,
                true,
            );
            let response = ui.add(egui::Label::new(label).sense(Sense::click()));

            if response.clicked() {
                app.notification_manager.show_panel = !app.notification_manager.show_panel;
            }
            set_hover_cursor(ui, &response);
            response
        })
        .inner
}

fn handle_notifications_panel(
    app: &mut CircleApp,
    ctx: &Context,
    ui: &mut Ui,
    theme: &Theme,
    bell_rect: eframe::epaint::Rect,
) {
    if app.notification_manager.show_panel {
        let screen_rect = ctx.screen_rect();
        let panel_height = 600.0;
        let panel_pos = Pos2::new(
            screen_rect.right() - 320.0,
            screen_rect.bottom() - panel_height - 50.0,
        );

        let panel_rect = render_notifications_panel(
            app,
            ctx,
            NotificationsPanelProps {
                pos: panel_pos,
                height: panel_height,
                theme,
            },
        );

        if ui.input(|i| i.pointer.any_released()) {
            let notification_dialog_open = app.notification_manager.notification_dialog.is_open;
            if !notification_dialog_open {
                if let Some(pos) = ui.input(|i| i.pointer.interact_pos()) {
                    let bell_rect = bell_rect.expand(20.0);
                    if !panel_rect.contains(pos) && !bell_rect.contains(pos) {
                        app.notification_manager.show_panel = false;
                    }
                }
            }
        }
    }
}
