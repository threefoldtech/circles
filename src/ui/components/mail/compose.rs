use egui::{FontFamily, FontId, Margin, RichText, Stroke, Vec2};
use rfd::FileDialog;
use std::path::PathBuf;

use crate::{
    app::CircleApp,
    models::notification::{AppNotification, NotificationPriority},
    models::{
        dummy_data::Email,
        features::{ComposeDraft, ComposeDraftMode},
    },
    ui::components::button::render_button,
    utils::config::Theme,
    utils::notifications::{NotificationConfig, NotificationType, send_desktop_notification},
};

/// Create a new compose draft for replying to an email
pub fn create_reply_draft(email: &Email) -> ComposeDraft {
    let subject = if !email.subject.to_lowercase().starts_with("re:") {
        format!("Re: {}", email.subject)
    } else {
        email.subject.clone()
    };

    let draft = ComposeDraft {
        to: email.sender.clone(),
        subject,
        body: String::new(),
        attachments: Vec::new(),
        reply_to: Some(email.sender.clone()),
        forward_from: None,
        original_subject: Some(email.subject.clone()),
        original_date: Some(email.timestamp.format("%Y-%m-%d %H:%M:%S").to_string()),
        original_content: Some(email.content.clone()),
        mode: ComposeDraftMode::Reply,
    };

    let config = NotificationConfig::new(
        "Reply Draft Created",
        format!("Started composing reply to \"{}\"", email.subject),
        NotificationType::Info,
    )
    .with_timeout(30000);
    if let Err(e) = send_desktop_notification(&config) {
        eprintln!("Failed to send desktop notification: {}", e);
    }

    draft
}

/// Create a new compose draft for forwarding an email
pub fn create_forward_draft(email: &Email) -> ComposeDraft {
    let subject = if !email.subject.to_lowercase().starts_with("fwd:") {
        format!("Fwd: {}", email.subject)
    } else {
        email.subject.clone()
    };

    let draft = ComposeDraft {
        to: String::new(),
        subject,
        body: String::new(),
        attachments: Vec::new(),
        reply_to: None,
        forward_from: Some(email.sender.clone()),
        original_subject: Some(email.subject.clone()),
        original_date: Some(email.timestamp.format("%Y-%m-%d %H:%M:%S").to_string()),
        original_content: Some(email.content.clone()),
        mode: ComposeDraftMode::Forward,
    };

    let config = NotificationConfig::new(
        "Forward Draft Created",
        format!("Started forwarding \"{}\"", email.subject),
        NotificationType::Info,
    )
    .with_timeout(30000);
    if let Err(e) = send_desktop_notification(&config) {
        eprintln!("Failed to send desktop notification: {}", e);
    }

    draft
}

/// Open the compose screen
pub fn open_compose_screen(app: &mut CircleApp, _: &mut egui::Ui) {
    app.compose_dialog_open = true;
    app.compose_draft = Some(ComposeDraft {
        to: String::new(),
        subject: String::new(),
        body: String::new(),
        attachments: Vec::new(),
        reply_to: None,
        forward_from: None,
        original_subject: None,
        original_date: None,
        original_content: None,
        mode: ComposeDraftMode::New,
    });
}

/// Represents a file attachment
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Attachment {
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
}

impl ComposeDraft {
    fn add_attachment(&mut self, path: PathBuf) {
        if let Ok(metadata) = std::fs::metadata(&path) {
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();

            self.attachments.push(Attachment {
                name,
                path,
                size: metadata.len(),
            });
        }
    }

    fn remove_attachment(&mut self, index: usize) {
        if index < self.attachments.len() {
            self.attachments.remove(index);
        }
    }
}

/// Format file size in human-readable format
fn format_file_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if size >= GB {
        format!("{:.1} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.1} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.1} KB", size as f64 / KB as f64)
    } else {
        format!("{} B", size)
    }
}

/// Render attachments section
fn render_attachments(ui: &mut egui::Ui, draft: &mut ComposeDraft, theme: &Theme) {
    if !draft.attachments.is_empty() {
        ui.add_space(16.0);

        let attachment_frame = egui::Frame::new()
            .fill(theme.hover.gamma_multiply(0.5))
            .inner_margin(Margin::same(12))
            .corner_radius(8.0);

        attachment_frame.show(ui, |ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new("Attachments").size(14.0).strong());
                ui.add_space(8.0);

                let mut to_remove = None;

                for (idx, attachment) in draft.attachments.iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("📎").size(16.0));
                        ui.add_space(4.0);
                        ui.label(RichText::new(&attachment.name).size(14.0));

                        ui.add_space(8.0);
                        ui.label(
                            RichText::new(format_file_size(attachment.size))
                                .size(12.0)
                                .color(theme.secondary_text),
                        );

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui
                                .add(
                                    egui::Button::new(
                                        RichText::new("✖").size(14.0).color(theme.error),
                                    )
                                    .frame(false),
                                )
                                .clicked()
                            {
                                to_remove = Some(idx);
                            }
                        });
                    });

                    if idx < draft.attachments.len() - 1 {
                        ui.add_space(8.0);
                        ui.separator();
                        ui.add_space(8.0);
                    }
                }

                if let Some(idx) = to_remove {
                    draft.remove_attachment(idx);
                }
            });
        });
    }
}

/// Handle attachment button click
fn handle_attachment_click(draft: &mut ComposeDraft) {
    if let Some(files) = FileDialog::new()
        .set_title("Choose files to attach")
        .pick_files()
    {
        for path in files {
            draft.add_attachment(path);
        }
    }
}

/// Handle email sending and show notifications
fn handle_email_send(app: &mut CircleApp, draft: &ComposeDraft) {
    let (title, _) = match draft.mode {
        ComposeDraftMode::New => (
            "Email Sent",
            format!("Email \"{}\" sent to {}", draft.subject, draft.to),
        ),
        ComposeDraftMode::Reply => (
            "Reply Sent",
            format!("Reply \"{}\" sent to {}", draft.subject, draft.to),
        ),
        ComposeDraftMode::Forward => (
            "Email Forwarded",
            format!("Email \"{}\" forwarded to {}", draft.subject, draft.to),
        ),
    };

    let attachment_info = if !draft.attachments.is_empty() {
        format!(" with {} attachments", draft.attachments.len())
    } else {
        String::new()
    };

    let notification_message = match draft.mode {
        ComposeDraftMode::New => format!(
            "Email \"{}\" sent to {}{}",
            draft.subject, draft.to, attachment_info
        ),
        ComposeDraftMode::Reply => format!(
            "Replied to {} - \"{}\"{}",
            draft.to, draft.subject, attachment_info
        ),
        ComposeDraftMode::Forward => format!(
            "Forwarded \"{}\" to {}{}",
            draft.subject, draft.to, attachment_info
        ),
    };

    // Send desktop notification with longer timeout and higher urgency
    let config = NotificationConfig::new(title, &notification_message, NotificationType::Success)
        .with_timeout(60000); // 60 seconds for better visibility

    match send_desktop_notification(&config) {
        Ok(_) => println!(
            "Desktop notification sent successfully for email: {}",
            draft.subject
        ),
        Err(e) => {
            eprintln!("Desktop notification error: {:?}", e);
            // Fallback to just in-app notification
            app.notification_manager.add(AppNotification::new(
                "Notification Error",
                &format!("Could not show desktop notification: {}", e),
                NotificationPriority::High,
            ));
        }
    }

    // Keep the in-app notification
    app.notification_manager.add(AppNotification::new(
        title,
        notification_message,
        NotificationPriority::Normal,
    ));

    // Close the compose dialog after sending
    app.compose_dialog_open = false;
    app.compose_draft = None;
}

/// Render the compose screen
pub fn render_compose_screen(ui: &mut egui::Ui, app: &mut CircleApp, theme: &Theme) {
    ui.vertical(|ui| {
        // Header
        ui.horizontal(|ui| {
            let title = match app.compose_draft.as_ref().map(|d| d.mode.clone()) {
                Some(ComposeDraftMode::Reply) => "Reply to email",
                Some(ComposeDraftMode::Forward) => "Forward email",
                _ => "Draft a new email",
            };
            ui.heading(
                RichText::new(title)
                    .size(20.0)
                    .color(theme.header_text)
                    .strong(),
            );
        });
        ui.add_space(20.0);

        // Handle draft rendering
        let mut action = None;
        if let Some(draft) = &mut app.compose_draft {
            // Original email quote for replies and forwards
            if draft.mode != ComposeDraftMode::New {
                let quote_frame = egui::Frame::new()
                    .fill(theme.hover)
                    .inner_margin(Margin::same(12))
                    .corner_radius(4.0);

                quote_frame.show(ui, |ui| {
                    ui.vertical(|ui| {
                        if let (Some(date), Some(from)) = (
                            &draft.original_date,
                            if draft.mode == ComposeDraftMode::Reply {
                                &draft.reply_to
                            } else {
                                &draft.forward_from
                            },
                        ) {
                            ui.label(
                                RichText::new(format!("On {} {} wrote:", date, from))
                                    .color(theme.secondary_text)
                                    .size(12.0),
                            );
                            ui.add_space(8.0);
                        }

                        if let Some(content) = &draft.original_content {
                            ui.label(
                                RichText::new(content)
                                    .color(theme.secondary_text)
                                    .size(12.0),
                            );
                        }
                    });
                });
                ui.add_space(16.0);
            }

            // Common input frame style
            let input_frame = egui::Frame::new()
                .fill(theme.hover)
                .inner_margin(Margin::same(10))
                .corner_radius(20.0)
                .stroke(Stroke::NONE);

            // Common style override for transparent backgrounds
            let original_style = ui.style().clone();
            ui.style_mut().visuals.widgets.inactive.bg_fill = theme.transparent;
            ui.style_mut().visuals.widgets.active.bg_fill = theme.transparent;
            ui.style_mut().visuals.widgets.hovered.bg_fill = theme.transparent;

            // To field
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    // Create a fixed-width area for the label
                    ui.allocate_ui_with_layout(
                        Vec2::new(80.0, ui.available_height()),
                        egui::Layout::right_to_left(egui::Align::Center),
                        |ui| {
                            ui.add(egui::Label::new(
                                RichText::new("To:").size(14.0).color(theme.text).strong(),
                            ))
                            .on_hover_text("Email recipient");
                        },
                    );

                    input_frame.show(ui, |ui| {
                        ui.set_max_width(500.0);
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("👤").size(16.0).color(theme.icon_fg));
                            ui.add_space(8.0);
                            ui.add(
                                egui::TextEdit::singleline(&mut draft.to)
                                    .hint_text(
                                        RichText::new("Enter recipient...")
                                            .color(theme.placeholder_text),
                                    )
                                    .text_color(theme.text)
                                    .frame(false)
                                    .margin(Vec2::new(0.0, 0.0))
                                    .desired_width(400.0)
                                    .font(FontId::new(14.0, FontFamily::Proportional)),
                            );
                        });
                    });
                });
            });
            ui.add_space(12.0);

            // Subject field
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    // Create a fixed-width area for the label
                    ui.allocate_ui_with_layout(
                        Vec2::new(80.0, ui.available_height()),
                        egui::Layout::right_to_left(egui::Align::Center),
                        |ui| {
                            ui.add(egui::Label::new(
                                RichText::new("Subject:")
                                    .size(14.0)
                                    .color(theme.text)
                                    .strong(),
                            ))
                            .on_hover_text("Email subject");
                        },
                    );

                    input_frame.show(ui, |ui| {
                        ui.set_max_width(500.0);
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("📜").size(16.0).color(theme.icon_fg));
                            ui.add_space(8.0);
                            ui.add(
                                egui::TextEdit::singleline(&mut draft.subject)
                                    .hint_text(
                                        RichText::new("Enter subject...")
                                            .color(theme.placeholder_text),
                                    )
                                    .text_color(theme.text)
                                    .frame(false)
                                    .margin(Vec2::new(0.0, 0.0))
                                    .desired_width(400.0)
                                    .font(FontId::new(14.0, FontFamily::Proportional)),
                            );
                        });
                    });
                });
            });
            ui.add_space(12.0);

            // Message body
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    // Create a fixed-width area for the label
                    ui.allocate_ui_with_layout(
                        Vec2::new(80.0, ui.available_height()),
                        egui::Layout::right_to_left(egui::Align::Center),
                        |ui| {
                            ui.add(egui::Label::new(
                                RichText::new("Body:").size(14.0).color(theme.text).strong(),
                            ))
                            .on_hover_text("Email content");
                        },
                    );

                    input_frame.show(ui, |ui| {
                        ui.set_max_width(500.0);
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("📝").size(16.0).color(theme.icon_fg));
                            ui.add_space(8.0);
                            ui.add(
                                egui::TextEdit::multiline(&mut draft.body)
                                    .hint_text(
                                        RichText::new("Enter message...")
                                            .color(theme.placeholder_text),
                                    )
                                    .text_color(theme.text)
                                    .frame(false)
                                    .margin(Vec2::new(0.0, 0.0))
                                    .desired_width(400.0)
                                    .desired_rows(15)
                                    .font(FontId::new(14.0, FontFamily::Proportional)),
                            );
                        });
                    });
                });
            });
            ui.add_space(20.0);

            // Restore original style after input fields
            ui.set_style(original_style);

            // Render attachments
            render_attachments(ui, draft, theme);

            // Bottom buttons
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    // Custom styling for attach button (not using standard active/inactive states)
                    let attach_text = RichText::new("📎 Attach").size(14.0).color(theme.accent);
                    if ui
                        .add(
                            egui::Button::new(attach_text)
                                .fill(theme.background)
                                .stroke(Stroke::new(1.0, theme.accent))
                                .min_size(Vec2::new(110.0, 40.0))
                                .corner_radius(6.0),
                        )
                        .clicked()
                    {
                        handle_attachment_click(draft);
                    }

                    if !draft.attachments.is_empty() {
                        ui.add_space(8.0);
                        ui.label(
                            RichText::new(format!("{} attached", draft.attachments.len()))
                                .size(14.0)
                                .color(theme.secondary_text),
                        );
                    }
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if render_button(ui, "Send", true, theme, None).clicked() {
                        action = Some(("send", draft.clone()));
                    }

                    // Custom styling for discard button (not using standard active/inactive states)
                    let discard_text = RichText::new("Discard").size(14.0).color(theme.error);
                    if ui
                        .add(
                            egui::Button::new(discard_text)
                                .fill(theme.background)
                                .stroke(Stroke::new(1.0, theme.error))
                                .min_size(Vec2::new(110.0, 40.0))
                                .corner_radius(6.0),
                        )
                        .clicked()
                    {
                        action = Some(("discard", ComposeDraft::new()));
                    }
                    ui.add_space(8.0);
                });
            });
        }

        // Handle actions after draft borrow is released
        if let Some((action_type, draft)) = action {
            match action_type {
                "send" => {
                    handle_email_send(app, &draft);
                    app.compose_dialog_open = false;
                    app.compose_draft = None;
                }
                "discard" => {
                    app.compose_dialog_open = false;
                    app.compose_draft = None;
                }
                _ => {}
            }
        }
    });
}
