use eframe::egui::{self, Color32, RichText, Vec2};
use uuid::Uuid;

use crate::models::features::video_conf::VideoConferenceState;
use crate::utils::config::Theme;

use super::utils::calculate_grid_dimensions;
use super::video_tile::VideoTile;

/// Video grid component for the video conference meeting
pub struct VideoGrid;

impl VideoGrid {
    /// Render the video grid with participant tiles
    pub fn render(
        ui: &mut egui::Ui,
        state: &mut VideoConferenceState,
        theme: &Theme,
        user_id: Uuid,
        available_height: f32,
    ) {
        if let Some(meeting) = state.current_meeting.as_ref() {
            // Use all available space with a black background
            ui.vertical(|ui| {
                // Display participants in a grid
                let participants = &meeting.participants;
                if participants.is_empty() {
                    ui.vertical_centered(|ui| {
                        ui.add_space(available_height / 3.0);
                        ui.label(
                            RichText::new("No participants in the meeting")
                                .size(18.0)
                                .color(Color32::WHITE),
                        );
                    });
                } else {
                    // Calculate grid dimensions based on participant count
                    let participant_count = participants.len();
                    let (cols, rows) = calculate_grid_dimensions(participant_count);

                    // Calculate video tile size with minimal padding
                    let padding = 4.0; // Reduced padding for more space (Zoom-like)
                    let local_width = ui.available_width();
                    let local_height = available_height;

                    // Calculate aspect ratio to maintain 16:9 video tiles if possible
                    let aspect_ratio = 16.0 / 9.0;

                    // Calculate tile dimensions based on available space
                    let tile_width_by_cols = (local_width / cols as f32) - padding;
                    let tile_height_by_rows = (local_height / rows as f32) - padding;

                    // Determine if we should constrain by width or height
                    let (tile_width, tile_height) =
                        if tile_width_by_cols / aspect_ratio <= tile_height_by_rows {
                            // Constrained by width
                            (tile_width_by_cols, tile_width_by_cols / aspect_ratio)
                        } else {
                            // Constrained by height
                            (tile_height_by_rows * aspect_ratio, tile_height_by_rows)
                        };

                    let tile_size = Vec2::new(tile_width, tile_height);

                    // Calculate total grid size
                    let grid_width = cols as f32 * (tile_width + padding) - padding;
                    let grid_height = rows as f32 * (tile_height + padding) - padding;

                    // Center the grid in the available space
                    let start_x = (local_width - grid_width) / 2.0;
                    let start_y = (local_height - grid_height) / 2.0;

                    // Render video grid
                    let mut row = 0;
                    let mut col = 0;

                    // Special case for gallery view with 1 participant (make it larger and centered)
                    if participant_count == 1 {
                        let single_tile_width = local_width * 0.6; // 60% of width
                        let single_tile_height = single_tile_width / aspect_ratio;

                        // Center the single tile
                        let center_x = (local_width - single_tile_width) / 2.0;
                        let center_y = (local_height - single_tile_height) / 2.0;

                        let rect = egui::Rect::from_min_size(
                            egui::Pos2::new(
                                ui.min_rect().min.x + center_x,
                                ui.min_rect().min.y + center_y,
                            ),
                            Vec2::new(single_tile_width, single_tile_height),
                        );

                        // Draw the single video tile
                        VideoTile::render(
                            ui,
                            rect,
                            &participants[0],
                            theme,
                            participants[0].id == user_id,
                        );

                        // Show a message encouraging inviting others - properly centered
                        // Position at the bottom of the video area
                        let message_height = 30.0;
                        let message_width = single_tile_width; // Match the width of the video tile

                        let message_rect = egui::Rect::from_center_size(
                            egui::pos2(
                                ui.min_rect().min.x + local_width / 2.0,
                                ui.min_rect().min.y + center_y + single_tile_height + 20.0,
                            ),
                            egui::vec2(message_width, message_height),
                        );

                        // Create a semi-transparent background for the message
                        let painter = ui.painter();
                        painter.rect_filled(
                            message_rect,
                            4.0,
                            Color32::from_rgba_premultiplied(0, 0, 0, 150),
                        );

                        ui.allocate_ui_at_rect(message_rect, |ui| {
                            ui.with_layout(
                                egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                                |ui| {
                                    ui.label(
                                        RichText::new(
                                            "You're the only one here. Invite others to join!",
                                        )
                                        .size(16.0)
                                        .color(Color32::WHITE)
                                        .strong(),
                                    );
                                },
                            );
                        });
                    } else {
                        // Regular grid layout for multiple participants
                        for participant in participants {
                            if col >= cols {
                                col = 0;
                                row += 1;
                            }

                            let rect = egui::Rect::from_min_size(
                                egui::Pos2::new(
                                    ui.min_rect().min.x
                                        + start_x
                                        + col as f32 * (tile_width + padding),
                                    ui.min_rect().min.y
                                        + start_y
                                        + row as f32 * (tile_height + padding),
                                ),
                                tile_size,
                            );

                            // Draw video tile
                            VideoTile::render(
                                ui,
                                rect,
                                participant,
                                theme,
                                participant.id == user_id,
                            );

                            col += 1;
                        }
                    }
                }
            });
        }
    }
}
