use eframe::egui;

use crate::ui::app_layout::{create_action_button, create_content_frame, render_header};

pub fn render_mail(ui: &mut egui::Ui) {
    render_header(ui, "📧", "Mail");
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        if ui.add(create_action_button("Compose", "✏️")).clicked() {
            // TODO: Implement new message
        }
        ui.add_space(8.0);
        if ui.add(create_action_button("Refresh", "🔄")).clicked() {
            // TODO: Implement refresh
        }
        ui.add_space(8.0);
    });
    ui.add_space(16.0);
    create_content_frame().show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.set_width(200.0);
                ui.strong("Folders");
                ui.separator();
                for folder in ["📥 Inbox (3)", "📤 Sent", "📝 Drafts", "🗑️ Trash"] {
                    ui.selectable_value(&mut (), (), folder);
                }
            });
            ui.separator();
            ui.vertical(|ui| {
                ui.strong("Messages");
                ui.separator();
                for i in 1..=3 {
                    egui::Frame::none()
                        .fill(egui::Color32::from_rgb(248, 249, 250))
                        .rounding(egui::Rounding::same(4.0))
                        .inner_margin(egui::Margin::same(8.0))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(format!("👤 Sender {}", i));
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        ui.label(format!("{}h ago", i));
                                    },
                                );
                            });
                            ui.label(format!("Sample email subject {}", i));
                            ui.label(egui::RichText::new("This is a preview...").weak());
                        });
                    ui.add_space(4.0);
                }
            });
        });
    });
}
