use egui::{Button, Color32, RichText, Stroke, Vec2};

pub fn create_button<'a>(text: &'a str, icon: &'a str) -> Button<'a> {
    Button::new(
        RichText::new(format!("{} {}", icon, text))
            .size(14.0)
            .color(Color32::WHITE),
    )
    .corner_radius(20)
    .fill(Color32::from_rgb(66, 133, 244))
    .stroke(Stroke::NONE)
    .min_size(Vec2::new(100.0, 36.0))
}
