/// Utility functions for video conference components

/// Calculate the number of columns and rows for the video grid
pub fn calculate_grid_dimensions(count: usize) -> (usize, usize) {
    match count {
        0 => (1, 1),
        1 => (1, 1),
        2 => (2, 1),
        3..=4 => (2, 2),
        5..=6 => (3, 2),
        7..=9 => (3, 3),
        10..=12 => (4, 3),
        13..=16 => (4, 4),
        _ => {
            let cols = (count as f64).sqrt().ceil() as usize;
            let rows = (count + cols - 1) / cols;
            (cols, rows)
        }
    }
}

/// Format a timestamp for display
pub fn format_timestamp(timestamp: chrono::DateTime<chrono::Utc>) -> String {
    timestamp.format("%H:%M").to_string()
}
