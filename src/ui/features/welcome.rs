// File: src/ui/features/welcome.rs
use crate::app::CircleApp;
use eframe::egui::{
    Button, Color32, CursorIcon, Frame, Margin, RichText, Rounding, Stroke, Ui, Vec2,
};

// Renders the welcome screen for the Circle Collaboration System
pub fn render_welcome_screen(_: &CircleApp, ui: &mut Ui) {
    let available_height = ui.available_height();
    let welcome_frame = Frame::none()
        .fill(Color32::WHITE)
        .inner_margin(Margin::same(0.0))
        .outer_margin(Margin::same(0.0));

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

            const INSTRUCTIONS: [&str; 4] = [
                "• Check out the 'Welcome to Circles' guide in the OTHERS section",
                "• Explore the different features using the navigation bar above",
                "• Create your own circles using the + button in the sidebar",
                "• Stay updated with the Circles Bot Channel in the OTHERS section",
            ];

            for instruction in INSTRUCTIONS {
                ui.label(
                    RichText::new(instruction)
                        .size(16.0)
                        .color(Color32::from_rgb(70, 80, 90)),
                );
                ui.add_space(8.0);
            }

            ui.add_space(40.0);
            let get_started_button = Button::new(
                RichText::new("Get Started")
                    .size(18.0)
                    .color(Color32::WHITE),
            )
            .min_size(Vec2::new(180.0, 50.0))
            .rounding(Rounding::same(8.0))
            .fill(Color32::from_rgb(66, 133, 244))
            .stroke(Stroke::new(1.0, Color32::from_rgb(45, 100, 200)));

            ui.add(get_started_button)
                .on_hover_cursor(CursorIcon::PointingHand);
            ui.add_space(available_height * 0.15);
        });
    });
}
