use crate::app::CircleApp;
use crate::utils::config::Theme;
use eframe::egui::{
    self, Align, Color32, Context, Frame, Layout, RichText, Rounding, TopBottomPanel,
};

// Footer rendering function
pub fn render_status_bar(app: &CircleApp, ctx: &Context, app_layout: &Frame, theme: &Theme) {
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
