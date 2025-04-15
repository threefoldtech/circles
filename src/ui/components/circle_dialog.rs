use crate::models::circle::{Circle, CircleType, JoinPolicy, NotificationSettings, Visibility};
use eframe::egui::{self, Color32, RichText, Stroke, Vec2};

/// State for the circle creation dialog
#[derive(Debug)]
pub struct CircleDialogState {
    pub is_open: bool,
    pub name: String,
    pub circle_type: CircleType,
    pub visibility: Visibility,
    pub join_policy: JoinPolicy,
    pub email_notifications: bool,
    pub push_notifications: bool,
    pub in_app_notifications: bool,
    pub error_message: Option<String>,
}

impl CircleDialogState {
    /// Create a new dialog state
    pub fn new() -> Self {
        Self {
            is_open: false,
            name: String::new(),
            circle_type: CircleType::Personal,
            visibility: Visibility::Private,
            join_policy: JoinPolicy::InviteOnly,
            email_notifications: true,
            push_notifications: true,
            in_app_notifications: true,
            error_message: None,
        }
    }

    /// Reset the dialog state
    pub fn reset(&mut self) {
        self.name = String::new();
        self.circle_type = CircleType::Personal;
        self.visibility = Visibility::Private;
        self.join_policy = JoinPolicy::InviteOnly;
        self.email_notifications = true;
        self.push_notifications = true;
        self.in_app_notifications = true;
        self.error_message = None;
    }

    /// Validate the dialog inputs
    pub fn validate(&mut self) -> bool {
        if self.name.trim().is_empty() {
            self.error_message = Some("Circle name cannot be empty".to_string());
            return false;
        }
        true
    }
}

/// Render the circle creation dialog
pub fn render_circle_dialog(
    state: &mut CircleDialogState,
    ctx: &egui::Context,
    creator_id: uuid::Uuid,
) -> Option<Circle> {
    if !state.is_open {
        return None;
    }

    let mut created_circle = None;
    let mut should_close = false;

    // Create a modal dialog with increased width and height
    egui::Window::new("Create New Circle")
        .fixed_size([520.0, 620.0]) // Increased width and height for better content display
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0]) // Point 2: Already centered
        .collapsible(false)
        .resizable(false)
        .frame(
            egui::Frame::window(&ctx.style())
                .fill(Color32::from_rgb(255, 255, 255))
                .corner_radius(12)
                .shadow(egui::epaint::Shadow {
                    color: Color32::from_black_alpha(25),
                    offset: [0, 4],
                    blur: 8,
                    spread: 0,
                })
                .inner_margin(egui::Margin::same(24)), // Further increased padding for better spacing
        )
        .show(ctx, |ui| {
            // Main layout
            ui.vertical(|ui| {
                // Heading
                ui.vertical_centered(|ui| {
                    ui.heading(
                        RichText::new("Create New Circle")
                            .size(24.0) // Optimized heading font size
                            .strong()
                            .color(Color32::from_rgb(40, 50, 60)),
                    );
                    ui.add_space(20.0);
                });

                // Form content (no scroll, increased height instead)
                // Point 1: Form takes full width by default with adjusted margins
                // Circle name
                ui.label(
                    RichText::new("Circle Name *")
                        .strong()
                        .size(16.0) // Optimized label font size
                        .color(Color32::from_rgb(40, 50, 60)),
                );
                // Set light background for all widgets before adding text field
                ui.style_mut().visuals.extreme_bg_color = Color32::from_rgb(248, 248, 248);
                ui.style_mut().visuals.widgets.inactive.bg_fill = Color32::from_rgb(248, 248, 248);
                ui.style_mut().visuals.widgets.active.bg_fill = Color32::from_rgb(240, 240, 240);
                ui.style_mut().visuals.widgets.hovered.bg_fill = Color32::from_rgb(235, 235, 235);

                // Create text edit with consistent styling
                ui.add(
                    egui::TextEdit::singleline(&mut state.name)
                        .margin(egui::Vec2::new(10.0, 8.0))
                        .desired_width(f32::INFINITY)
                        .font(egui::FontId::proportional(16.0)), // Removed trailing comma
                )
                .on_hover_text(
                    egui::RichText::new("Enter a name for your circle").color(Color32::WHITE),
                )
                .on_hover_cursor(egui::CursorIcon::Text);

                // Add some space after the text field
                ui.add_space(4.0);

                // Error message directly below the Circle Name field
                if let Some(error) = &state.error_message {
                    ui.label(
                        RichText::new(error)
                            .color(Color32::from_rgb(200, 40, 40))
                            .size(14.0), // Optimized error message font size
                    );
                    ui.add_space(8.0);
                } else {
                    ui.add_space(16.0);
                }

                // Circle type
                ui.label(
                    RichText::new("Circle Type *")
                        .strong()
                        .size(16.0) // Optimized label font size
                        .color(Color32::from_rgb(40, 50, 60)),
                );
                ui.horizontal(|ui| {
                    ui.style_mut().spacing.item_spacing = Vec2::new(12.0, 0.0);
                    ui.radio_value(&mut state.circle_type, CircleType::Personal, "Personal")
                        .on_hover_cursor(egui::CursorIcon::PointingHand);
                    ui.radio_value(&mut state.circle_type, CircleType::Team, "Team")
                        .on_hover_cursor(egui::CursorIcon::PointingHand);
                    ui.radio_value(&mut state.circle_type, CircleType::Private, "Private")
                        .on_hover_cursor(egui::CursorIcon::PointingHand);
                });
                ui.add_space(16.0);

                // Visibility
                ui.label(
                    RichText::new("Visibility")
                        .strong()
                        .size(16.0) // Optimized label font size
                        .color(Color32::from_rgb(40, 50, 60)),
                );
                ui.horizontal(|ui| {
                    ui.style_mut().spacing.item_spacing = Vec2::new(12.0, 0.0);
                    ui.radio_value(&mut state.visibility, Visibility::Public, "Public")
                        .on_hover_cursor(egui::CursorIcon::PointingHand);
                    ui.radio_value(&mut state.visibility, Visibility::Private, "Private")
                        .on_hover_cursor(egui::CursorIcon::PointingHand);
                    ui.radio_value(&mut state.visibility, Visibility::Secret, "Secret")
                        .on_hover_cursor(egui::CursorIcon::PointingHand);
                });
                ui.add_space(16.0);

                // Join policy
                ui.label(
                    RichText::new("Join Policy")
                        .strong()
                        .size(16.0) // Optimized label font size
                        .color(Color32::from_rgb(40, 50, 60)),
                );
                ui.horizontal(|ui| {
                    ui.style_mut().spacing.item_spacing = Vec2::new(12.0, 0.0);
                    ui.radio_value(&mut state.join_policy, JoinPolicy::Open, "Open")
                        .on_hover_cursor(egui::CursorIcon::PointingHand);
                    ui.radio_value(
                        &mut state.join_policy,
                        JoinPolicy::ApprovalRequired,
                        "Approval Required",
                    )
                    .on_hover_cursor(egui::CursorIcon::PointingHand);
                    ui.radio_value(
                        &mut state.join_policy,
                        JoinPolicy::InviteOnly,
                        "Invite Only",
                    )
                    .on_hover_cursor(egui::CursorIcon::PointingHand);
                });
                ui.add_space(16.0);

                // Notification settings
                ui.label(
                    RichText::new("Notification Settings")
                        .strong()
                        .size(16.0) // Optimized label font size
                        .color(Color32::from_rgb(40, 50, 60)),
                );
                ui.checkbox(&mut state.email_notifications, "Email Notifications")
                    .on_hover_cursor(egui::CursorIcon::PointingHand);
                ui.checkbox(&mut state.push_notifications, "Push Notifications")
                    .on_hover_cursor(egui::CursorIcon::PointingHand);
                ui.checkbox(&mut state.in_app_notifications, "In-App Notifications")
                    .on_hover_cursor(egui::CursorIcon::PointingHand);
                ui.add_space(28.0); // Increased spacing before footer

                // Fixed footer with buttons
                ui.separator();
                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    // Cancel button
                    let cancel_button = egui::Button::new(
                        RichText::new("Cancel")
                            .size(16.0) // Optimized button text font size
                            .color(Color32::from_rgb(70, 80, 90)),
                    )
                    .min_size(Vec2::new(110.0, 38.0)) // Slightly larger button
                    .corner_radius(8.0)
                    .fill(Color32::from_rgb(230, 235, 240))
                    .stroke(Stroke::new(1.0, Color32::from_rgb(200, 210, 220)));

                    if ui
                        .add(cancel_button)
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        should_close = true;
                    }

                    ui.allocate_space(ui.available_size_before_wrap());

                    // Create button
                    let create_button = egui::Button::new(
                        RichText::new("Create Circle")
                            .size(16.0) // Optimized button text font size
                            .color(Color32::WHITE),
                    )
                    .min_size(Vec2::new(130.0, 38.0)) // Slightly larger button
                    .corner_radius(8.0)
                    .fill(Color32::from_rgb(66, 133, 244))
                    .stroke(Stroke::new(1.0, Color32::from_rgb(45, 100, 200)));

                    if ui
                        .add(create_button)
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        if state.validate() {
                            let mut circle =
                                Circle::new(state.name.clone(), state.circle_type, creator_id);

                            circle.settings.visibility = state.visibility;
                            circle.settings.join_policy = state.join_policy;
                            circle.settings.notification_settings = NotificationSettings {
                                email_notifications: state.email_notifications,
                                push_notifications: state.push_notifications,
                                in_app_notifications: state.in_app_notifications,
                            };

                            created_circle = Some(circle);
                            should_close = true;
                        }
                    }
                });
                ui.add_space(16.0); // Increased bottom spacing
            });
        });

    if should_close {
        state.is_open = false;
        state.reset();
    }

    created_circle
}
