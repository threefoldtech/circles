use egui::{Color32, FontFamily, FontId, Margin, RichText, Stroke, Vec2};
use rfd::FileDialog; // Add this to Cargo.toml
use std::path::PathBuf;

use crate::{
    app::CircleApp,
    models::{
        dummy_data::Email,
        features::{ComposeDraft, ComposeDraftMode},
    },
    utils::config::Theme,
};

/// Create a new compose draft for replying to an email
pub fn create_reply_draft(email: &Email) -> ComposeDraft {
    let subject = if !email.subject.to_lowercase().starts_with("re:") {
        format!("Re: {}", email.subject)
    } else {
        email.subject.clone()
    };

    ComposeDraft {
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
    }
}

/// Create a new compose draft for forwarding an email
pub fn create_forward_draft(email: &Email) -> ComposeDraft {
    let subject = if !email.subject.to_lowercase().starts_with("fwd:") {
        format!("Fwd: {}", email.subject)
    } else {
        email.subject.clone()
    };

    ComposeDraft {
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
    }
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
pub struct Attachment {
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
}

impl ComposeDraft {
    /// Add a new attachment
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

    /// Remove an attachment by index
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

        // Attachments container
        let attachment_frame = egui::Frame::none()
            .fill(theme.hover.gamma_multiply(0.5))
            .inner_margin(Margin::same(12))
            .corner_radius(8.0);

        attachment_frame.show(ui, |ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new("Attachments").size(14.0).strong());
                ui.add_space(8.0);

                // Store indices of attachments to remove
                let mut to_remove = None;

                // Iterate over attachments with indices
                for (idx, attachment) in draft.attachments.iter().enumerate() {
                    ui.horizontal(|ui| {
                        // File icon and name
                        ui.label(RichText::new("📎").size(16.0));
                        ui.add_space(4.0);
                        ui.label(RichText::new(&attachment.name).size(14.0));

                        // File size
                        ui.add_space(8.0);
                        ui.label(
                            RichText::new(format_file_size(attachment.size))
                                .size(12.0)
                                .color(theme.secondary_text),
                        );

                        // Remove button
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

                // Remove attachment if needed
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

/// Render the compose screen
pub fn render_compose_screen(ui: &mut egui::Ui, app: &mut CircleApp, theme: &Theme) {
    // Get the title first, using an immutable borrow
    let title = if let Some(draft) = &app.compose_draft {
        match draft.mode {
            ComposeDraftMode::Reply => "Reply to email",
            ComposeDraftMode::Forward => "Forward email",
            ComposeDraftMode::New => "Draft a new email",
        }
    } else {
        "Draft a new email"
    };

    ui.vertical(|ui| {
        // Header
        ui.horizontal(|ui| {
            ui.heading(
                RichText::new(title)
                    .size(20.0)
                    .color(theme.header_text)
                    .strong(),
            );
        });
        ui.add_space(20.0);

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
            ui.style_mut().visuals.widgets.inactive.bg_fill = Color32::TRANSPARENT;
            ui.style_mut().visuals.widgets.active.bg_fill = Color32::TRANSPARENT;
            ui.style_mut().visuals.widgets.hovered.bg_fill = Color32::TRANSPARENT;

            // To field
            ui.horizontal(|ui| {
                ui.label(RichText::new("To:").size(14.0).color(theme.text).strong());
                ui.add_space(8.0);
                input_frame.show(ui, |ui| {
                    ui.set_max_width(500.0); // Adjust to fit dialog width
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("👤")
                                .size(16.0)
                                .color(Color32::from_rgb(70, 80, 90)),
                        );
                        ui.add_space(8.0);
                        ui.add(
                            egui::TextEdit::singleline(&mut draft.to)
                                .hint_text(
                                    RichText::new("Enter recipient...")
                                        .color(Color32::from_rgb(120, 130, 140)),
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
            ui.add_space(12.0);

            // Subject field
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("Subject:")
                        .size(14.0)
                        .color(theme.text)
                        .strong(),
                );
                ui.add_space(8.0);
                input_frame.show(ui, |ui| {
                    ui.set_max_width(500.0);
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("📜")
                                .size(16.0)
                                .color(Color32::from_rgb(70, 80, 90)),
                        );
                        ui.add_space(8.0);
                        ui.add(
                            egui::TextEdit::singleline(&mut draft.subject)
                                .hint_text(
                                    RichText::new("Enter subject...")
                                        .color(Color32::from_rgb(120, 130, 140)),
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
            ui.add_space(12.0);

            // Message body
            ui.horizontal(|ui| {
                ui.label(RichText::new("Body:").size(14.0).color(theme.text).strong());
                ui.add_space(8.0);
                input_frame.show(ui, |ui| {
                    ui.set_max_width(500.0);
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("📝")
                                .size(16.0)
                                .color(Color32::from_rgb(70, 80, 90)),
                        );
                        ui.add_space(8.0);
                        ui.add(
                            egui::TextEdit::multiline(&mut draft.body)
                                .hint_text(
                                    RichText::new("Enter message...")
                                        .color(Color32::from_rgb(120, 130, 140)),
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
            ui.add_space(20.0);

            // Restore original style after input fields
            ui.set_style(original_style);

            // Render attachments section before bottom buttons
            render_attachments(ui, draft, theme);

            // Bottom buttons
            let mut should_close = false;
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    if ui
                        .add(
                            egui::Button::new(
                                RichText::new("📎 Attach").size(14.0).color(theme.accent),
                            )
                            .fill(theme.background)
                            .stroke(Stroke::new(1.0, theme.accent))
                            .min_size(Vec2::new(100.0, 36.0))
                            .corner_radius(6.0),
                        )
                        .clicked()
                    {
                        handle_attachment_click(draft);
                    }

                    // Show attachment count if any
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
                    if ui
                        .add(
                            egui::Button::new(
                                RichText::new("Send").size(14.0).color(Color32::WHITE),
                            )
                            .fill(theme.accent)
                            .min_size(Vec2::new(100.0, 36.0))
                            .corner_radius(6.0),
                        )
                        .clicked()
                    {
                        should_close = true;
                    }

                    if ui
                        .add(
                            egui::Button::new(
                                RichText::new("Discard")
                                    .size(14.0)
                                    .color(Color32::from_rgb(200, 50, 50)),
                            )
                            .fill(theme.background)
                            .stroke(Stroke::new(1.0, Color32::from_rgb(200, 50, 50)))
                            .min_size(Vec2::new(100.0, 36.0))
                            .corner_radius(6.0),
                        )
                        .clicked()
                    {
                        should_close = true;
                    }
                    ui.add_space(8.0);
                });
            });

            if should_close {
                app.compose_dialog_open = false;
                app.compose_draft = None;
            }
        }
    });
}
