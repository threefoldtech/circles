use crate::app::CircleApp;
use crate::ui::components::notifications_panel::{
    NotificationsPanelProps, render_notifications_panel,
};
use crate::utils::config::Theme;
use chrono::Local;
use eframe::egui::{
    Align, Color32, Context, Frame, Layout, Margin, Pos2, Response, RichText, Sense, Stroke,
    TopBottomPanel, Ui, Vec2,
};
use std::fs;

use super::sidebar::render_user_status_menu;

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
#[derive(Debug, Default)]
pub struct UserMenuState {
    pub show_menu: bool,
}

// Handle user menu click based on index
fn handle_user_menu_click(app: &mut CircleApp, index: usize) {
    match index {
        0 => {
            // Profile
            // Handle profile action
        }
        1 => {
            // Edit Profile
            // Handle edit profile action
        }
        2 => {
            // Snooze
            // Handle snooze action
        }
        3 => {
            // Logout
            handle_logout(app);
        }
        _ => {}
    }
}

fn render_user_status(app: &mut CircleApp, _: &Context, ui: &mut Ui, theme: &Theme) {
    let user_name = app.user.as_ref().map_or("Guest", |u| &u.name);
    let user_frame = StatusFrameProps::new(theme)
        .with_margin(Margin::symmetric(10, 4))
        .build()
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let response = ui.label(create_styled_text(user_name, theme.text, 13.0, true));
                ui.add_space(4.0);
                render_status_dot(ui, theme, 6.0);
                
                // Check if the label was clicked
                if response.clicked() {
                    app.user_menu_state.show_menu = !app.user_menu_state.show_menu;
                }
            });
        });
    
    // Check if the frame itself was clicked
    if user_frame.response.clicked() {
        app.user_menu_state.show_menu = !app.user_menu_state.show_menu;
    }
    
    set_hover_cursor(ui, &user_frame.response);

    // Show context menu when clicked
    if app.user_menu_state.show_menu {
        // Use the context_menu method to maintain the original style
        user_frame.response.context_menu(|ui| {
            // Use the render_user_status_menu function from sidebar/context_menu.rs
            let clicked_indices = render_user_status_menu(ui, app, theme);

            // Handle clicked items
            if !clicked_indices.is_empty() {
                let index = clicked_indices[0]; // Get the first clicked index
                handle_user_menu_click(app, index);
            }
        });
    }
}

// Render logout confirmation dialog
pub fn render_logout_confirmation_dialog(ctx: &Context, app: &mut CircleApp, theme: &Theme) {
    // Only proceed if the dialog should be shown
    if !app.logout_confirmation_state {
        return;
    }

    // Create a simple confirmation dialog
    egui::Window::new("Confirm Logout")
        .fixed_size([400.0, 200.0])
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .collapsible(false)
        .resizable(false)
        .frame(
            egui::Frame::window(&ctx.style())
                .fill(theme.background)
                .corner_radius(16)
                .shadow(egui::epaint::Shadow {
                    color: theme.shadow,
                    offset: [0, 4],
                    blur: 8,
                    spread: 0,
                })
                .inner_margin(egui::Margin::same(24)),
        )
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.heading(egui::RichText::new("Confirm Logout").color(theme.error));
                ui.add_space(10.0);
                ui.label("Are you sure you want to log out? You will need to sign in again to access your circles.");
                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    let cancel_button = egui::Button::new(
                        egui::RichText::new("Cancel").color(theme.text)
                    ).fill(theme.accent);
                    
                    if ui.add(cancel_button).clicked() {
                        app.logout_confirmation_state = false;
                    }

                    let confirm_button = egui::Button::new(
                        egui::RichText::new("Logout").color(theme.white)
                    ).fill(theme.error);

                    if ui.add(confirm_button).clicked() {
                        perform_logout(app);
                        app.logout_confirmation_state = false;
                    }
                });
            });
        });
}

// Handle logout menu click - shows confirmation dialog
fn handle_logout(app: &mut CircleApp) {
    app.logout_confirmation_state = true;
}

// Perform the actual logout action
fn perform_logout(app: &mut CircleApp) {
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
