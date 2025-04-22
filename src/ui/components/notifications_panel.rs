use super::button::create_button;
use crate::{app::CircleApp, utils::config::Theme};
use chrono::{DateTime, Local, Utc};
use eframe::egui::{self, Align, Color32, Frame, Layout, Order, Pos2, Rect, RichText, Vec2};
use egui::{Area, Id, Margin, Stroke};

pub struct NotificationsPanelProps<'a> {
    pub pos: Pos2,
    pub width: f32,
    pub height: f32,
    pub theme: &'a Theme,
}

pub fn render_notifications_panel(
    app: &mut CircleApp,
    ctx: &egui::Context,
    props: NotificationsPanelProps,
) -> Rect {
    // Glass-like overlay
    let overlay_rect = ctx.screen_rect();
    Area::new(Id::new("notification_overlay"))
        .order(Order::Middle)
        .show(ctx, |ui| {
            ui.painter()
                .rect_filled(overlay_rect, 0.0, Color32::from_black_alpha(90));
        });

    let panel_rect = Rect::from_min_size(props.pos, Vec2::new(props.width, props.height));

    Area::new(Id::new("notification_panel"))
        .order(Order::Foreground)
        .fixed_pos(props.pos)
        .show(ctx, |ui| {
            Frame::popup(ui.style())
                .fill(props.theme.panel)
                .stroke(Stroke::new(1.0, props.theme.border))
                .inner_margin(Margin::same(16))
                .show(ui, |ui| {
                    ui.set_min_size(Vec2::new(props.width, props.height));
                    ui.set_max_size(Vec2::new(props.width, props.height));
                    ui.vertical(|ui| {
                        ui.heading(
                            RichText::new("Notifications")
                                .size(18.0)
                                .strong()
                                .color(props.theme.header_text),
                        );

                        ui.add_space(16.0);

                        app.notification_manager.render(ui, props.theme);
                    });
                });
        });

    panel_rect
}
