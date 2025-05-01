use crate::utils::config::Theme;
use egui::{Button, Context, Margin, RichText, Window};

/// Props for the confirmation modal
pub struct ConfirmationModalProps<'a> {
    /// Whether the modal is open
    pub open: bool,
    /// The title of the modal
    pub title: &'a str,
    /// The body text of the modal
    pub body: &'a str,
    /// The text for the confirm button
    pub confirm_button_text: &'a str,
    /// The text for the cancel button
    pub cancel_button_text: &'a str,
    /// Optional color accent for critical actions
    pub accent_color: Option<egui::Color32>,
}

/// Renders a confirmation modal with the given props
///
/// Returns a tuple of (confirm_clicked, cancel_clicked)
pub fn render_confirmation_modal<F, G>(
    ctx: &Context,
    props: ConfirmationModalProps,
    theme: &Theme,
    on_confirm: F,
    on_cancel: G,
) where
    F: FnOnce(),
    G: FnOnce(),
{
    if props.open {
        Window::new(props.title)
            .fixed_size([600.0, 620.0]) // Fixed size to match event dialog
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0]) // Centered horizontally
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
                    .inner_margin(Margin::same(24)), // Appropriate padding for content
            )
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);
                    // Use accent color for title if provided, otherwise use theme error color
                    let title_color = props.accent_color.unwrap_or(theme.error);
                    ui.heading(RichText::new(props.title).color(title_color));
                    ui.add_space(10.0);
                    ui.label(props.body);
                    ui.add_space(20.0);

                    ui.horizontal(|ui| {
                        let cancel_button =
                            Button::new(RichText::new(props.cancel_button_text).color(theme.text))
                                .fill(theme.accent);
                        if ui.add(cancel_button).clicked() {
                            on_cancel();
                        }

                        // Use accent color for confirm button if provided, otherwise use theme error color
                        let confirm_button_color = props.accent_color.unwrap_or(theme.error);
                        let confirm_button = Button::new(
                            RichText::new(props.confirm_button_text).color(theme.white),
                        )
                        .fill(confirm_button_color);

                        if ui.add(confirm_button).clicked() {
                            on_confirm();
                        }
                    });
                });
            });
    }
}
