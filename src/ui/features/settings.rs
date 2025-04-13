use crate::{
    ui::app_layout::{create_content_frame, render_header},
    utils::config::Theme,
};
use egui::{Button, Color32, RichText, Rounding, Stroke, Ui, Vec2};

use crate::app::CircleApp;

/// Renders the settings screen with a full-width settings card.
pub fn render_settings(_: &CircleApp, ui: &mut Ui) {
    render_header(ui, "", "Settings");
    ui.add_space(16.0);

    // Create a full-width frame for the settings card.
    let mut frame = create_content_frame();
    frame.inner_margin = egui::Margin::same(16.0); // Ensure consistent padding.

    frame.show(ui, |ui| {
        // Ensure the frame takes the full available width.
        ui.set_min_width(ui.available_width());

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

            // Center the Save Settings button.
            ui.horizontal(|ui| {
                ui.add_space(ui.available_width() - 120.0 - 16.0); // Adjust for button width and padding.
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
    });
}

/// Renders a settings section with checkboxes, ensuring it takes the full width.
pub fn render_settings_section(ui: &mut Ui, title: &str, settings: &[(bool, &str)]) {
    let theme = Theme::new();

    // Ensure the section takes the full width.
    ui.set_min_width(ui.available_width());

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
