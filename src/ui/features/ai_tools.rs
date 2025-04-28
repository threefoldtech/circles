use egui::{RichText, Stroke, Ui, Vec2};

use crate::{
    app::{ActiveFeature, CircleApp},
    ui::app_layout::create_content_frame,
};

// Define AI tool types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AIToolType {
    Chat,
    ImageGeneration,
    TextSummarization,
    CodeAssistant,
    VoiceTranscription,
    DataAnalysis,
}

impl AIToolType {
    fn get_name(&self) -> &'static str {
        match self {
            AIToolType::Chat => "Chat AI",
            AIToolType::ImageGeneration => "Image Generator",
            AIToolType::TextSummarization => "Text Summarizer",
            AIToolType::CodeAssistant => "Code Assistant",
            AIToolType::VoiceTranscription => "Voice Transcription",
            AIToolType::DataAnalysis => "Data Analyzer",
        }
    }

    fn get_icon(&self) -> &'static str {
        match self {
            AIToolType::Chat => "💬",
            AIToolType::ImageGeneration => "🖼️",
            AIToolType::TextSummarization => "📝",
            AIToolType::CodeAssistant => "💻",
            AIToolType::VoiceTranscription => "🎤",
            AIToolType::DataAnalysis => "📊",
        }
    }

    fn get_description(&self) -> &'static str {
        match self {
            AIToolType::Chat => "Have a conversation with an AI assistant",
            AIToolType::ImageGeneration => "Generate images from text descriptions",
            AIToolType::TextSummarization => "Summarize long texts automatically",
            AIToolType::CodeAssistant => "Get help with coding tasks",
            AIToolType::VoiceTranscription => "Convert speech to text",
            AIToolType::DataAnalysis => "Analyze and visualize your data",
        }
    }
}

pub fn render_ai_tools(app: &mut CircleApp, ui: &mut Ui) {
    ui.add_space(16.0);
    let theme = app.get_current_theme();

    create_content_frame(&theme).show(ui, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading(RichText::new("AI Tools").size(24.0).color(theme.text));
            ui.add_space(8.0);
            ui.label(
                RichText::new("Select an AI tool to get started")
                    .size(16.0)
                    .color(theme.secondary_text),
            );
            ui.add_space(16.0);
        });

        // Define the AI tools to display
        let tools = [
            AIToolType::Chat,
            AIToolType::ImageGeneration,
            AIToolType::TextSummarization,
            AIToolType::CodeAssistant,
            AIToolType::VoiceTranscription,
            AIToolType::DataAnalysis,
        ];

        // Calculate grid layout
        let available_width = ui.available_width();
        let card_width = 200.0;
        let _card_height = 180.0;
        let card_spacing = 16.0;
        let cards_per_row = (available_width / (card_width + card_spacing))
            .floor()
            .max(1.0) as usize;

        // Create grid layout

        // Use a grid layout to ensure all tools are visible
        egui::Grid::new("ai_tools_grid")
            .spacing([card_spacing, card_spacing + 30.0]) // Add extra vertical spacing (30px)
            .min_col_width(card_width)
            .max_col_width(card_width)
            .show(ui, |ui| {
                let mut current_col = 0;

                for (i, tool) in tools.iter().enumerate() {
                    // Render the card with a unique ID
                    let card_response = render_ai_tool_card(
                        ui,
                        *tool,
                        &theme,
                        format!("ai_tool_{}", tool.get_name()),
                    );

                    // Handle card click
                    if card_response.clicked() {
                        match tool {
                            AIToolType::Chat => {
                                app.set_active_feature(ActiveFeature::Chat);
                            }
                            _ => {
                                // Other tools not implemented yet
                            }
                        }
                    }

                    current_col += 1;

                    // End the row after reaching the maximum columns per row
                    if current_col >= cards_per_row && i < tools.len() - 1 {
                        ui.end_row();
                        current_col = 0;
                    }
                }
            });
    });
}

fn render_ai_tool_card(
    ui: &mut Ui,
    tool: AIToolType,
    theme: &crate::utils::config::Theme,
    id: String,
) -> egui::Response {
    // Create a frame for the card
    let frame = egui::Frame::new()
        .fill(theme.panel)
        .stroke(Stroke::new(1.0, theme.border))
        .corner_radius(8)
        .inner_margin(egui::Margin::same(16));

    // Use a button inside the frame for click detection
    let response = frame
        .show(ui, |ui| {
            // Set a fixed size for the card
            ui.set_min_size(Vec2::new(200.0, 180.0));

            // Create a vertical layout for the card content
            ui.vertical_centered(|ui| {
                // Icon
                ui.label(RichText::new(tool.get_icon()).size(48.0));
                ui.add_space(8.0);

                // Title
                ui.label(
                    RichText::new(tool.get_name())
                        .size(18.0)
                        .color(theme.text)
                        .strong(),
                );
                ui.add_space(8.0);

                // Description
                ui.label(
                    RichText::new(tool.get_description())
                        .size(14.0)
                        .color(theme.secondary_text)
                        .text_style(egui::TextStyle::Body),
                );
            });

            // Make the entire card clickable with a unique ID
            ui.interact(ui.min_rect(), ui.id().with(id), egui::Sense::click())
        })
        .inner;

    response
}
