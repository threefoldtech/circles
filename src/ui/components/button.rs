use crate::utils::config::Theme;
use egui::{Button, Color32, Response, RichText, Stroke, Ui, Vec2};

/// Renders a button with consistent styling across the app
///
/// # Arguments
/// * `ui` - The UI to add the button to
/// * `label` - The text to display on the button
/// * `is_active` - Whether the button is in active state
/// * `theme` - The current theme
/// * `icon` - Optional icon to display before the text
///
/// # Returns
/// The button response which can be used to check if it was clicked
pub fn render_button(
    ui: &mut Ui,
    label: &str,
    is_active: bool,
    theme: &Theme,
    icon: Option<&str>,
) -> Response {
    let text_content = if let Some(icon_str) = icon {
        format!("{} {}", icon_str, label)
    } else {
        label.to_string()
    };

    let text = RichText::new(text_content).size(14.0).color(if is_active {
        Color32::WHITE
    } else {
        theme.text
    });

    let background = if is_active {
        theme.accent
    } else {
        theme.secondary_background
    };

    ui.add(
        Button::new(text)
            .min_size(Vec2::new(110.0, 40.0))
            .corner_radius(6.0)
            .fill(background)
            .stroke(Stroke::NONE),
    )
}

/// Legacy function for backward compatibility
/// Consider using render_button instead for new code
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
