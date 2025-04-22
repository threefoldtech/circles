use egui::Color32;

use crate::app::ActiveFeature;

pub const NAV_ITEMS: &[(&str, &str, ActiveFeature)] = &[
    ("📧", "Mail", ActiveFeature::Mail),
    ("📅", "Calendar", ActiveFeature::Calendar),
    ("💬", "Chat", ActiveFeature::Chat),
    ("📄", "Documents", ActiveFeature::Documents),
    ("🤖", "AI Tools", ActiveFeature::AITools),
    ("📹", "Video", ActiveFeature::VideoConference),
    ("⚙️", "Settings", ActiveFeature::Settings),
];

pub struct LayoutConfig {
    pub spacing: f32,
    pub sidebar_width: f32,
    pub navbar_height: f32,
    // pub button_size: Vec2,
}

impl LayoutConfig {
    pub fn new() -> Self {
        Self {
            spacing: 12.0,
            sidebar_width: 300.0,
            navbar_height: 95.0,
            // button_size: Vec2::new(90.0, 40.0),
        }
    }
}

#[allow(dead_code)]
pub struct Theme {
    pub accent: Color32,
    pub background: Color32,
    pub panel: Color32,
    pub text: Color32,
    pub header_text: Color32,
    pub secondary_text: Color32,
    pub secondary_background: Color32,
    pub border: Color32,
    pub active: Color32,
    pub hover: Color32,
    pub shadow: Color32,
    pub success: Color32,
    pub team: Color32,
    pub private: Color32,
    pub error: Color32,
}

impl Theme {
    pub fn new() -> Self {
        Self {
            accent: Color32::from_rgb(59, 130, 246),      // Royal blue
            background: Color32::from_rgb(248, 250, 252), // Warm cream
            panel: Color32::from_rgb(255, 255, 255),      // White
            text: Color32::from_rgb(30, 41, 59),          // Dark slate
            header_text: Color32::from_rgb(71, 85, 105),  // Slate blue
            secondary_text: Color32::from_rgb(100, 116, 139), // Soft gray
            secondary_background: Color32::from_rgb(226, 232, 240), // Light blue-gray
            border: Color32::from_rgb(203, 213, 225),     // Subtle gray
            active: Color32::from_rgb(96, 165, 250),      // Bright blue
            hover: Color32::from_rgb(239, 246, 255),      // Pale blue
            shadow: Color32::from_black_alpha(15),        // Soft shadow
            success: Color32::from_rgb(52, 211, 153),     // Muted green
            team: Color32::from_rgb(2, 132, 199),         // Vibrant blue
            private: Color32::from_rgb(168, 85, 247),     // Soft purple
            error: Color32::from_rgb(239, 68, 68),        // Soft red
        }
    }
}
