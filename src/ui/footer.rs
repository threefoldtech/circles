use crate::app::CircleApp;
use crate::ui::components::notifications_panel::{
    NotificationsPanelProps, render_notifications_panel,
};
use crate::utils::config::Theme;
use chrono::Local;
use eframe::egui::{
    Align, Color32, Context, Frame, Layout, Margin, Pos2, RichText, Sense, Stroke, TopBottomPanel,
    Vec2,
};

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

fn render_left_section(ui: &mut egui::Ui, _: &Theme) {
    // Left side - Connection status
    ui.add_space(16.0);
    let _status_frame = Frame::new()
        .fill(Color32::from_rgb(240, 245, 250))
        .corner_radius(12)
        .inner_margin(Margin::symmetric(10, 4))
        .stroke(Stroke::new(1.0, Color32::from_rgb(220, 225, 230)))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                // Green dot for connected status
                ui.painter().circle_filled(
                    ui.min_rect().left_center() + Vec2::new(6.0, 0.0),
                    4.0,
                    Color32::from_rgb(50, 180, 50),
                );
                ui.add_space(12.0);
                ui.label(
                    RichText::new("Connected")
                        .size(13.0)
                        .color(Color32::from_rgb(70, 80, 90)),
                );
            });
        });

    ui.add_space(12.0);

    // Current date and time
    let now = Local::now();
    let date_str = now.format("%a, %d %b %Y").to_string();
    let time_str = now.format("%H:%M:%S").to_string();

    ui.label(
        RichText::new(format!("📅 {}", date_str))
            .size(13.0)
            .color(Color32::from_rgb(70, 80, 90)),
    );

    ui.add_space(8.0);

    ui.label(
        RichText::new(format!("🕒 {}", time_str))
            .size(13.0)
            .color(Color32::from_rgb(70, 80, 90)),
    );
}

fn render_right_section(app: &mut CircleApp, ctx: &Context, ui: &mut egui::Ui, theme: &Theme) {
    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
        ui.add_space(16.0);

        // Notifications indicator
        let notif_frame = render_notification_bell(app, ui, theme);

        // Render notifications panel if shown
        if app.notification_manager.show_panel {
            let screen_rect = ui.ctx().screen_rect();
            let panel_width = 320.0;
            let panel_height = 600.0;

            let panel_pos = Pos2::new(
                screen_rect.right() - panel_width - 20.0,
                screen_rect.bottom() - panel_height - 50.0,
            );

            let panel_rect = render_notifications_panel(
                app,
                ctx,
                NotificationsPanelProps {
                    pos: panel_pos,
                    width: panel_width,
                    height: panel_height,
                    theme,
                },
            );

            // Handle outside clicks
            if ui.input(|i| i.pointer.any_released()) {
                let mouse_pos = ui.input(|i| i.pointer.interact_pos());
                if let Some(pos) = mouse_pos {
                    let bell_rect = notif_frame.rect.expand(20.0);
                    if !panel_rect.contains(pos) && !bell_rect.contains(pos) {
                        app.notification_manager.show_panel = false;
                    }
                }
            }
        }

        ui.add_space(12.0);

        render_user_status(app, ui, theme);
    });
}

fn render_notification_bell(app: &mut CircleApp, ui: &mut egui::Ui, _: &Theme) -> egui::Response {
    let unread_count = app.notification_manager.unread_count();
    let notif_frame = Frame::new()
        .fill(if unread_count > 0 {
            Color32::from_rgb(240, 70, 70)
        } else {
            Color32::from_rgb(200, 210, 220)
        })
        .corner_radius(10)
        .inner_margin(Margin::symmetric(8, 4));

    let response = notif_frame
        .show(ui, |ui| {
            let response = ui.add(
                egui::Label::new(
                    RichText::new(format!("🔔 {}", unread_count))
                        .size(13.0)
                        .color(Color32::WHITE)
                        .strong(),
                )
                .sense(Sense::click()),
            );

            if response.clicked() {
                app.notification_manager.show_panel = !app.notification_manager.show_panel;
            }

            if response.hovered() {
                ui.output_mut(|o| o.cursor_icon = eframe::egui::CursorIcon::PointingHand);
            }

            response
        })
        .inner;

    response
}

fn render_user_status(app: &mut CircleApp, ui: &mut egui::Ui, theme: &Theme) {
    let user_name = app.user.as_ref().map_or("Guest", |u| &u.name);
    let user_frame = Frame::new()
        .fill(theme.hover)
        .corner_radius(12)
        .inner_margin(Margin::symmetric(10, 4))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.painter().circle_filled(
                    ui.min_rect().left_center() + Vec2::new(6.0, 0.0),
                    4.0,
                    theme.success,
                );
                ui.add_space(12.0);
                ui.label(
                    RichText::new(format!("{}", user_name))
                        .size(13.0)
                        .color(theme.text)
                        .strong(),
                );
            });
        });

    if user_frame.response.hovered() {
        ui.output_mut(|o| o.cursor_icon = eframe::egui::CursorIcon::PointingHand);
    }
}
