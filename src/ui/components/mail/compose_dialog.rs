use egui::TextStyle;

use crate::{app::CircleApp, models::features::ComposeDraft, utils::config::Theme};
use egui_extras::{Size, StripBuilder};

/// Open the compose dialog
pub fn open_compose_dialog(app: &mut CircleApp, _: &mut egui::Ui) {
    app.compose_dialog_open = true;
    app.compose_draft = Some(ComposeDraft {
        to: String::new(),
        subject: String::new(),
        body: String::new(),
        attachments: Vec::new(),
    });
}

/// Render the compose dialog
/// Includes the compose form
pub fn render_compose_dialog(ui: &mut egui::Ui, _: &mut CircleApp, _: &Theme) {
    let body_text_size = TextStyle::Body.resolve(ui.style()).size;
    StripBuilder::new(ui)
        .size(Size::exact(50.0))
        .size(Size::remainder())
        .size(Size::relative(0.5).at_least(60.0))
        .size(Size::exact(body_text_size))
        .vertical(|mut strip| {
            strip.cell(|ui| {
                ui.painter();
                ui.label("width: 100%\nheight: 50px");
            });
            strip.strip(|builder| {
                builder.sizes(Size::remainder(), 2).horizontal(|mut strip| {
                    strip.cell(|ui| {
                        ui.painter();
                        ui.label("width: 50%\nheight: remaining");
                    });
                    strip.strip(|builder| {
                        builder.sizes(Size::remainder(), 3).vertical(|mut strip| {
                            strip.empty();
                            strip.cell(|ui| {
                                ui.painter();
                                ui.label("width: 50%\nheight: 1/3 of the red region");
                            });
                            strip.empty();
                        });
                    });
                });
            });
            strip.strip(|builder| {
                builder
                    .size(Size::remainder())
                    .size(Size::exact(120.0))
                    .size(Size::remainder())
                    .size(Size::exact(70.0))
                    .horizontal(|mut strip| {
                        strip.empty();
                        strip.strip(|builder| {
                            builder
                                .size(Size::remainder())
                                .size(Size::exact(60.0))
                                .size(Size::remainder())
                                .vertical(|mut strip| {
                                    strip.empty();
                                    strip.cell(|ui| {
                                        ui.painter();
                                        ui.label("width: 120px\nheight: 60px");
                                    });
                                });
                        });
                        strip.empty();
                        strip.cell(|ui| {
                            ui.painter();
                            ui.label("width: 70px\n\nheight: 50%, but at least 60px.");
                        });
                    });
            });
            // strip.cell(|ui| {
            //     ui.vertical_centered(|ui| {
            //         ui.add(crate::egui_github_link_file!());
            //     });
            // });
        });
}
