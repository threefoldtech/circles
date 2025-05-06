use crate::app::CircleApp;
use crate::models::circle::Role;
use crate::models::notification::{AppNotification, NotificationPriority};
use crate::utils::config::Theme;
use egui::{ComboBox, Context, RichText, TextEdit, Window};

use super::helpers::add_log_to_circles_bot;

/// Render the add member dialog
pub fn render_add_member_dialog(ctx: &Context, app: &mut CircleApp, theme: &Theme) {
    // Only proceed if the dialog should be shown
    if !app.add_member_state.open {
        return;
    }

    let circle_id = app.add_member_state.circle_id;
    let circle_name = app.add_member_state.circle_name.clone();
    
    // Validate email
    let is_valid_email = if !app.add_member_state.member_email.is_empty() {
        crate::ui::features::auth::is_valid_email(&app.add_member_state.member_email)
    } else {
        true // Empty is not invalid, just incomplete
    };

    // Create a simple dialog
    let mut result = None;

    Window::new("Add Member to Circle")
        .fixed_size([450.0, 380.0])
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
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
                .inner_margin(egui::Margin::same(24)),
        )
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.heading(RichText::new("Add Member to Circle").color(theme.accent));
                ui.add_space(20.0);

                // Member Email field
                ui.label(RichText::new("Member Email *").strong().size(16.0).color(theme.text));
                ui.add_space(4.0);
                
                // Set theme-appropriate background for all widgets
                ui.style_mut().visuals.extreme_bg_color = theme.secondary_background;
                ui.style_mut().visuals.widgets.inactive.bg_fill = theme.secondary_background;
                ui.style_mut().visuals.widgets.active.bg_fill = theme.hover;
                ui.style_mut().visuals.widgets.hovered.bg_fill = theme.hover;
                
                // Create text edit with consistent styling
                let email_response = ui.add(
                    TextEdit::singleline(&mut app.add_member_state.member_email)
                        .margin(egui::Vec2::new(10.0, 8.0))
                        .desired_width(f32::INFINITY)
                        .hint_text("Enter email address")
                        .font(egui::FontId::proportional(16.0)),
                );
                
                // Show email validation error if needed
                if !app.add_member_state.member_email.is_empty() && !is_valid_email {
                    ui.add_space(4.0);
                    ui.label(RichText::new("Please enter a valid email address").color(theme.error).size(14.0));
                }
                
                ui.add_space(15.0);

                // Member Permissions dropdown
                ui.label(RichText::new("Member Permissions *").strong().size(16.0).color(theme.text));
                ui.add_space(4.0);
                
                // Style the dropdown to match the theme
                ui.style_mut().visuals.widgets.inactive.weak_bg_fill = theme.secondary_background;
                ui.style_mut().visuals.widgets.active.weak_bg_fill = theme.hover;
                ui.style_mut().visuals.widgets.hovered.weak_bg_fill = theme.hover;
                
                // Simple dropdown for permissions
                ComboBox::new("member_permissions", "Select role")
                    .selected_text(format!("{:?}", app.add_member_state.selected_role))
                    .width(ui.available_width() * 0.9)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut app.add_member_state.selected_role, Role::Read, "Read – View-only access");
                        ui.selectable_value(&mut app.add_member_state.selected_role, Role::Write, "Write – Can read and modify content");
                        ui.selectable_value(&mut app.add_member_state.selected_role, Role::Administrator, "Administrator – Full control over content and membership");
                        ui.selectable_value(&mut app.add_member_state.selected_role, Role::Coordinator, "Coordinator – Similar to Administrator but focused on task coordination");
                    });
                
                ui.add_space(20.0);

                // // Roles overview section
                // ui.group(|ui| {
                //     ui.vertical_centered(|ui| {
                //         ui.label(RichText::new("Roles Overview").strong().color(theme.accent));
                //     });
                //     ui.add_space(5.0);
                //     ui.label("Each member of a circle has a role that defines their access:");
                //     ui.add_space(5.0);
                //     ui.label(RichText::new("• Read:").strong().color(theme.accent));
                //     ui.label("  View-only access");
                //     ui.label(RichText::new("• Write:").strong().color(theme.accent));
                //     ui.label("  Can read and modify content");
                //     ui.label(RichText::new("• Administrator:").strong().color(theme.accent));
                //     ui.label("  Full control over content and membership");
                //     ui.label(RichText::new("• Coordinator:").strong().color(theme.accent));
                //     ui.label("  Similar to administrator but focused on task coordination and calendar management");
                // });
                
                ui.add_space(20.0);

                // Buttons
                ui.horizontal(|ui| {
                    let close_response = egui::Button::new(RichText::new("Close").color(theme.dark_color)).fill(theme.secondary_background);
                    if ui.add(close_response).clicked() {
                        result = Some(false);
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let invite_response = ui.add_enabled(
                            !app.add_member_state.member_email.trim().is_empty(),
                            egui::Button::new(RichText::new("Invite").color(theme.white)).fill(theme.button_primary)
                        );
                        
                        if invite_response.clicked() {
                            result = Some(true);
                        }
                    });
                });
            });
        });

    // Handle the result
    if let Some(confirmed) = result {
        if confirmed {
            // User confirmed invitation
            if let Some(_id) = circle_id {
                // Here we would normally send an invitation email
                // For now, just log the action
                // Log the action to the circles bot
                add_log_to_circles_bot(
                    app,
                    format!(
                        "Invited {} to {} with {} permissions",
                        app.add_member_state.member_email,
                        circle_name,
                        format!("{:?}", app.add_member_state.selected_role)
                    ),
                );
                
                // Add an in-app notification
                let notification = AppNotification::new(
                    "Invitation Sent",
                    format!("An invitation email has been sent to {}", app.add_member_state.member_email),
                    NotificationPriority::Normal,
                );
                
                // Add the notification to the notification manager
                app.notification_manager.add(notification);
            }
        }
        // Reset the state in either case
        app.add_member_state = crate::models::features::AddMemberState::default();
    }
}
