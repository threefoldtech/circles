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
                Color32::from_rgb(240, 70, 70)
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
    theme: &Theme,
    size: f32,
    strong: bool,
) -> RichText {
    let mut rich_text = RichText::new(text).size(size).color(theme.text);
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
        .frame(app_layout.clone().corner_radius(0))
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                render_left_section(ui, theme);
                render_right_section(app, ctx, ui, theme);
            });
        });
}

fn render_date_time(ui: &mut Ui, theme: &Theme) {
    let now = Local::now();
    let date_str = now.format("%a, %d %b %Y").to_string();
    let time_str = now.format("%H:%M:%S").to_string();

    ui.label(create_styled_text(
        format!("📅 {}", date_str),
        theme,
        13.0,
        false,
    ));
    ui.add_space(8.0);
    ui.label(create_styled_text(
        format!("🕒 {}", time_str),
        theme,
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
                ui.label(create_styled_text("Connected", theme, 13.0, true));
            });
        });

    set_hover_cursor(ui, &status_frame.response);
    ui.add_space(12.0);
    render_date_time(ui, theme);
}

fn render_user_status(app: &mut CircleApp, ui: &mut Ui, theme: &Theme) {
    let user_name = app.user.as_ref().map_or("Guest", |u| &u.name);
    let user_frame = StatusFrameProps::new(theme)
        .with_margin(Margin::symmetric(10, 4))
        .build()
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(create_styled_text(user_name, theme, 13.0, true));
                ui.add_space(4.0);
                render_status_dot(ui, theme, 6.0);
            });
        });

    set_hover_cursor(ui, &user_frame.response);
}

fn render_right_section(app: &mut CircleApp, ctx: &Context, ui: &mut Ui, theme: &Theme) {
    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
        ui.add_space(16.0);

        let notif_response = render_notification_bell(app, ui, theme);
        handle_notifications_panel(app, ctx, ui, theme, notif_response.rect);
        ui.add_space(12.0);
        render_user_status(app, ui, theme);
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
            let label = create_styled_text(format!("🔔 {}", unread_count), theme, 13.0, true);
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
