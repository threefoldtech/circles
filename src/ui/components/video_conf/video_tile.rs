use eframe::egui::{self, Align2, Color32, Stroke};
use egui::StrokeKind;
use egui_phosphor::regular::{HAND_PALM, MICROPHONE_SLASH, SPEAKER_HIGH};

use crate::models::features::video_conf::Participant;
use crate::utils::config::Theme;

/// Video tile component for a single participant
pub struct VideoTile;

impl VideoTile {
    /// Render a single video tile for a participant
    pub fn render(
        ui: &mut egui::Ui,
        rect: egui::Rect,
        participant: &Participant,
        theme: &Theme,
        is_self: bool,
    ) {
        let painter = ui.painter();

        // Zoom-like styling
        let bg_color = Color32::from_rgb(0, 0, 0); // Black background

        let border_color = if participant.is_active_speaker {
            Color32::from_rgb(255, 255, 0) // Yellow for active speaker
        } else if is_self {
            Color32::from_rgb(0, 120, 215) // Blue for self
        } else {
            Color32::from_rgb(40, 40, 40) // Dark gray for others
        };

        // Draw the video tile background with subtle rounded corners (Zoom-like)
        painter.rect_filled(rect, 4.0, bg_color);

        // Only show border for active speaker or self
        if participant.is_active_speaker || is_self {
            let stroke = Stroke::new(2.0, border_color);
            painter.rect_stroke(rect, 4.0, stroke, StrokeKind::Inside);
        }

        if !participant.camera_on {
            // Camera is off - show avatar with initials (Zoom style)
            let initial = participant
                .name
                .chars()
                .next()
                .unwrap_or('?')
                .to_uppercase()
                .next()
                .unwrap_or('?');

            // Make avatar circle
            let avatar_radius = rect.height() / 6.0;
            let avatar_pos = egui::pos2(rect.center().x, rect.center().y);

            // Draw avatar circle (Zoom uses gray)
            painter.circle_filled(avatar_pos, avatar_radius, Color32::from_rgb(80, 80, 80));

            // Draw initial
            painter.text(
                avatar_pos,
                Align2::CENTER_CENTER,
                initial.to_string(),
                egui::FontId::proportional(avatar_radius * 1.2),
                Color32::WHITE,
            );
        } else {
            // Camera is on - simulate video feed with a Zoom-like placeholder
            let video_rect = rect.shrink(2.0);

            // Draw video background (dark gray like Zoom)
            painter.rect_filled(video_rect, 0.0, Color32::from_rgb(30, 30, 30));

            // Draw a simple avatar silhouette
            let head_radius = video_rect.height() / 8.0;
            let head_pos = egui::pos2(
                video_rect.center().x,
                video_rect.min.y + video_rect.height() * 0.4,
            );

            // Draw head
            painter.circle_filled(head_pos, head_radius, Color32::from_rgb(100, 100, 100));

            // Draw shoulders and body (simplified)
            let body_width = head_radius * 3.0;
            let body_height = head_radius * 4.0;
            let body_rect = egui::Rect::from_center_size(
                egui::pos2(head_pos.x, head_pos.y + body_height / 2.0),
                egui::vec2(body_width, body_height),
            );

            painter.rect_filled(body_rect, 0.0, Color32::from_rgb(70, 70, 70));
        }

        // Draw participant name and status indicators at the bottom of the tile (Zoom style)
        let bottom_height = 22.0; // Zoom uses a smaller bar
        let bottom_rect =
            egui::Rect::from_min_max(egui::pos2(rect.min.x, rect.max.y - bottom_height), rect.max);

        // Semi-transparent black bar at bottom (Zoom style)
        painter.rect_filled(
            bottom_rect,
            0.0,
            Color32::from_rgba_premultiplied(0, 0, 0, 180),
        );

        // Draw name with Zoom styling
        let name_text = if is_self {
            format!("{} (You)", participant.name)
        } else {
            participant.name.clone()
        };

        painter.text(
            egui::pos2(bottom_rect.min.x + 6.0, bottom_rect.center().y),
            Align2::LEFT_CENTER,
            name_text,
            egui::FontId::proportional(12.0),
            Color32::WHITE,
        );

        // Draw status indicators with icons (Zoom style)
        let mut indicator_x = bottom_rect.max.x - 6.0;
        let indicator_y = bottom_rect.center().y;

        // Mic status (red icon when muted)
        if !participant.mic_on {
            indicator_x -= 16.0;
            painter.text(
                egui::pos2(indicator_x, indicator_y),
                Align2::RIGHT_CENTER,
                MICROPHONE_SLASH,
                egui::FontId::proportional(14.0),
                Color32::from_rgb(255, 80, 80),
            );
        }

        // Hand raised (yellow icon)
        if participant.hand_raised {
            indicator_x -= 16.0;
            painter.text(
                egui::pos2(indicator_x, indicator_y),
                Align2::RIGHT_CENTER,
                HAND_PALM,
                egui::FontId::proportional(14.0),
                Color32::from_rgb(255, 220, 0),
            );
        }

        // Active speaker (green icon)
        if participant.is_active_speaker {
            indicator_x -= 16.0;
            painter.text(
                egui::pos2(indicator_x, indicator_y),
                Align2::RIGHT_CENTER,
                SPEAKER_HIGH,
                egui::FontId::proportional(14.0),
                Color32::from_rgb(0, 230, 0),
            );
        }
    }
}
